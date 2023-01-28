pub mod config;

use anyhow::Context as _;
use async_std::{future, task, task::JoinHandle};
use futures::StreamExt;
use log::{debug, info, trace};
use std::time::Duration;

use chromiumoxide::browser::Browser;
use chromiumoxide::page::Page;

pub use config::AcquirerConfig;

pub struct Acquirer {
    browser: Browser,
    config: AcquirerConfig,
    handle: JoinHandle<()>,
    page: Page,
}

impl Acquirer {
    pub async fn launch(config: AcquirerConfig) -> anyhow::Result<Acquirer> {
        let (browser, mut handler) = Browser::launch(config.browser.clone())
            .await
            .context("Failed to launch chrome browser")?;

        let handle = task::spawn(async move { while (handler.next().await).is_some() {} });

        let page = Self::wait_for_initial_page(&browser, config.timeout).await?;
        page.wait_for_navigation().await?;

        debug!("{:?}", browser);
        debug!("{:?}", browser.version().await?);

        Ok(Acquirer {
            browser,
            config,
            handle,
            page,
        })
    }

    async fn wait_for_initial_page(browser: &Browser, timeout: Duration) -> anyhow::Result<Page> {
        async fn _wait_for_initial_page(browser: &Browser) -> anyhow::Result<Page> {
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

        let page = future::timeout(timeout, _wait_for_initial_page(browser)).await;

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

    pub async fn navigate(&self, url: &str) -> anyhow::Result<()> {
        async fn _navigate(page: &Page, url: &str) -> anyhow::Result<()> {
            page.goto(url).await?;

            page.wait_for_navigation()
                .await
                .with_context(|| format!("Failed to navigate url = {}", url))?;

            Ok(())
        }

        future::timeout(self.config.timeout, _navigate(&self.page, url))
            .await
            .with_context(|| format!("Timeout to navigate url = {}", url))?
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

    use crate::args;

    use super::{Acquirer, AcquirerConfig};

    #[rstest]
    #[case("https://example.com/")]
    async fn incognito(#[case] url: &str) {
        let args = args!["--headless", url];
        let acquirer = Acquirer::launch(AcquirerConfig::build(&args).unwrap())
            .await
            .unwrap();
        assert_eq!(true, acquirer.browser.is_incognito());
    }

    #[rstest]
    #[case("https://example.com/")]
    #[should_panic(expected = "Timeout to navigate url")]
    #[case("nowhere")]
    async fn navigate(#[case] url: &str) {
        let args = args!["--timeout", "5", "--headless", url];
        let acquirer = Acquirer::launch(AcquirerConfig::build(&args).unwrap())
            .await
            .unwrap();
        acquirer.navigate(&args.url).await.unwrap();
        acquirer.close().await.unwrap();
    }
}
