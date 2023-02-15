use anyhow::{anyhow, Context as _};
use std::time::Duration;

use chromiumoxide::browser::BrowserConfig;
use chromiumoxide::handler::viewport::Viewport;

use crate::args::Args;

pub struct AcquirerConfig {
    pub browser: BrowserConfig,
    pub timeout: Duration,
}

impl AcquirerConfig {
    pub fn build(args: &Args) -> anyhow::Result<AcquirerConfig> {
        let browser = Self::build_browser_config(args)?;

        let timeout = Duration::from_secs(args.timeout.into());

        Ok(AcquirerConfig { browser, timeout })
    }

    fn build_browser_config(args: &Args) -> anyhow::Result<BrowserConfig> {
        let viewport = Viewport {
            width: 0,
            height: 0,
            ..Default::default()
        };

        let builder = BrowserConfig::builder().viewport(viewport).incognito();

        let builder = match &args.chrome {
            None => builder,
            Some(path) => builder.chrome_executable(path),
        };

        let builder = match args.headless {
            true => builder,
            false => builder.with_head(),
        };

        builder
            .build()
            .map_err(|e| anyhow!(e))
            .context("Failed to build browser config")
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use std::time::Duration;

    use crate::{args, args::Args};

    #[rstest]
    #[case(args!["--url", "https://example.com/"])]
    #[case(args!["--headless", "--url", "https://example.com/"])]
    #[case(args!["--chrome", "/path/to/chrome", "--url", "https://example.com/"])]
    #[case(args!["--timeout", "5", "--url", "https://example.com/"])]
    fn build(#[case] args: Args) {
        assert!(super::AcquirerConfig::build(&args).is_ok());
    }

    #[rstest]
    #[case(
        Duration::from_secs(10),
        args!["--url", "https://example.com/"],
    )]
    #[case(
        Duration::from_secs(5),
        args!["--timeout", "5", "--url", "https://example.com/"],
    )]
    fn timeout(#[case] estimated: Duration, #[case] args: Args) {
        let config = super::AcquirerConfig::build(&args).unwrap();
        assert_eq!(estimated, config.timeout);
    }
}
