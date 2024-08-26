use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The big endian bytes id"]
pub const BIG_ENDIAN_BYTES_ID: &str = "big_endian_bytes";
#[doc = "The big endian bytes description"]
pub const BIG_ENDIAN_BYTES_DESCRIPTION: &str = "Checks for the usage of the to_be_bytes method and/or the function from_be_bytes.";
#[doc = "The big endian bytes know problem"]
pub const BIG_ENDIAN_BYTES_KNOW_PROBLEM: Option<&'static str> = None;
#[doc = "The big endian bytes what it's bad"]
pub const BIG_ENDIAN_BYTES_WHATS_BAD: &str = "To ensure use of little-endian or the target’s endianness rather than big-endian.";
#[doc = "The big endian bytes uri issue"]
pub const BIG_ENDIAN_BYTES_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+big_endian_bytes");

#[doc = "The big endian bytes for novice"]
pub const NOVICE_BIG_ENDIAN_BYTES: ClippyLint = ClippyLint {
    id: BIG_ENDIAN_BYTES_ID,
    description: BIG_ENDIAN_BYTES_DESCRIPTION,
    whats_bad: BIG_ENDIAN_BYTES_WHATS_BAD,
    known_problems: BIG_ENDIAN_BYTES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: BIG_ENDIAN_BYTES_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The big endian bytes for expert"]
pub const EXPERT_BIG_ENDIAN_BYTES: ClippyLint = ClippyLint {
    id: BIG_ENDIAN_BYTES_ID,
    description: BIG_ENDIAN_BYTES_DESCRIPTION,
    whats_bad: BIG_ENDIAN_BYTES_WHATS_BAD,
    known_problems: BIG_ENDIAN_BYTES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: BIG_ENDIAN_BYTES_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "The big endian bytes for master"]
pub const MASTER_BIG_ENDIAN_BYTES: ClippyLint = ClippyLint {
    id: BIG_ENDIAN_BYTES_ID,
    description: BIG_ENDIAN_BYTES_DESCRIPTION,
    whats_bad: BIG_ENDIAN_BYTES_WHATS_BAD,
    known_problems: BIG_ENDIAN_BYTES_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: BIG_ENDIAN_BYTES_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};