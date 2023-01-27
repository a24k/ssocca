use anyhow::{anyhow, Context as _};

use chromiumoxide::browser::BrowserConfig;
use chromiumoxide::handler::viewport::Viewport;

use crate::args::Args;

pub fn build(args: &Args) -> anyhow::Result<BrowserConfig> {
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

#[cfg(test)]
mod tests {
    use rstest::*;

    use crate::args::{Args, Parser as _};

    #[rstest]
    #[case(vec!["ssocca", "https://example.com/"])]
    #[case(vec!["ssocca", "--headless", "https://example.com/"])]
    #[case(vec!["ssocca", "--chrome", "/path/to/chrome", "https://example.com/"])]
    fn build(#[case] input: Vec<&str>) {
        let args = Args::parse_from(input);
        assert!(super::build(&args).is_ok());
    }
}
