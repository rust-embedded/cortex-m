use crate::TestOutcome;
use cortex_m_rt as _;

pub fn check_outcome<T: TestOutcome>(outcome: T, should_error: bool) {
    if outcome.is_success() == should_error {
        let note: &str = if should_error {
            "`#[should_error]` "
        } else {
            ""
        };
        panic!("{}test failed with outcome: {:?}", note, outcome);
    }
}
