pub mod rule;

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
        Self::override_with_args(
            match &args.toml {
                Some(toml) => Self::build_from_toml(fs::read_to_string(toml).await?),
                None => Err(anyhow!("Found no toml configuration.")),
            },
            args,
        )
    }

    fn build_from_toml(toml: String) -> anyhow::Result<Scenario> {
        toml::from_str(&toml).map_err(|e| anyhow!(e))
    }

    fn override_with_args(
        scenario: anyhow::Result<Scenario>,
        args: &Args,
    ) -> anyhow::Result<Scenario> {
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
}
