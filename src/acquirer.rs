use std::sync::Arc;

use anyhow::{anyhow, Context as _};
use headless_chrome::{Browser as HChrome, LaunchOptions, Tab};

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::Handler;

pub struct AcquirerOxide {
    pub browser: Browser,
    pub handler: Handler,
}

impl AcquirerOxide {
    pub async fn launch_oxide() -> anyhow::Result<AcquirerOxide> {
        let config = BrowserConfig::builder()
            .with_head()
            .build()
            .map_err(|e| anyhow!(e))?;

        let (browser, handler) = Browser::launch(config).await?;

        Ok(AcquirerOxide { browser, handler })
    }
}

pub struct Acquirer {
    pub browser: HChrome,
    pub tab: Arc<Tab>,
}

impl Acquirer {
    pub fn launch(headless: bool) -> anyhow::Result<Acquirer> {
        let browser = HChrome::new(LaunchOptions {
            headless,
            ..Default::default()
        })
        .context("Failed to launch chrome browser")?;

        let tab = browser
            .wait_for_initial_tab()
            .context("Failed to initialize tab")?;

        Ok(Acquirer { browser, tab })
    }

    pub fn navigate(&self, url: &str) -> anyhow::Result<()> {
        self.tab
            .navigate_to(url)
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        self.tab
            .wait_until_navigated()
            .with_context(|| format!("Failed to navigate url = {}", url))?;

        Ok(())
    }

    pub fn dump(&self) -> anyhow::Result<()> {
        let cookies = self.tab.get_cookies().context("Failed to get cookies")?;

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
    #[should_panic(expected = "Failed to navigate url")]
    #[case("https://nowhere.local")]
    fn navigate(#[case] url: &str) {
        let acquirer = Acquirer::launch(true).unwrap();
        acquirer.navigate(url).unwrap();
    }
}
