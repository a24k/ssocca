pub mod config;

use anyhow::Context as _;
use async_std::{future, task, task::JoinHandle};
use futures::StreamExt;
use log::{debug, info, trace};
use std::time::Duration;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;

pub struct Acquirer {
    browser: Browser,
    handle: JoinHandle<()>,
    page: Page,
}

impl Acquirer {
    pub async fn launch(config: BrowserConfig) -> anyhow::Result<Acquirer> {
        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("Failed to launch chrome browser")?;

        let handle = task::spawn(async move { while (handler.next().await).is_some() {} });

        let page =
            Self::wait_for_initial_page_with_timeout(&browser, Duration::from_secs(10)).await?;
        page.wait_for_navigation().await?;

        debug!("{:?}", browser);
        debug!("{:?}", browser.version().await?);

        Ok(Acquirer {
            browser,
            handle,
            page,
        })
    }

    async fn wait_for_initial_page_with_timeout(
        browser: &Browser,
        timeout: Duration,
    ) -> anyhow::Result<Page> {
        async fn wait_for_initial_page(browser: &Browser) -> anyhow::Result<Page> {
            let wait = Duration::from_millis(100);

            loop {
                trace!("Waiting for the initial page");
                task::sleep(wait).await;

                let mut pages = browser.pages().await.context("Retrieve list of pages")?;
                match pages.pop() {
                    Some(page) => return Ok(page),
                    None => continue,
                }
            }
        }

        let page = future::timeout(timeout, wait_for_initial_page(browser)).await;

        match page {
            Ok(page) => page,
            _ => {
                debug!("Found no page. Create new one.");
                browser
                    .new_page("chrome://about/")
                    .await
                    .context("Failed to create new page")
            }
        }
    }

    pub async fn navigate_with_timeout(&self, url: &str, timeout: Duration) -> anyhow::Result<()> {
        future::timeout(timeout, self.navigate(url))
            .await
            .with_context(|| format!("Timeout to navigate url = {}", url))?
    }

    pub async fn navigate(&self, url: &str) -> anyhow::Result<()> {
        self.page.goto(url).await?;

        self.page
            .wait_for_navigation()
            .await
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        Ok(())
    }

    pub async fn dump(&self) -> anyhow::Result<()> {
        let cookies = self
            .page
            .get_cookies()
            .await
            .context("Failed to get cookies")?;
        info!("{cookies:?}");

        Ok(())
    }

    pub async fn close(mut self) -> anyhow::Result<()> {
        self.browser.close().await?;

        self.handle.await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{config, Acquirer};
    use crate::args::{Args, Parser as _};

    #[rstest]
    #[case("https://example.com/")]
    #[should_panic(expected = "Timeout to navigate url")]
    #[case("nowhere")]
    async fn navigate(#[case] input: &str) {
        let args = Args::parse_from(vec!["ssocca", "--headless", input]);
        let acquirer = Acquirer::launch(config::build(&args).unwrap())
            .await
            .unwrap();
        acquirer
            .navigate_with_timeout(&args.url, super::Duration::from_secs(5))
            .await
            .unwrap();
        acquirer.close().await.unwrap();
    }

    #[rstest]
    #[case("https://example.com/")]
    async fn incognito(#[case] input: &str) {
        let args = Args::parse_from(vec!["ssocca", "--headless", input]);
        let acquirer = Acquirer::launch(config::build(&args).unwrap())
            .await
            .unwrap();
        assert_eq!(true, acquirer.browser.is_incognito());
    }
}
