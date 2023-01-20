use std::error::Error;
use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptions, Tab};

pub struct Acquirer {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl Acquirer {
    pub fn launch(headless: bool) -> Result<Acquirer, Box<dyn Error>> {
        let browser = Browser::new(LaunchOptions {
            headless,
            ..Default::default()
        })?;

        let tab = browser.wait_for_initial_tab()?;

        Ok(Acquirer { browser, tab })
    }

    pub fn navigate(&self, url: &str) -> Result<(), Box<dyn Error>> {
        self.tab.navigate_to(url)?;
        Ok(())
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        let cookies = self.tab.get_cookies()?;

        cookies.iter().for_each(|cookie| {
            println!("{}={}; Domain={}", cookie.name, cookie.value, cookie.domain);
        });

        Ok(())
    }
}
