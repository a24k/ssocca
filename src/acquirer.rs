pub mod config;

use anyhow::Context as _;
use async_std::{task, task::JoinHandle};
use futures::StreamExt;
use log::{debug, info, warn};

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::Page;

pub struct Acquirer {
    browser: Browser,
    handle: JoinHandle<()>,
}

impl Acquirer {
    pub async fn launch(config: BrowserConfig) -> anyhow::Result<Acquirer> {
        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("Failed to launch chrome browser")?;

        // temporary
        let context = handler.default_browser_context();
        warn!("{:?}", context);

        let handle = task::spawn(async move { while (handler.next().await).is_some() {} });

        // temporary
        warn!("incognito = {:?}", browser.is_incognito());

        Ok(Acquirer { browser, handle })
    }

    pub async fn navigate(&self, url: &str) -> anyhow::Result<Page> {
        // temporary
        let pages = self.browser.pages().await?;
        warn!("{pages:?}");

        let page = self
            .browser
            .new_page(url)
            .await
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        // temporary
        self.dump(&page).await?;

        page.wait_for_navigation()
            .await
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        Ok(page)
    }

    pub async fn dump(&self, page: &Page) -> anyhow::Result<()> {
        debug!("{:?}", self.browser.version().await?);

        let cookies = page.get_cookies().await.context("Failed to get cookies")?;
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
