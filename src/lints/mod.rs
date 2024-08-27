use crate::{
    clippy::{absolute_path::ABSOLUTE_PATH_LINTS, assigning_clones::ASSIGNING_CLONES_LINTS},
    config::Lint,
};

const LINTERS: usize = 2;

const ABSOLUTE_PATH: (Lint, Lint, Lint) = ABSOLUTE_PATH_LINTS;
const ASSIGNING_CLONES: (Lint, Lint, Lint) = ASSIGNING_CLONES_LINTS;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [Lint; LINTERS] = [ABSOLUTE_PATH.0, ASSIGNING_CLONES.0];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [Lint; LINTERS] = [ABSOLUTE_PATH.1, ASSIGNING_CLONES.1];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [Lint; LINTERS] = [ABSOLUTE_PATH.2, ASSIGNING_CLONES.2];
