use std::sync::Arc;

use anyhow::Context as _;
use headless_chrome::{Browser, LaunchOptions, Tab};

pub struct Acquirer {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl Acquirer {
    pub fn launch(headless: bool) -> anyhow::Result<Acquirer> {
        let browser = Browser::new(LaunchOptions {
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
