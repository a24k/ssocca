mod args;

use args::{Args, Parser};

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
