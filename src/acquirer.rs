use std::error::Error;
use std::sync::Arc;

use headless_chrome::{Browser, LaunchOptions, Tab};

use crate::args::Args;

pub struct Acquirer {
    pub browser: Browser,
    pub tab: Arc<Tab>,
}

impl Acquirer {
    pub fn launch(args: &Args) -> Result<Acquirer, Box<dyn Error>> {
        let browser = Browser::new(LaunchOptions {
            headless: args.headless,
            ..Default::default()
        })?;

        let tab = browser.wait_for_initial_tab()?;

        Ok(Acquirer { browser, tab })
    }
}
