mod acquirer;
mod args;

use acquirer::{Acquirer, AcquirerOxide};
use args::{Args, Parser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let _acquirer = AcquirerOxide::launch_oxide().await?;

    let acquirer = Acquirer::launch(args.headless)?;

    acquirer.navigate(&args.url)?;

    acquirer.dump()
}
