use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The async yields async id"]
pub const ASYNC_YIELDS_ASYNC_ID: &str = "async_yields_async";
#[doc = "The async yields async description"]
pub const ASYNC_YIELDS_ASYNC_DESCRIPTION: &str = "Checks for async blocks that yield values of types that can themselves be awaited.";
#[doc = "The async yields async know problem"]
pub const ASYNC_YIELDS_ASYNC_KNOW_PROBLEM: Option<&'static str> = Some("While forbidden by the spec, OpAssign traits may have implementations that differ from the regular Op impl.");
#[doc = "The async yields async what it's bad"]
pub const ASYNC_YIELDS_ASYNC_WHATS_BAD: &str = "An await is likely missing.";
#[doc = "The async yields async uri issue"]
pub const ASYNC_YIELDS_ASYNC_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+async_yields_async");

#[doc = "The async yields async for novice"]
pub const NOVICE_ASYNC_YIELDS_ASYNC: ClippyLint = ClippyLint {
    id: ASYNC_YIELDS_ASYNC_ID,
    description: ASYNC_YIELDS_ASYNC_DESCRIPTION,
    whats_bad: ASYNC_YIELDS_ASYNC_WHATS_BAD,
    known_problems: ASYNC_YIELDS_ASYNC_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Correctness,
    issue: ASYNC_YIELDS_ASYNC_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The async yields async for expert"]
pub const EXPERT_ASYNC_YIELDS_ASYNC: ClippyLint = ClippyLint {
    id: ASYNC_YIELDS_ASYNC_ID,
    description: ASYNC_YIELDS_ASYNC_DESCRIPTION,
    whats_bad: ASYNC_YIELDS_ASYNC_WHATS_BAD,
    known_problems: ASYNC_YIELDS_ASYNC_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Correctness,
    issue: ASYNC_YIELDS_ASYNC_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};

#[doc = "The async yields async for master"]
pub const MASTER_ASYNC_YIELDS_ASYNC: ClippyLint = ClippyLint {
    id: ASYNC_YIELDS_ASYNC_ID,
    description: ASYNC_YIELDS_ASYNC_DESCRIPTION,
    whats_bad: ASYNC_YIELDS_ASYNC_WHATS_BAD,
    known_problems: ASYNC_YIELDS_ASYNC_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Deny,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Correctness,
    issue: ASYNC_YIELDS_ASYNC_ISSUE,
    applicability: Applicability::MaybeIncorrect,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
};
