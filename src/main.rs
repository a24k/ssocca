mod acquirer;
mod args;
mod logger;

use acquirer::Acquirer;
use args::{Args, Parser as _};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    logger::init(args.verbosity);

    let acquirer = Acquirer::launch(args.headless, args.chrome).await?;

    let page = acquirer.navigate(&args.url).await?;

    acquirer.dump(&page).await?;

    acquirer.close().await
}
