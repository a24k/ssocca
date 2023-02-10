#[allow(dead_code)]
pub struct Scenario {
    rules: Vec<rule::Rule>,
}

pub mod rule {
    use chromiumoxide::cdp::browser_protocol::page::NavigateParams;

    #[allow(dead_code)]
    pub enum Rule {
        Start(Start),
    }

    #[allow(dead_code)]
    pub struct Start {
        pub goto: NavigateParams,
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::rule::Start;

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
}
