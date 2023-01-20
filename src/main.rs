mod acquirer;
mod args;

use std::error::Error;
use std::{thread, time};

use acquirer::Acquirer;
use args::{Args, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let acquirer = Acquirer::launch(args.headless)?;

    acquirer.navigate(&args.url)?;

    thread::sleep(time::Duration::from_millis(3000));

    acquirer.dump()
}
