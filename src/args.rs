#[derive(Debug, clap::Parser)]
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

    /// Cookie name to acquire
    #[arg(long, value_name = "NAME")]
    pub cookie: Vec<String>,

    /// Url to initiate authentication
    #[arg(long, value_name = "URL")]
    pub url: String,
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use std::path::PathBuf;

    use super::Args;

    #[macro_export]
    macro_rules! args {
        ($($e:expr),*) => {
            clap::Parser::parse_from(vec!["ssocca", $($e),*])
        };
    }

    #[rstest]
    fn verify() {
        use clap::CommandFactory;
        Args::command().debug_assert()
    }

    #[rstest]
    #[case(
        false,
        args!["--url", "https://example.com/"],
    )]
    #[case(
        true,
        args!["-l", "--url", "https://example.com/"],
    )]
    #[case(
        true,
        args!["--headless", "--url", "https://example.com/"],
    )]
    fn headless(#[case] expected: bool, #[case] args: Args) {
        assert_eq!(expected, args.headless);
    }

    #[rstest]
    #[case(
        None,
        args!["--url", "https://example.com/"],
    )]
    #[case(
        Some(PathBuf::from("/path/to/chrome")),
        args!["--chrome", "/path/to/chrome", "--url", "https://example.com/"],
    )]
    fn chrome(#[case] expected: Option<PathBuf>, #[case] args: Args) {
        assert_eq!(expected, args.chrome);
    }

    #[rstest]
    #[case(
        vec![],
        args!["--url", "https://example.com/"],
    )]
    #[case(
        vec![String::from("cookie_name")],
        args!["--cookie", "cookie_name", "--url", "https://example.com/"],
    )]
    #[case(
        vec![String::from("cookie_name1"), String::from("cookie_name2")],
        args!["--cookie", "cookie_name1", "--cookie", "cookie_name2", "--url", "https://example.com/"],
    )]
    fn cookie(#[case] expected: Vec<String>, #[case] args: Args) {
        assert_eq!(expected, args.cookie);
    }

    #[rstest]
    #[case(
        10,
        args!["--url", "https://example.com/"],
    )]
    #[case(
        5,
        args!["--timeout", "5", "--url", "https://example.com/"],
    )]
    fn timeout(#[case] expected: u8, #[case] args: Args) {
        assert_eq!(expected, args.timeout);
    }
}
