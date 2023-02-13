#[allow(dead_code)]
pub struct Scenario {
    rules: Vec<rule::Rule>,
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;

    use urlpattern::UrlPattern;

    #[allow(dead_code)]
    pub enum Rule {
        Start(Start),
        Input(Input),
        Finish(Finish),
    }

    #[allow(dead_code)]
    pub struct Start {
        pub goto: NavigateParams,
    }

    type CssSelector = String;

    #[allow(dead_code)]
    pub struct Input {
        pub on: Option<UrlPattern>,
        pub to: CssSelector,
        pub value: String,
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
    use regex::Regex;
    use url::Url;
    use urlpattern::{UrlPattern, UrlPatternInit, UrlPatternMatchInput};

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
        "https://example.com",
        "input[type=email]",
        "mail@example.com",
        Input { on: None, to: "input[type=email]".into(), value: "mail@example.com".into() },
        )]
    #[case(
        "https://example.com",
        "input[type=email]",
        "mail@example.com",
        Input {
            on: Some(UrlPattern::parse(UrlPatternInit::parse_constructor_string::<Regex>("https://*.com", None).unwrap()).unwrap()),
            to: "input[type=email]".into(), value: "mail@example.com".into() },
        )]
    #[case(
        "https://example.com/login/first?auth=id",
        "input[type=email]",
        "mail@example.com",
        Input {
            on: Some(UrlPattern::parse(UrlPatternInit::parse_constructor_string::<Regex>("https://*.com/login/first?auth=:auth", None).unwrap()).unwrap()),
            to: "input[type=email]".into(), value: "mail@example.com".into() },
        )]
    #[case(
        "https://example.com/login/first?auth=id",
        "input[type=email]",
        "mail@example.com",
        Input {
            on: Some(UrlPattern::parse(UrlPatternInit{
                hostname: Some("*.com".into()),
                pathname: Some("/login/:path".into()),
                search: Some("auth=:auth".into()),
                ..Default::default()
            }).unwrap()),
            to: "input[type=email]".into(), value: "mail@example.com".into() },
        )]
    fn input(
        #[case] match_url: &str,
        #[case] expected_to: &str,
        #[case] expected_value: &str,
        #[case] rule: Input,
    ) {
        match rule.on {
            Some(pattern) => assert!(pattern
                .test(UrlPatternMatchInput::Url(Url::parse(match_url).unwrap()))
                .unwrap()),
            None => assert!(true),
        }
        assert_eq!(expected_to, rule.to);
        assert_eq!(expected_value, rule.value);
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
