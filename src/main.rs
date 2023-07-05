mod acquirer;
mod args;
mod logger;

use async_std::task;
use log::error;
use std::process::ExitCode;

use acquirer::{runner, Acquirer, AcquirerConfig, Scenario};
use args::Args;

fn main() -> ExitCode {
    async fn main(args: &Args) -> anyhow::Result<()> {
        let scenario = Scenario::build(args).await?;

        let acquirer = Acquirer::launch(AcquirerConfig::build(args)?).await?;

        runner::run(&acquirer, &scenario).await?;

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
