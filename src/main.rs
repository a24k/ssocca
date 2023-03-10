mod acquirer;
mod args;
mod logger;

use async_std::task;
use log::error;
use std::process::ExitCode;
use std::time::Duration;

use acquirer::{Acquirer, AcquirerConfig};
use args::Args;

fn main() -> ExitCode {
    async fn main(args: &Args) -> anyhow::Result<()> {
        let acquirer = Acquirer::launch(AcquirerConfig::build(args)?).await?;

        acquirer.navigate(&args.url).await?;

        if let Some(cookie) = &args.cookie {
            loop {
                task::sleep(Duration::from_millis(500)).await;
                let cookie = acquirer.acquire(cookie).await?;
                if let Some(cookie) = cookie {
                    println!("{}={}", cookie.name, cookie.value);
                    break;
                }
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
