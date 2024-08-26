use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The assertions on constants id"]
pub const ASSERTIONS_ON_CONSTANTS_ID: &str = "assertions_on_constants";
#[doc = "The assertions on constants description"]
pub const ASSERTIONS_ON_CONSTANTS_DESCRIPTION: &str = "Checks for assert!(true) and assert!(false) calls.";
#[doc = "The assertions on constants know problem"]
pub const ASSERTIONS_ON_CONSTANTS_PROBLEM: Option<&'static str> = None;
#[doc = "The assertions on constants what it's bad"]
pub const ASSERTIONS_ON_CONSTANTS_WHATS_BAD: &str = "Will be optimized out by the compiler or should probably be replaced by a panic!() or unreachable!()";
#[doc = "The assertions on constants uri issue"]
pub const ASSERTIONS_ON_CONSTANTS_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+assertions_on_constants");

#[doc = "The as ptr cast mut for novice"]
pub const NOVICE_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Style,
    issue: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The as ptr cast mut for expert"]
pub const EXPERT_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    issue: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The as ptr cast mut for master"]
pub const MASTER_ASSERTIONS_ON_CONSTANTS: ClippyLint = ClippyLint {
    id: ASSERTIONS_ON_CONSTANTS_ID,
    description: ASSERTIONS_ON_CONSTANTS_DESCRIPTION,
    whats_bad: ASSERTIONS_ON_CONSTANTS_WHATS_BAD,
    known_problems: ASSERTIONS_ON_CONSTANTS_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Style,
    issue: ASSERTIONS_ON_CONSTANTS_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
