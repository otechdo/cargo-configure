use crate::{
    config::Applicability,
    config::ClippyLevel,
    config::ClippyLevel::{Expert, Master, Novice},
    config::ClippyLint,
    config::LintGroup,
    config::LintSeverity,
};
#[doc = "The lint id"]
const ID: &str = "rc_clone_in_vec_init";

#[doc = "The description of the lint"]
const DESCRIPTION: &str =
    "Checks for reference-counted pointers (Arc, Rc, rc::Weak, and sync::Weak) in vec![elem; len]";

#[doc = "The default clippy lint group"]
const GROUP: LintGroup = LintGroup::Suspicious;

#[doc = "The applicability group"]
const APPLICABILITY: Applicability = Applicability::HasPlaceholders;
#[doc = "The recommended clippy lint severity"]
const RECOMMENDED_SEVERITY: LintSeverity = LintSeverity::Allow;

#[doc = "The recommended increase clippy lint severity"]
const RECOMMENDED_INCREASE_SEVERITY: LintSeverity = LintSeverity::Increase(&RECOMMENDED_SEVERITY);
#[doc = "The recommended decrease clippy lint severity"]
const RECOMMENDED_DECREASE_SEVERITY: LintSeverity = LintSeverity::Decrease(&RECOMMENDED_SEVERITY);

#[doc = "The recommended lint severity for novice"]
const RECOMMENDED_SEVERITY_FOR_NOVICE: LintSeverity = LintSeverity::Deny;
#[doc = "The recommended lint severity for expert"]
const RECOMMENDED_SEVERITY_FOR_EXPERT: LintSeverity = LintSeverity::Deny;
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
pub const NOVICE_RC_CLONE_IN_VEC_INIT_LINT: ClippyLint = lint(&Novice);

#[doc = "The expert lint"]
pub const EXPERT_RC_CLONE_IN_VEC_INIT_LINT: ClippyLint = lint(&Expert);

#[doc = "The master lint"]
pub const MASTER_RC_CLONE_IN_VEC_INIT_LINT: ClippyLint = lint(&Master);
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
