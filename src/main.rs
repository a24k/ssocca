mod acquirer;
mod args;

use acquirer::Acquirer;
use args::{Args, Parser as _};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init();

    let acquirer = Acquirer::launch(args.headless).await?;

    let page = acquirer.navigate(&args.url).await?;

    acquirer.dump(&page).await?;

    acquirer.close().await
}
