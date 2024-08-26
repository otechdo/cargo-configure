use crate::config::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The assertions on result states id"]
pub const ASSERTIONS_ON_RESULTS_STATES_ID: &str = "assertions_on_result_states";
#[doc = "The assertions on result states description"]
pub const ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION: &str =
    "Checks for assert!(r.is_ok()) or assert!(r.is_err()) calls.";
#[doc = "The assertions on result states know problem"]
pub const ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM: Option<&'static str> =
    Some("The suggested replacement decreases the readability of code and log output.");
#[doc = "The assertions on result states what it's bad"]
pub const ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD: &str = "This form of assertion does not show any of the information present in the Result other than which variant is none";
#[doc = "The as underscore uri issue"]
pub const ASSERTIONS_ON_RESULTS_STATES_ISSUE: Option<&'static str> = Some(
    "https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assertions_on_result_states",
);

#[doc = "The assertions on result states for novice"]
pub const NOVICE_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assertions on result states for expert"]
pub const EXPERT_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The assertions on result states for master"]
pub const MASTER_ASSERTIONS_ON_RESULTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_RESULTS_STATES_ID,
    description: ASSERTIONS_ON_RESULTS_STATES_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_RESULTS_STATES_WHATS_BAD,
    known_problems: ASSERTIONS_ON_RESULTS_STATES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ASSERTIONS_ON_RESULTS_STATES_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};
