mod config;

use anyhow::Context as _;
use async_std::{task, task::JoinHandle};
use futures::StreamExt;

use chromiumoxide::browser::Browser;
use chromiumoxide::page::Page;

pub struct Acquirer {
    pub browser: Browser,
    pub handle: JoinHandle<()>,
}

impl Acquirer {
    pub async fn launch(headless: bool) -> anyhow::Result<Acquirer> {
        let config = config::build(headless)?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("Failed to launch chrome browser")?;

        let handle = task::spawn(async move { while (handler.next().await).is_some() {} });

        Ok(Acquirer { browser, handle })
    }

    pub async fn navigate(&self, url: &str) -> anyhow::Result<Page> {
        let page = self
            .browser
            .new_page(url)
            .await
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        page.wait_for_navigation()
            .await
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        Ok(page)
    }

    pub async fn dump(&self, page: &Page) -> anyhow::Result<()> {
        let cookies = page.get_cookies().await.context("Failed to get cookies")?;

        cookies.iter().for_each(|cookie| {
            println!(
                "{} = {}\n  ; Domain = {}",
                cookie.name, cookie.value, cookie.domain
            );
        });

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

    use super::Acquirer;

    #[rstest]
    #[case("https://www.google.com")]
    #[should_panic(expected = "Failed to navigate url")]
    #[case("nowhere")]
    async fn navigate(#[case] url: &str) {
        let acquirer = Acquirer::launch(true).await.unwrap();
        acquirer.navigate(url).await.unwrap();
        acquirer.close().await.unwrap();
    }
}
