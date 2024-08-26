use crate::clippy::absolute_path::{
    EXPERT_ABSOLUTE_PATH_LINT, MASTER_ABSOLUTE_PATH_LINT, NOVICE_ABSOLUTE_PATH_LINT,
};
use crate::clippy::alloc_instead_of_core::{
    EXPERT_ALLOW_INS_OF_CORE_LINT, MASTER_ALLOW_INS_OF_CORE_LINT, NOVICE_ALLOW_INS_OF_CORE_LINT,
};
use crate::config::ClippyLint;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [ClippyLint; 2] =
    [NOVICE_ABSOLUTE_PATH_LINT, NOVICE_ALLOW_INS_OF_CORE_LINT];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [ClippyLint; 2] =
    [EXPERT_ABSOLUTE_PATH_LINT, EXPERT_ALLOW_INS_OF_CORE_LINT];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [ClippyLint; 2] =
    [MASTER_ABSOLUTE_PATH_LINT, MASTER_ALLOW_INS_OF_CORE_LINT];
