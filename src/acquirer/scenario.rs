use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Scenario {
    pub start: rule::Start,
    pub rules: Vec<rule::Rule>,
    pub finish: rule::Finish,
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(tag = "type", rename_all="lowercase")]
    pub enum Rule {
        Input(Input),
        Totp(Totp),
        Click(Click),
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Start {
        pub from: NavigateParams,
    }

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
        Start { from: "https://example.com".into() },
    )]
    fn start(#[case] expected: &str, #[case] rule: Start) {
        assert_eq!(expected, rule.from.url);
        assert_eq!(None, rule.from.referrer);
        assert_eq!(None, rule.from.transition_type);
        assert_eq!(None, rule.from.frame_id);
        assert_eq!(None, rule.from.referrer_policy);
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
