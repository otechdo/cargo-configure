use crate::clippy::core::{Applicability, ClippyLint, LintGroup, LintSeverity};
#[doc = "The clippy alloc instead of core id"]
pub const ALLOC_INSTEAD_OF_CORE_ID: &str = "alloc_instead_of_core";
#[doc = "The clippy alloc instead of core description"]
pub const ALLOC_INSTEAD_OF_CORE_DESCRIPTION: &str =
    "Finds items imported through alloc when available through core.";
#[doc = "The clippy alloc instead of core know problem"]
pub const ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM: Option<&'static str> = Some("The lint is only partially aware of the required MSRV for items that were originally in std but moved to core.");
#[doc = "The clippy alloc instead of core what it's bad"]
pub const ALLOC_INSTEAD_OF_CORE_WHATS_BAD: &str = "Crates which have no_std compatibility and may optionally require alloc may wish to ensure types are imported from core to ensure disabling alloc does not cause the crate to fail to compile. This lint is also useful for crates migrating to become no_std compatible.";
#[doc = "The clippy alloc instead of core what it's bad uri issue"]
pub const ALLOC_INSTEAD_OF_CORE_ISSUE: Option<&'static str> = Some("https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+alloc_instead_of_core");

#[doc = "clippy alloc instead of core lint for novice"]
pub const NOVICE_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "clippy alloc instead of core lint for expert"]
pub const EXPERT_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Warn,
    group: LintGroup::Restriction,
    issue: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Warn),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Warn),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};

#[doc = "clippy alloc instead of core lint for master"]
pub const MASTER_ALLOC_INSTEAD_OF_CORE: ClippyLint = ClippyLint {
    id: ALLOC_INSTEAD_OF_CORE_ID,
    description: ALLOC_INSTEAD_OF_CORE_DESCRIPTION,
    whats_bad: ALLOC_INSTEAD_OF_CORE_WHATS_BAD,
    known_problems: ALLOC_INSTEAD_OF_CORE_KNOW_PROBLEM,
    enabled_by_default: true,
    default_clippy_severity: LintSeverity::Allow,
    use_clippy_severity: false,
    severity: LintSeverity::Deny,
    group: LintGroup::Restriction,
    issue: ALLOC_INSTEAD_OF_CORE_ISSUE,
    applicability: Applicability::MachineApplicable,
    all_increase_config_default_possible_severity: LintSeverity::Increase(&LintSeverity::Deny),
    all_decrease_config_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Deny),
    all_increase_clippy_default_possible_severity: LintSeverity::Increase(&LintSeverity::Allow),
    all_decrease_clippy_default_possible_severity: LintSeverity::Decrease(&LintSeverity::Allow),
};