use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct Args {
    /// Use browser in headless mode
    #[arg(long, default_value_t = false)]
    headless: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
