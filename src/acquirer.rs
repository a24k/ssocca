pub mod config;

use anyhow::Context as _;
use async_std::{future,task, task::JoinHandle};
use futures::StreamExt;
use log::{debug, info};

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

        async fn wait_for_initial_page(browser: &Browser) -> anyhow::Result<Page> {
            loop {
                debug!("loop!");
                task::sleep(std::time::Duration::from_millis(1000)).await;
                let mut pages = browser.pages().await?;
                match pages.pop() {
                    Some(page) => return Ok(page),
                    None => continue,
                }
            }
        }

        let page = future::timeout(std::time::Duration::from_secs(30), wait_for_initial_page(&browser)).await??;
        page.wait_for_navigation().await?;

        debug!("{:?}", browser.version().await?);

        Ok(Acquirer {
            browser,
            handle,
            page,
        })
    }

    pub async fn navigate(&self, url: &str) -> anyhow::Result<()> {
        // temporary
        self.dump().await?;

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
    #[should_panic(expected = "Failed to navigate url")]
    #[case("nowhere")]
    async fn navigate(#[case] input: &str) {
        let args = Args::parse_from(vec!["ssocca", "--headless", input]);
        let acquirer = Acquirer::launch(config::build(&args).unwrap())
            .await
            .unwrap();
        acquirer.navigate(&args.url).await.unwrap();
        acquirer.close().await.unwrap();
    }
}
