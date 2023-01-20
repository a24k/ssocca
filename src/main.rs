mod acquirer;
mod args;

use acquirer::Acquirer;
use args::{Args, Parser};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let acquirer = Acquirer::launch(args.headless)?;

    acquirer.navigate(&args.url)?;

    acquirer.dump()
}
