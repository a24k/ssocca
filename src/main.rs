mod acquirer;
mod args;
mod logger;

use anyhow::anyhow;
use async_std::task;
use std::process::ExitCode;

use acquirer::{Acquirer, AcquirerConfig};
use args::Args;
use log::error;

fn main() -> ExitCode {
    async fn main(args: &Args) -> anyhow::Result<()> {
        let acquirer = Acquirer::launch(AcquirerConfig::build(args)?).await?;

        acquirer.dump().await?;
        acquirer.navigate(&args.url).await?;
        acquirer.dump().await?;

        acquirer.close().await
    }

    let args: anyhow::Result<Args> = clap::Parser::try_parse().map_err(|e| anyhow!(e));

    match args {
        Ok(args) => {
            logger::init(&args.verbosity);
            let ret = task::block_on(async { main(&args).await });
            match ret {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    error!("{}", e);
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            env_logger::builder().init();
            error!("{}", e);
            ExitCode::FAILURE
        }
    }
}
