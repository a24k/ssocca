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
        let scenario: anyhow::Result<Scenario> = Self::build_from_toml(&args.toml).await;

        let start = scenario.as_ref().map_or_else(
            |_| match &args.url {
                Some(url) => Ok(rule::Start(url.into())),
                None => Err(anyhow!("Found no toml configuration or url option.")),
            },
            |scenario| Ok(scenario.start.clone()),
        )?;

        let rules = scenario
            .as_ref()
            .map_or_else(|_| vec![], |scenario| scenario.rules.clone());

        let finish = {
            let on = scenario
                .as_ref()
                .map_or(None, |scenario| scenario.finish.on.clone());

            let with = scenario.as_ref().map_or_else(
                |_| args.cookie.clone(),
                |scenario| {
                    scenario
                        .finish
                        .with
                        .clone()
                        .into_iter()
                        .chain(args.cookie.clone().into_iter())
                        .collect()
                },
            );

            rule::Finish { on, with }
        };

        Ok(Scenario {
            start,
            rules,
            finish,
        })
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
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(tag = "type", rename_all = "lowercase")]
    pub enum Rule {
        Input(Input),
        Totp(Totp),
        Click(Click),
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Start(pub NavigateParams);

    type UrlPattern = String;
    type CssSelector = String;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Input {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub value: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Totp {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub seed: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
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
