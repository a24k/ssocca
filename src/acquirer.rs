use anyhow::{anyhow, Context as _};
use async_std::task;
use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::{viewport::Viewport, Handler};
use chromiumoxide::page::Page;

pub struct Acquirer {
    pub browser: Browser,
    pub handle: task::JoinHandle<Handler>,
}

fn build_browser_config(headless: bool) -> anyhow::Result<BrowserConfig> {
    let viewport = Viewport {
        width: 0,
        height: 0,
        ..Default::default()
    };

    let builder = BrowserConfig::builder().viewport(viewport);
    let builder = match headless {
        true => builder,
        false => builder.with_head(),
    };

    builder.build().map_err(|e| anyhow!(e))
}

impl Acquirer {
    pub async fn launch(headless: bool) -> anyhow::Result<Acquirer> {
        let config =
            build_browser_config(headless).context("Failed to configure chrome browser")?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .context("Failed to launch chrome browser")?;

        let handle = task::spawn(async move {
            loop {
                let _ = handler.next().await.unwrap();
            }
        });

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
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::Acquirer;

    #[rstest]
    #[case("https://github.com")]
    #[should_panic(expected = "Failed to navigate url")]
    #[case("nowhere")]
    async fn navigate(#[case] url: &str) {
        let acquirer = Acquirer::launch(true).await.unwrap();
        acquirer.navigate(url).await.unwrap();
    }
}
