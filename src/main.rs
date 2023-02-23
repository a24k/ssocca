mod acquirer;
mod args;
mod logger;

use async_std::task;
use log::error;
use std::process::ExitCode;
use std::time::Duration;

use acquirer::{
    scenario::rule::{Finish, Input, Rule, Start},
    Acquirer, AcquirerConfig, Scenario,
};
use args::Args;

fn main() -> ExitCode {
    async fn main(args: &Args) -> anyhow::Result<()> {
        let acquirer = Acquirer::launch(AcquirerConfig::build(args)?).await?;

        let scenario = Scenario {
            start: Start {
                from: (&args.url).into(),
            },
            rules: vec![
                Rule::Input(Input {
                    on: None,
                    to: "selector01".into(),
                    value: "value01".into(),
                }),
                Rule::Input(Input {
                    on: None,
                    to: "selector02".into(),
                    value: "value02".into(),
                }),
            ],
            finish: Finish {
                on: None,
                with: args.cookie.clone(),
            },
        };

        // Serialize TOML
        println!("{}", toml::to_string(&scenario).unwrap());

        // Start
        acquirer.navigate(&scenario.start.from).await?;

        // Finish
        let mut cookeys = scenario.finish.with;
        while !cookeys.is_empty() {
            task::sleep(Duration::from_millis(500)).await;

            let cookies = acquirer.acquire(&cookeys).await?;
            let cookies_keys: Vec<String> =
                cookies.iter().map(|cookie| cookie.name.clone()).collect();

            cookeys.retain(|cookey| !cookies_keys.contains(cookey));

            cookies
                .iter()
                .for_each(|cookie| println!("{}={}", cookie.name, cookie.value));
        }

        acquirer.close().await
    }

    let args: Args = match clap::Parser::try_parse() {
        Ok(args) => args,
        Err(err) => return logger::handle_clap_error(err),
    };

    logger::init_with_verbosity(&args.verbosity);

    let result = task::block_on(async { main(&args).await });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{:#}", err);
            ExitCode::FAILURE
        }
    }
}
