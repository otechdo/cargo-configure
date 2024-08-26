use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The approx constant id"]
pub const APPROX_CONSTANT_ID: &str = "approx_constant";
#[doc = "The approx constant description"]
pub const APPROX_CONSTANT_DESCRIPTION: &str = "Checks for floating point literals that approximate constants which are defined in std::f32::consts or std::f64::consts, respectively, suggesting to use the predefined constant.";
#[doc = "The approx constant know problem"]
pub const APPROX_CONSTANT_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The approx constant what it's bad"]
pub const APPROX_CONSTANT_WHATS_BAD: &str = "Usually, the definition in the standard library is more precise than what people come up with. If you find that your definition is actually more precise.";
#[doc = "The approx constant issue uri"]
pub const APPROX_CONSTANT_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+approx_constant");

#[doc = "The almost complete range for novice"]
pub const NOVICE_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for expert"]
pub const EXPERT_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The almost complete range for master"]
pub const MASTER_APPROX_CONSTANT: ClippyLint = ClippyLint {
    id: APPROX_CONSTANT_ID,
    description: APPROX_CONSTANT_DESCRIPTION,
    whats_bad: APPROX_CONSTANT_WHATS_BAD,
    known_problems: APPROX_CONSTANT_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: APPROX_CONSTANT_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};