use std::sync::Arc;
use headless_chrome::{Browser, LaunchOptions, Tab};

pub struct Acquirer {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl Acquirer {
    pub fn new(args: super::Args) -> Result<Acquirer, Box<dyn std::error::Error>> {
        let browser = Browser::new(LaunchOptions {
            headless: args.headless,
            ..Default::default()
        })?;

        let tab = browser.wait_for_initial_tab()?;

        Ok(Acquirer { browser, tab })
    }
}
