use anyhow::anyhow;
use async_std::fs;
use serde::{Deserialize, Serialize};

use crate::args::Args;

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub start: rule::Start,
    pub rules: Vec<rule::Rule>,
    pub finish: rule::Finish,
}

impl Scenario {
    pub async fn build(args: &Args) -> anyhow::Result<Scenario> {
        let scenario_from_toml = Self::build_from_toml(&args.toml).await;
        let scenario_from_args = Self::build_from_args(args).await;

        match (scenario_from_toml, scenario_from_args) {
            (Ok(scenario_from_toml), Ok(scenario_from_args)) => Ok(Scenario {
                start: scenario_from_args.start,
                rules: scenario_from_toml.rules,
                finish: rule::Finish {
                    on: scenario_from_toml.finish.on,
                    with: scenario_from_toml
                        .finish
                        .with
                        .into_iter()
                        .chain(scenario_from_args.finish.with.into_iter())
                        .collect(),
                },
            }),
            (Ok(scenario), Err(_)) | (Err(_), Ok(scenario)) => Ok(scenario),
            (Err(_), Err(_)) => Err(anyhow!("Found no toml configuration or url option.")),
        }
    }

    async fn build_from_toml(toml: &Option<std::path::PathBuf>) -> anyhow::Result<Scenario> {
        match toml {
            Some(toml) => {
                let toml = fs::read_to_string(toml).await?;
                toml::from_str(&toml).map_err(|e| anyhow!(e))
            }
            None => Err(anyhow!("Found no toml configuration.")),
        }
    }

    async fn build_from_args(args: &Args) -> anyhow::Result<Scenario> {
        let url = args
            .url
            .as_ref()
            .ok_or_else(|| anyhow!("Found no url option."))?;
        Ok(Scenario {
            start: rule::Start(url.into()),
            rules: vec![],
            finish: rule::Finish {
                on: None,
                with: args.cookie.clone(),
            },
        })
    }
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(tag = "type", rename_all = "lowercase")]
    pub enum Rule {
        Input(Input),
        Totp(Totp),
        Click(Click),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Start(pub NavigateParams);

    type UrlPattern = String;
    type CssSelector = String;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Input {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub value: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Totp {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub seed: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Click {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
    }

    type CookieKey = String;
    type CookieDomain = String;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Finish {
        pub on: Option<CookieDomain>,
        pub with: Vec<CookieKey>,
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::rule::*;

    #[rstest]
    #[case(
        "https://example.com",
        Start ( "https://example.com".into() ),
    )]
    fn start(#[case] expected: &str, #[case] rule: Start) {
        assert_eq!(expected, rule.0.url);
        assert_eq!(None, rule.0.referrer);
        assert_eq!(None, rule.0.transition_type);
        assert_eq!(None, rule.0.frame_id);
        assert_eq!(None, rule.0.referrer_policy);
    }

    #[rstest]
    #[case(
        vec!["cookey"],
        None,
        Finish { with: vec!["cookey".into()], on: None },
    )]
    #[case(
        vec!["cookey1", "cookey2"],
        None,
        Finish { with: vec!["cookey1".into(), "cookey2".into()], on: None },
    )]
    #[case(
        vec!["cookey1", "cookey2"],
        Some("example.com"),
        Finish { with: vec!["cookey1".into(), "cookey2".into()], on: Some("example.com".into()) },
    )]
    fn finish(
        #[case] expected_with: Vec<&str>,
        #[case] expected_on: Option<&str>,
        #[case] rule: Finish,
    ) {
        assert_eq!(expected_with, rule.with);
        assert_eq!(expected_on.map(|str| str.into()), rule.on);
    }
}
