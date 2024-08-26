use crate::clippy::absolute_path::{
    EXPERT_ABSOLUTE_PATH_LINT, MASTER_ABSOLUTE_PATH_LINT, NOVICE_ABSOLUTE_PATH_LINT,
};
use crate::clippy::absurd_extreme_comparisons::{
    EXPERT_ABSURD_EXTREME_COMPARISON_LINT, MASTER_ABSURD_EXTREME_COMPARISON_LINT,
    NOVICE_ABSURD_EXTREME_COMPARISON_LINT,
};
use crate::clippy::alloc_instead_of_core::{
    EXPERT_ALLOW_INS_OF_CORE_LINT, MASTER_ALLOW_INS_OF_CORE_LINT, NOVICE_ALLOW_INS_OF_CORE_LINT,
};
use crate::clippy::allow_attributes::{
    EXPERT_ALLOW_ATTRIBUTE_LINT, MASTER_ALLOW_ATTRIBUTE_LINT, NOVICE_ALLOW_ATTRIBUTE_LINT,
};
use crate::clippy::allow_attributes_without_reason::{
    EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT, MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT,
    NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT,
};
use crate::clippy::almost_complete_range::{
    EXPERT_ALMOST_COMPLETE_RANGE_LINT, MASTER_ALMOST_COMPLETE_RANGE_LINT,
    NOVICE_ALMOST_COMPLETE_RANGE_LINT,
};
use crate::clippy::approx_constant::{
    EXPERT_APPROX_CONSTANT_LINT, MASTER_APPROX_CONSTANT_LINT, NOVICE_APPROX_CONSTANT_LINT,
};
use crate::clippy::arc_with_non_send_sync::{
    EXPERT_ARC_WITH_NO_SEND_SYNC_LINT, MASTER_ARC_WITH_NO_SEND_SYNC_LINT,
    NOVICE_ARC_WITH_NO_SEND_SYNC_LINT,
};
use crate::clippy::arithmetic_side_effects::{
    EXPERT_ARTHMETIC_SIDE_EFFECTS_LINT, MASTER_ARTHMETIC_SIDE_EFFECTS_LINT,
    NOVICE_ARTHMETIC_SIDE_EFFECTS_LINT,
};
use crate::clippy::as_conversions::{
    EXPERT_AS_CONVERSIONS_LINT, MASTER_AS_CONVERSIONS_LINT, NOVICE_AS_CONVERSIONS_LINT,
};
use crate::clippy::as_underscore::{
    EXPERT_AS_UNDERSCORE_LINT, MASTER_AS_UNDERSCORE_LINT, NOVICE_AS_UNDERSCORE_LINT,
};
use crate::config::ClippyLint;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [ClippyLint; 11] = [
    NOVICE_ABSOLUTE_PATH_LINT,
    NOVICE_ALLOW_INS_OF_CORE_LINT,
    NOVICE_ABSURD_EXTREME_COMPARISON_LINT,
    NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT,
    NOVICE_ALLOW_ATTRIBUTE_LINT,
    NOVICE_ALMOST_COMPLETE_RANGE_LINT,
    NOVICE_APPROX_CONSTANT_LINT,
    NOVICE_ARC_WITH_NO_SEND_SYNC_LINT,
    NOVICE_ARTHMETIC_SIDE_EFFECTS_LINT,
    NOVICE_AS_CONVERSIONS_LINT,
    NOVICE_AS_UNDERSCORE_LINT,
];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [ClippyLint; 11] = [
    EXPERT_ABSOLUTE_PATH_LINT,
    EXPERT_ALLOW_INS_OF_CORE_LINT,
    EXPERT_ABSURD_EXTREME_COMPARISON_LINT,
    EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT,
    EXPERT_ALLOW_ATTRIBUTE_LINT,
    EXPERT_ALMOST_COMPLETE_RANGE_LINT,
    EXPERT_APPROX_CONSTANT_LINT,
    EXPERT_ARC_WITH_NO_SEND_SYNC_LINT,
    EXPERT_ARTHMETIC_SIDE_EFFECTS_LINT,
    EXPERT_AS_CONVERSIONS_LINT,
    EXPERT_AS_UNDERSCORE_LINT,
];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [ClippyLint; 11] = [
    MASTER_ABSOLUTE_PATH_LINT,
    MASTER_ALLOW_INS_OF_CORE_LINT,
    MASTER_ABSURD_EXTREME_COMPARISON_LINT,
    MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON_LINT,
    MASTER_ALLOW_ATTRIBUTE_LINT,
    MASTER_ALMOST_COMPLETE_RANGE_LINT,
    MASTER_APPROX_CONSTANT_LINT,
    MASTER_ARC_WITH_NO_SEND_SYNC_LINT,
    MASTER_ARTHMETIC_SIDE_EFFECTS_LINT,
    MASTER_AS_CONVERSIONS_LINT,
    MASTER_AS_UNDERSCORE_LINT,
];
