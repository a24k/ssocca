#[allow(dead_code)]
pub struct Scenario {
    pub start: Option<rule::Start>,      // TODO: to be mandatory
    pub rules: Vec<rule::Rule>,
    pub finish: Option<rule::Finish>,    // TODO: to be mandatory
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;

    #[allow(dead_code)]
    pub enum Rule {
        Input(Input),
        Totp(Totp),
        Click(Click),
    }

    #[allow(dead_code)]
    pub struct Start {
        pub goto: NavigateParams,
    }

    type UrlPattern = String;
    type CssSelector = String;

    #[allow(dead_code)]
    pub struct Input {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub value: String,
    }

    #[allow(dead_code)]
    pub struct Totp {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub seed: String,
    }

    #[allow(dead_code)]
    pub struct Click {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
    }

    type CookieKey = String;
    type CookieDomain = String;

    #[allow(dead_code)]
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
        Start { goto: "https://example.com".into() },
    )]
    fn start(#[case] expected: &str, #[case] rule: Start) {
        assert_eq!(expected, rule.goto.url);
        assert_eq!(None, rule.goto.referrer);
        assert_eq!(None, rule.goto.transition_type);
        assert_eq!(None, rule.goto.frame_id);
        assert_eq!(None, rule.goto.referrer_policy);
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
