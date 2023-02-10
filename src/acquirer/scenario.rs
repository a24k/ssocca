use chromiumoxide::cdp::browser_protocol::page::NavigateParams;

#[allow(dead_code)]
pub struct AcquirerScenario {
    rules: Vec<AcquirerScenarioRule>,
}

#[allow(dead_code)]
pub enum AcquirerScenarioRule {
    Start(AcquirerScenarioRuleStart),
}

#[allow(dead_code)]
pub struct AcquirerScenarioRuleStart {
    pub goto: NavigateParams,
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case(
        "https://example.com",
        AcquirerScenarioRuleStart { goto: "https://example.com".into() },
    )]
    fn start(#[case] expected: &str, #[case] input: AcquirerScenarioRuleStart) {
        assert_eq!(expected, input.goto.url);
        assert_eq!(None, input.goto.referrer);
        assert_eq!(None, input.goto.transition_type);
        assert_eq!(None, input.goto.frame_id);
        assert_eq!(None, input.goto.referrer_policy);
    }
}
