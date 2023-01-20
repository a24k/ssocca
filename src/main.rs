mod acquirer;
mod args;

use std::error::Error;

use acquirer::Acquirer;
use args::{Args, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    Acquirer::launch(&args)?;

    Ok(())
}
