mod acquirer;
mod args;
mod logger;

use acquirer::{Acquirer, AcquirerConfig};
use args::Args;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = clap::Parser::parse();

    logger::init(&args.verbosity);

    let acquirer = Acquirer::launch(AcquirerConfig::build(&args)?).await?;

    acquirer.dump().await?;
    acquirer.navigate(&args.url).await?;
    acquirer.dump().await?;

    acquirer.close().await
}
