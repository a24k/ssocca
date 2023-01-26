pub use clap::Parser;

use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Args {
    /// Use browser in headless mode
    #[arg(short('l'), long, default_value_t = false)]
    pub headless: bool,

    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,

    /// Specify the path to Chrome executable
    #[arg(long)]
    pub chrome: Option<PathBuf>,

    /// Url to initiate authentication
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn verify() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }
}
