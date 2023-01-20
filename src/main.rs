use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct Args {
    /// Use browser in headless mode
    #[arg(short('l'), long, default_value_t = false)]
    headless: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
