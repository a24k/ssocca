mod acquirer;
mod args;

use acquirer::Acquirer;
use args::{Args, Parser};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let acquirer = Acquirer::launch(args.headless).await?;

    let page = acquirer.navigate(&args.url).await?;

    acquirer.dump(&page).await?;

    //acquirer.handle.await;

    Ok(())
}
