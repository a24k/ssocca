use async_std::task;
use log::warn;
use std::time::Duration;

use super::{Acquirer, Scenario};

pub async fn run(acquirer: &Acquirer, scenario: &Scenario) -> anyhow::Result<()> {
    // Start
    acquirer.navigate(&scenario.start.0).await?;

    // Finish
    let mut cookeys = scenario.finish.with.clone();
    while !cookeys.is_empty() {
        task::sleep(Duration::from_millis(500)).await;

        let cookies = acquirer.acquire(&cookeys).await?;
        let cookies_keys: Vec<String> = cookies.iter().map(|cookie| cookie.name.clone()).collect();

        cookeys.retain(|cookey| !cookies_keys.contains(cookey));

        cookies
            .iter()
            .for_each(|cookie| println!("{}={}", cookie.name, cookie.value));

        // Input / Totp / Click
        for rule in &scenario.rules {
            task::sleep(Duration::from_millis(200)).await;
            let result = match rule {
                super::scenario::rule::Rule::Input(input) => acquirer.fillin(input).await,
                super::scenario::rule::Rule::Totp(totp) => acquirer.totp(totp).await,
                super::scenario::rule::Rule::Click(click) => acquirer.click(click).await,
            };
            if let Err(err) = result {
                warn!("{:#}", err);
            }
        }
    }

    Ok(())
}
