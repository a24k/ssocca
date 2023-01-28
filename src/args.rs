pub use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Args {
    /// Use browser in headless mode
    #[arg(short('l'), long, default_value_t = false)]
    pub headless: bool,

    #[clap(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,

    /// Specify path to a Chrome executable
    #[arg(long, value_name = "PATH")]
    pub chrome: Option<std::path::PathBuf>,

    /// Timeout duration in secs
    #[arg(long, default_value_t = 10, value_name = "SEC")]
    pub timeout: u8,

    /// Url to initiate authentication
    pub url: String,
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use std::path::PathBuf;

    use super::{Args, Parser as _};

    #[rstest]
    fn verify() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }

    #[rstest]
    #[case(
        false,
        vec!["ssocca", "https://example.com/"],
    )]
    #[case(
        true,
        vec!["ssocca", "-l", "https://example.com/"],
    )]
    #[case(
        true,
        vec!["ssocca", "--headless", "https://example.com/"],
    )]
    fn headless(#[case] expected: bool, #[case] input: Vec<&str>) {
        let args = Args::parse_from(input);
        assert_eq!(expected, args.headless);
    }

    #[rstest]
    #[case(
        None,
        vec!["ssocca", "https://example.com/"],
    )]
    #[case(
        Some(PathBuf::from("/path/to/chrome")),
        vec!["ssocca", "--chrome", "/path/to/chrome", "https://example.com/"],
    )]
    fn chrome(#[case] expected: Option<PathBuf>, #[case] input: Vec<&str>) {
        let args = Args::parse_from(input);
        assert_eq!(expected, args.chrome);
    }

    #[rstest]
    #[case(
        10,
        vec!["ssocca", "https://example.com/"],
    )]
    #[case(
        5,
        vec!["ssocca", "--timeout", "5", "https://example.com/"],
    )]
    fn timeout(#[case] expected: u8, #[case] input: Vec<&str>) {
        let args = Args::parse_from(input);
        assert_eq!(expected, args.timeout);
    }
}
