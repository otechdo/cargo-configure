use crate::{
    clippy::{
        ABSOLUTE_PATH_LINTS, ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS, ALMOST_COMPLETE_RANGE_LINTS,
        ALMOST_SWAPED_LINTS, ASSIGNING_CLONES_LINTS, BLOCKS_IN_CONDITIONS_LINTS,
    },
    config::Lint,
};

const LINTERS: usize = 6;

const ABSOLUTE_PATH: (Lint, Lint, Lint) = ABSOLUTE_PATH_LINTS;
const ASSIGNING_CLONES: (Lint, Lint, Lint) = ASSIGNING_CLONES_LINTS;
const BLOCKS_IN_CONDITIONS: (Lint, Lint, Lint) = BLOCKS_IN_CONDITIONS_LINTS;
const ALLOW_ATTRIBUTES_WITHOUT_REASON: (Lint, Lint, Lint) = ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS;
const ALMOST_COMPLETE_RANGE_LINT: (Lint, Lint, Lint) = ALMOST_COMPLETE_RANGE_LINTS;
const ALMOST_SWAPED_LINT: (Lint, Lint, Lint) = ALMOST_SWAPED_LINTS;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.0,
    ASSIGNING_CLONES.0,
    BLOCKS_IN_CONDITIONS.0,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.0,
    ALMOST_COMPLETE_RANGE_LINT.0,
    ALMOST_SWAPED_LINT.0,
];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.1,
    ASSIGNING_CLONES.1,
    BLOCKS_IN_CONDITIONS.1,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.1,
    ALMOST_COMPLETE_RANGE_LINT.1,
    ALMOST_SWAPED_LINT.1,
];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.2,
    ASSIGNING_CLONES.2,
    BLOCKS_IN_CONDITIONS.2,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.2,
    ALMOST_COMPLETE_RANGE_LINT.2,
    ALMOST_SWAPED_LINT.2,
];
