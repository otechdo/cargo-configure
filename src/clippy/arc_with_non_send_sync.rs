use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};

#[doc = "The arc with non send sync id"]
pub const ARC_WITH_NO_SEND_SYNC_ID: &str = "arc_with_non_send_sync";
#[doc = "The arc with non send sync description"]
pub const ARC_WITH_NO_SEND_SYNC_DESCRIPTION: &str =
    "This lint warns when you use Arc with a type that does not implement Send or Sync.";
#[doc = "The arc with non send sync know problem"]
pub const ARC_WITH_NO_SEND_SYNC_KNOW_PROBLEM: &str = "";
#[doc = "The arc with non send sync whats bad"]
pub const ARC_WITH_NO_SEND_SYNC_WHATS_BAD: &str = "Arc<T> is a thread-safe Rc<T> and guarantees that updates to the reference counter use atomic operations. To send an Arc<T> across thread boundaries and share ownership between multiple threads, T must be both Send and Sync, so either T should be made Send + Sync or a Rc should be used instead of an Arc.";
#[doc = "The arc with non send sync issue uri"]
pub const ARC_WITH_NO_SEND_SYNC_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+arc_with_non_send_sync");

#[doc = "The arc with non send sync for novice"]
pub const NOVICE_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The arc with non send sync for expert"]
pub const EXPERT_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Suspicious,
    issue: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};

#[doc = "The arc with non send sync for master"]
pub const MASTER_ARC_WITH_NO_SEND_SYNC: ClippyLint = ClippyLint {
    id: ARC_WITH_NO_SEND_SYNC_ID,
    description: ARC_WITH_NO_SEND_SYNC_DESCRIPTION,
    whats_bad: ARC_WITH_NO_SEND_SYNC_WHATS_BAD,
    known_problems: None,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Warn,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ARC_WITH_NO_SEND_SYNC_ISSUE,
    applicability: Applicability::Unspecified,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
};
