use anyhow::{anyhow, Context as _};
use std::path::PathBuf;

use chromiumoxide::browser::BrowserConfig;
use chromiumoxide::handler::viewport::Viewport;

pub(super) fn build(headless: bool, chrome: Option<PathBuf>) -> anyhow::Result<BrowserConfig> {
    let viewport = Viewport {
        width: 0,
        height: 0,
        ..Default::default()
    };

    let builder = BrowserConfig::builder().viewport(viewport);

    let builder = match chrome {
        Some(path) => builder.chrome_executable(path),
        None => builder,
    };

    let builder = match headless {
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

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn build(#[case] headless: bool) {
        assert!(super::build(headless, None).is_ok());
    }
}
