mod acquirer;
mod args;
mod logger;

use acquirer::{config, Acquirer};
use args::{Args, Parser as _};

#[async_std::main]
async fn main() -> anyhow::Result<()> {

    let args = Args::parse();

    logger::init(&args.verbosity);

    let acquirer = Acquirer::launch(config::build(&args)?).await?;

    acquirer.dump().await?;
    acquirer.navigate_with_timeout(&args.url, std::time::Duration::from_secs(5)).await?;
    acquirer.dump().await?;

    acquirer.close().await
}
