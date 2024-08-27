use crate::{
    clippy::{
        ABSOLUTE_PATH_LINTS, ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS, ALMOST_COMPLETE_RANGE_LINTS,
        ALMOST_SWAPED_LINTS, APPROX_CONSTANT_LINTS, ARC_WITH_NO_SEND_LINTS,
        ARITMETIC_SIDE_EFFECTS_LINTS, ASSIGNING_CLONES_LINTS, AS_CONVERSIONS_LINTS,
        AS_UNDERSCORE_LINTS, BLOCKS_IN_CONDITIONS_LINTS,
    },
    config::Lint,
};

const LINTERS: usize = 11;
const ABSOLUTE_PATH: (Lint, Lint, Lint) = ABSOLUTE_PATH_LINTS;
const ASSIGNING_CLONES: (Lint, Lint, Lint) = ASSIGNING_CLONES_LINTS;
const BLOCKS_IN_CONDITIONS: (Lint, Lint, Lint) = BLOCKS_IN_CONDITIONS_LINTS;
const ALLOW_ATTRIBUTES_WITHOUT_REASON: (Lint, Lint, Lint) = ALLOW_ATTRIBUTES_WITHOUT_REASON_LINTS;
const ALMOST_COMPLETE_RANGE_LINT: (Lint, Lint, Lint) = ALMOST_COMPLETE_RANGE_LINTS;
const ALMOST_SWAPED_LINT: (Lint, Lint, Lint) = ALMOST_SWAPED_LINTS;
const APPROX_CONSTANT_LINT: (Lint, Lint, Lint) = APPROX_CONSTANT_LINTS;
const ARC_WITH_NO_SEND_LINT: (Lint, Lint, Lint) = ARC_WITH_NO_SEND_LINTS;
const ARITMETIC_SIDE_EFFECTS_LINT: (Lint, Lint, Lint) = ARITMETIC_SIDE_EFFECTS_LINTS;
const AS_UNDERSCORE_LINT: (Lint, Lint, Lint) = AS_UNDERSCORE_LINTS;
const AS_CONVERSIONS_LINT: (Lint, Lint, Lint) = AS_CONVERSIONS_LINTS;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.0,
    ASSIGNING_CLONES.0,
    BLOCKS_IN_CONDITIONS.0,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.0,
    ALMOST_COMPLETE_RANGE_LINT.0,
    ALMOST_SWAPED_LINT.0,
    APPROX_CONSTANT_LINT.0,
    ARC_WITH_NO_SEND_LINT.0,
    ARITMETIC_SIDE_EFFECTS_LINT.0,
    AS_UNDERSCORE_LINT.0,
    AS_CONVERSIONS_LINT.0,
];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.1,
    ASSIGNING_CLONES.1,
    BLOCKS_IN_CONDITIONS.1,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.1,
    ALMOST_COMPLETE_RANGE_LINT.1,
    ALMOST_SWAPED_LINT.1,
    APPROX_CONSTANT_LINT.1,
    ARC_WITH_NO_SEND_LINT.1,
    ARITMETIC_SIDE_EFFECTS_LINT.1,
    AS_UNDERSCORE_LINT.1,
    AS_CONVERSIONS_LINT.1,
];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [Lint; LINTERS] = [
    ABSOLUTE_PATH.2,
    ASSIGNING_CLONES.2,
    BLOCKS_IN_CONDITIONS.2,
    ALLOW_ATTRIBUTES_WITHOUT_REASON.2,
    ALMOST_COMPLETE_RANGE_LINT.2,
    ALMOST_SWAPED_LINT.2,
    APPROX_CONSTANT_LINT.2,
    ARC_WITH_NO_SEND_LINT.2,
    ARITMETIC_SIDE_EFFECTS_LINT.2,
    AS_UNDERSCORE_LINT.2,
    AS_CONVERSIONS_LINT.2,
];
