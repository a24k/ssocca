#[allow(dead_code)]
pub struct Scenario {
    rules: Vec<rule::Rule>,
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;

    #[allow(dead_code)]
    pub enum Rule {
        Start(Start),
        Finish(Finish),
    }

    #[allow(dead_code)]
    pub struct Start {
        pub goto: NavigateParams,
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
    fn start(#[case] expected: &str, #[case] input: Start) {
        assert_eq!(expected, input.goto.url);
        assert_eq!(None, input.goto.referrer);
        assert_eq!(None, input.goto.transition_type);
        assert_eq!(None, input.goto.frame_id);
        assert_eq!(None, input.goto.referrer_policy);
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
        #[case] input: Finish,
    ) {
        assert_eq!(expected_with, input.with);
        assert_eq!(expected_on.map(|str| str.into()), input.on);
    }
}
