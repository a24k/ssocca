use chromiumoxide::cdp::browser_protocol::page::NavigateParams;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Rule {
    Input(Input),
    Totp(Totp),
    Click(Click),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Start(pub NavigateParams);

type UrlPattern = String;
type CssSelector = String;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Input {
    pub on: Option<UrlPattern>,
    pub to: CssSelector,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Totp {
    pub on: Option<UrlPattern>,
    pub to: CssSelector,
    pub seed: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Click {
    pub on: Option<UrlPattern>,
    pub to: CssSelector,
}

type CookieKey = String;
type CookieDomain = String;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Finish {
    pub on: Option<CookieDomain>,
    #[serde(default)]
    pub with: Vec<CookieKey>,
}

impl Default for Finish {
    fn default() -> Self {
        Finish {
            on: None,
            with: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

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
