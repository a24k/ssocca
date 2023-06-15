pub mod config;
pub mod scenario;

use anyhow::{anyhow, Context as _};
use async_std::{future, task, task::JoinHandle};
use boringauth::oath::{HashFunction, TOTPBuilder};
use futures::StreamExt;
use log::{debug, info, trace, warn};
use regex::Regex;
use std::time::Duration;

use chromiumoxide::browser::Browser;
use chromiumoxide::cdp::browser_protocol::network::Cookie;
use chromiumoxide::cdp::browser_protocol::page::NavigateParams;
use chromiumoxide::page::Page;

pub use config::AcquirerConfig;
pub use scenario::{
    rule::{Click, Input, Rule, Totp, UrlPattern},
    Scenario,
};

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
                warn!("Found no page. Create new one.");
                browser
                    .new_page("chrome://about/")
                    .await
                    .context("Failed to create new page")
            }
        }
    }

    pub async fn navigate(&self, to: &NavigateParams) -> anyhow::Result<()> {
        async fn _navigate(page: &Page, to: &NavigateParams) -> anyhow::Result<()> {
            page.goto(to.clone()).await?;

            page.wait_for_navigation()
                .await
                .with_context(|| format!("Failed to navigate url = {:?}", to))?;

            Ok(())
        }

        future::timeout(self.config.timeout, _navigate(&self.page, to))
            .await
            .with_context(|| format!("Timeout to navigate url = {to:?}"))?
    }

    async fn is_on(&self, on: Option<&UrlPattern>) -> anyhow::Result<()> {
        match on {
            Some(on) => match self.page.url().await {
                Ok(Some(url)) => {
                    let re = Regex::new(on)?;
                    match re.is_match(&url) {
                        true => Ok(()),
                        false => Err(anyhow!(
                            "Skip rule because the page({url}) doesn't match url rule."
                        )),
                    }
                }
                Ok(None) | Err(_) => Err(anyhow!("Skip rule because the page has no url.")),
            },
            None => Ok(()),
        }
    }

    pub async fn fillin(&self, input: &Input) -> anyhow::Result<()> {
        self.is_on(input.on.as_ref()).await?;

        let element = self.page.find_element(&input.to).await?;
        element.click().await?.type_str(&input.value).await?;
        Ok(())
    }

    pub async fn totp(&self, totp: &Totp) -> anyhow::Result<()> {
        self.is_on(totp.on.as_ref()).await?;

        #[allow(deprecated)]
        let generator = TOTPBuilder::new()
            .base32_key(&totp.seed)
            .output_len(6)
            .hash_function(HashFunction::Sha1)
            .finalize()
            .unwrap();

        let element = self.page.find_element(&totp.to).await?;
        element.click().await?.type_str(generator.generate()).await?;
        Ok(())
    }

    pub async fn click(&self, click: &Click) -> anyhow::Result<()> {
        self.is_on(click.on.as_ref()).await?;

        let element = self.page.find_element(&click.to).await?;
        element.click().await?;
        //self.page.wait_for_navigation().await?;
        Ok(())
    }

    pub async fn cookies(&self) -> anyhow::Result<Vec<Cookie>> {
        let cookies = self
            .page
            .get_cookies()
            .await
            .context("Failed to get cookies")?;

        debug!("{cookies:?}");

        Ok(cookies)
    }

    pub async fn acquire(&self, cookeys: &[String]) -> anyhow::Result<Vec<Cookie>> {
        let cookies = self.cookies().await?;

        let found: Vec<Cookie> = cookies
            .into_iter()
            .filter(|cookie| cookeys.contains(&cookie.name))
            .collect();

        info!("Found {found:?}");

        Ok(found)
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
        let args = args!["--headless", "--url", url, "dummy.toml"];
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
        let args = args!["--timeout", "5", "--headless", "--url", url, "dummy.toml"];
        let acquirer = Acquirer::launch(AcquirerConfig::build(&args).unwrap())
            .await
            .unwrap();
        let Some(url) = args.url else { panic!() };
        acquirer.navigate(&(url).into()).await.unwrap();
        acquirer.close().await.unwrap();
    }
}
