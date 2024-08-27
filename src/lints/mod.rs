use crate::{clippy::absolute_path::NOVICE_ABSOLUTE_PATH_LINTS, config::Lint};

const LINTERS: usize = 1;

const NOVICE_ABSOLUTE_PATH: (Lint, Lint, Lint) = NOVICE_ABSOLUTE_PATH_LINTS;
#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [Lint; LINTERS] = [NOVICE_ABSOLUTE_PATH.0];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [Lint; LINTERS] = [NOVICE_ABSOLUTE_PATH.1];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [Lint; LINTERS] = [NOVICE_ABSOLUTE_PATH.2];
