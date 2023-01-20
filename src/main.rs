mod args;
mod acquirer;

use args::{Args, Parser};
use acquirer::Acquirer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    Acquirer::new(args)?;

    Ok(())
}
