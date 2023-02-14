mod acquirer;
mod args;
mod logger;

use async_std::task;
use log::error;
use std::process::ExitCode;
use std::time::Duration;

use acquirer::{
    scenario::rule::{Finish, Start},
    Acquirer, AcquirerConfig, Scenario,
};
use args::Args;

fn main() -> ExitCode {
    async fn main(args: &Args) -> anyhow::Result<()> {
        let acquirer = Acquirer::launch(AcquirerConfig::build(args)?).await?;

        let scenario = Scenario {
            start: Some(Start {
                goto: (&args.url).into(),
            }),
            rules: vec![],
            finish: args.cookie.as_ref().map(|cookie| Finish {
                on: None,
                with: vec![cookie.into()],
            }),
        };

        if let Some(start) = scenario.start {
            acquirer.navigate(&start.goto).await?;
        }

        if let Some(finish) = scenario.finish {
            let mut cookeys = finish.with;
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
