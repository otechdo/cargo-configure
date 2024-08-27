use crate::{
    config::Applicability,
    config::ClippyLevel,
    config::ClippyLevel::{Expert, Master, Novice},
    config::ClippyLint,
    config::LintGroup,
    config::LintSeverity,
};
#[doc = "The lint id"]
const ID: &str = "assigning_clones";

#[doc = "The description of the lint"]
const DESCRIPTION: &str = "Checks for code like foo = bar.clone();";

#[doc = "The default clippy lint group"]
const GROUP: LintGroup = LintGroup::Pedantic;

#[doc = "The applicability group"]
const APPLICABILITY: Applicability = Applicability::Unspecified;
#[doc = "The recommended clippy lint severity"]
const RECOMMENDED_SEVERITY: LintSeverity = LintSeverity::Allow;

#[doc = "The recommended increase clippy lint severity"]
const RECOMMENDED_INCREASE_SEVERITY: LintSeverity = LintSeverity::Increase(&RECOMMENDED_SEVERITY);
#[doc = "The recommended decrease clippy lint severity"]
const RECOMMENDED_DECREASE_SEVERITY: LintSeverity = LintSeverity::Decrease(&RECOMMENDED_SEVERITY);

#[doc = "The recommended lint severity for novice"]
const RECOMMENDED_SEVERITY_FOR_NOVICE: LintSeverity = LintSeverity::Warn;
#[doc = "The recommended lint severity for expert"]
const RECOMMENDED_SEVERITY_FOR_EXPERT: LintSeverity = LintSeverity::Warn;
#[doc = "The recommended lint severity for master"]
const RECOMMENDED_SEVERITY_FOR_MASTER: LintSeverity = LintSeverity::Deny;

#[doc = "The recommended lint increase severity for novice"]
const RECOMMENDED_INCREASE_SEVERITY_FOR_NOVICE: LintSeverity =
    LintSeverity::Increase(&RECOMMENDED_SEVERITY_FOR_NOVICE);

#[doc = "The recommended lint decrease severity for novice"]
const RECOMMENDED_DECREASE_SEVERITY_FOR_NOVICE: LintSeverity =
    LintSeverity::Decrease(&RECOMMENDED_SEVERITY_FOR_NOVICE);

#[doc = "The recommended lint increase severity for expert"]
const RECOMMENDED_INCREASE_SEVERITY_FOR_EXPERT: LintSeverity =
    LintSeverity::Increase(&RECOMMENDED_SEVERITY_FOR_EXPERT);

#[doc = "The recommended lint decrease severity for expert"]
const RECOMMENDED_DECREASE_SEVERITY_FOR_EXPERT: LintSeverity =
    LintSeverity::Decrease(&RECOMMENDED_SEVERITY_FOR_EXPERT);

#[doc = "The recommended lint increase severity for master"]
const RECOMMENDED_INCREASE_SEVERITY_FOR_MASTER: LintSeverity =
    LintSeverity::Increase(&RECOMMENDED_SEVERITY_FOR_MASTER);

#[doc = "The recommended lint decrease severity for master"]
const RECOMMENDED_DECREASE_SEVERITY_FOR_MASTER: LintSeverity =
    LintSeverity::Decrease(&RECOMMENDED_SEVERITY_FOR_MASTER);

#[doc = "The novice lint"]
pub const NOVICE_ASSIGN_CLONE_LINT: ClippyLint = lint(&Novice);

#[doc = "The expert lint"]
pub const EXPERT_ASSIGN_CLONE_LINT: ClippyLint = lint(&Expert);

#[doc = "The master lint"]
pub const MASTER_ASSIGN_CLONE_LINT: ClippyLint = lint(&Master);
#[doc = "save clippy lint conf"]
const fn lint(level: &ClippyLevel) -> ClippyLint {
    match level {
        Novice => ClippyLint {
            id: ID,
            description: DESCRIPTION,
            severity_by_clippy: RECOMMENDED_SEVERITY,
            severity_by_config: RECOMMENDED_SEVERITY_FOR_NOVICE,
            group: GROUP,
            applicability: APPLICABILITY,
            increase_config: RECOMMENDED_INCREASE_SEVERITY_FOR_NOVICE,
            decrease_config: RECOMMENDED_DECREASE_SEVERITY_FOR_NOVICE,
            increase_clippy: RECOMMENDED_INCREASE_SEVERITY,
            decrease_clippy: RECOMMENDED_DECREASE_SEVERITY,
        },
        Expert => ClippyLint {
            id: ID,
            description: DESCRIPTION,
            severity_by_clippy: RECOMMENDED_SEVERITY,
            severity_by_config: RECOMMENDED_SEVERITY_FOR_EXPERT,
            group: GROUP,
            applicability: APPLICABILITY,
            increase_config: RECOMMENDED_INCREASE_SEVERITY_FOR_EXPERT,
            decrease_config: RECOMMENDED_DECREASE_SEVERITY_FOR_EXPERT,
            increase_clippy: RECOMMENDED_INCREASE_SEVERITY,
            decrease_clippy: RECOMMENDED_DECREASE_SEVERITY,
        },
        Master => ClippyLint {
            id: ID,
            description: DESCRIPTION,
            severity_by_clippy: RECOMMENDED_SEVERITY,
            severity_by_config: RECOMMENDED_SEVERITY_FOR_MASTER,
            group: GROUP,
            applicability: APPLICABILITY,
            increase_config: RECOMMENDED_INCREASE_SEVERITY_FOR_MASTER,
            decrease_config: RECOMMENDED_DECREASE_SEVERITY_FOR_MASTER,
            increase_clippy: RECOMMENDED_INCREASE_SEVERITY,
            decrease_clippy: RECOMMENDED_DECREASE_SEVERITY,
        },
    }
}
