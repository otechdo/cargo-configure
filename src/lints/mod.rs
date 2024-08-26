use crate::clippy::absolute_path::{EXPERT_ABSOLUTE_PATH, MASTER_ABSOLUTE_PATH, NOVICE_ABSOLUTE_PATH};
use crate::clippy::absurd_extreme_comparisons::{EXPERT_ABSURD_EXTREME_COMPARISON, MASTER_ABSURD_EXTREME_COMPARISON, NOVICE_ABSURD_EXTREME_COMPARISON};
use crate::clippy::alloc_instead_of_core::{EXPERT_ALLOC_INSTEAD_OF_CORE, MASTER_ALLOC_INSTEAD_OF_CORE, NOVICE_ALLOC_INSTEAD_OF_CORE};
use crate::clippy::allow_attributes::{EXPERT_ALLOW_ATTRIBUTE, MASTER_ALLOW_ATTRIBUTE, NOVICE_ALLOW_ATTRIBUTE};
use crate::clippy::allow_attributes_without_reason::{EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON, MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON, NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON};
use crate::clippy::almost_complete_range::{EXPERT_ALMOST_COMPLETE_RANGE, MASTER_ALMOST_COMPLETE_RANGE, NOVICE_ALMOST_COMPLETE_RANGE};
use crate::clippy::almost_swapped::{EXPERT_ALMOST_SWAPPED, MASTER_ALMOST_SWAPPED, NOVICE_ALMOST_SWAPPED};
use crate::clippy::approx_constant::{EXPERT_APPROX_CONSTANT, MASTER_APPROX_CONSTANT, NOVICE_APPROX_CONSTANT};
use crate::clippy::arc_with_non_send_sync::{EXPERT_ARC_WITH_NO_SEND_SYNC, MASTER_ARC_WITH_NO_SEND_SYNC, NOVICE_ARC_WITH_NO_SEND_SYNC};
use crate::clippy::arithmetic_side_effects::{EXPERT_ARITHMETIC_SIDE_EFFECT, MASTER_ARITHMETIC_SIDE_EFFECT, NOVICE_ARITHMETIC_SIDE_EFFECT};
use crate::clippy::as_conversions::{EXPERT_AS_CONVERSIONS, MASTER_AS_CONVERSIONS, NOVICE_AS_CONVERSIONS};
use crate::clippy::as_ptr_cast_mut::{EXPERT_AS_PTR_CAST_MUT, MASTER_AS_PTR_CAST_MUT, NOVICE_AS_PTR_CAST_MUT};
use crate::clippy::as_underscore::{EXPERT_AS_UNDERSCORE, MASTER_AS_UNDERSCORE, NOVICE_AS_UNDERSCORE};
use crate::clippy::assertions_on_constants::{EXPERT_ASSERTIONS_ON_CONSTANTS, MASTER_ASSERTIONS_ON_CONSTANTS, NOVICE_ASSERTIONS_ON_CONSTANTS};
use crate::clippy::assertions_on_result_states::{EXPERT_ASSERTIONS_ON_RESULTS, MASTER_ASSERTIONS_ON_RESULTS, NOVICE_ASSERTIONS_ON_RESULTS};
use crate::clippy::assign_op_pattern::{EXPERT_ASSIGN_OP_PATTERN, MASTER_ASSIGN_OP_PATTERN, NOVICE_ASSIGN_OP_PATTERN};
use crate::clippy::assigning_clones::{EXPERT_ASSIGNING_CLONE, MASTER_ASSIGNING_CLONE, NOVICE_ASSIGNING_CLONE};
use crate::clippy::async_yields_async::{EXPERT_ASYNC_YIELDS_ASYNC, MASTER_ASYNC_YIELDS_ASYNC, NOVICE_ASYNC_YIELDS_ASYNC};
use crate::clippy::await_holding_invalid_type::{EXPERT_AWAIT_HOLDING_INVALID, MASTER_AWAIT_HOLDING_INVALID, NOVICE_AWAIT_HOLDING_INVALID};
use crate::clippy::await_holding_lock::{EXPERT_AWAIT_HOLDING_LOCK, MASTER_AWAIT_HOLDING_LOCK, NOVICE_AWAIT_HOLDING_LOCK};
use crate::clippy::await_holding_refcell_ref::{EXPERT_AWAIT_HOLDING_REFCELL_REF, MASTER_AWAIT_HOLDING_REFCELL_REF, NOVICE_AWAIT_HOLDING_REFCELL_REF};
use crate::clippy::bad_bit_mask::{EXPERT_BAD_BIT_MASK, MASTER_BAD_BIT_MASK, NOVICE_BAD_BIT_MASK};
use crate::clippy::big_endian_bytes::{EXPERT_BIG_ENDIAN_BYTES, MASTER_BIG_ENDIAN_BYTES, NOVICE_BIG_ENDIAN_BYTES};
use crate::clippy::blanket_clippy_restriction_lints::{EXPERT_BLANKET_CLIPPY_RESTRICTION_LINTS, MASTER_BLANKET_CLIPPY_RESTRICTION_LINTS, NOVICE_BLANKET_CLIPPY_RESTRICTION_LINTS};
use crate::clippy::core::ClippyLint;

#[doc = "All lints for novice"]
pub const NOVICE_LINTS: [ClippyLint; 24] = [
    NOVICE_ABSOLUTE_PATH,
    NOVICE_ALLOC_INSTEAD_OF_CORE,
    NOVICE_ALLOW_ATTRIBUTE,
    NOVICE_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    NOVICE_ALMOST_COMPLETE_RANGE,
    NOVICE_ALMOST_SWAPPED,
    NOVICE_APPROX_CONSTANT,
    NOVICE_ARC_WITH_NO_SEND_SYNC,
    NOVICE_ARITHMETIC_SIDE_EFFECT,
    NOVICE_ABSURD_EXTREME_COMPARISON,
    NOVICE_AS_CONVERSIONS,
    NOVICE_AS_UNDERSCORE,
    NOVICE_ASSERTIONS_ON_RESULTS,
    NOVICE_AS_PTR_CAST_MUT,
    NOVICE_ASSERTIONS_ON_CONSTANTS,
    NOVICE_ASSIGN_OP_PATTERN,
    NOVICE_ASSIGNING_CLONE,
    NOVICE_ASYNC_YIELDS_ASYNC,
    NOVICE_AWAIT_HOLDING_INVALID,
    NOVICE_AWAIT_HOLDING_LOCK,
    NOVICE_AWAIT_HOLDING_REFCELL_REF,
    NOVICE_BAD_BIT_MASK,
    NOVICE_BIG_ENDIAN_BYTES,
    NOVICE_BLANKET_CLIPPY_RESTRICTION_LINTS,
];

#[doc = "All lints for expert"]
pub const EXPERT_LINTS: [ClippyLint; 24] = [
    EXPERT_ABSOLUTE_PATH,
    EXPERT_ALLOC_INSTEAD_OF_CORE,
    EXPERT_ALLOW_ATTRIBUTE,
    EXPERT_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    EXPERT_ALMOST_COMPLETE_RANGE,
    EXPERT_ALMOST_SWAPPED,
    EXPERT_APPROX_CONSTANT,
    EXPERT_ARC_WITH_NO_SEND_SYNC,
    EXPERT_ARITHMETIC_SIDE_EFFECT,
    EXPERT_ABSURD_EXTREME_COMPARISON,
    EXPERT_AS_CONVERSIONS,
    EXPERT_AS_UNDERSCORE,
    EXPERT_ASSERTIONS_ON_RESULTS,
    EXPERT_AS_PTR_CAST_MUT,
    EXPERT_ASSERTIONS_ON_CONSTANTS,
    EXPERT_ASSIGN_OP_PATTERN,
    EXPERT_ASSIGNING_CLONE,
    EXPERT_ASYNC_YIELDS_ASYNC,
    EXPERT_AWAIT_HOLDING_INVALID,
    EXPERT_AWAIT_HOLDING_LOCK,
    EXPERT_AWAIT_HOLDING_REFCELL_REF,
    EXPERT_BAD_BIT_MASK,
    EXPERT_BIG_ENDIAN_BYTES,
    EXPERT_BLANKET_CLIPPY_RESTRICTION_LINTS,
];

#[doc = "All lints for master"]
pub const MASTER_LINTS: [ClippyLint; 24] = [
    MASTER_ABSOLUTE_PATH,
    MASTER_ALLOC_INSTEAD_OF_CORE,
    MASTER_ALLOW_ATTRIBUTE,
    MASTER_ALLOW_ATTRIBUTE_WITHOUT_REASON,
    MASTER_ALMOST_COMPLETE_RANGE,
    MASTER_ALMOST_SWAPPED,
    MASTER_APPROX_CONSTANT,
    MASTER_ARC_WITH_NO_SEND_SYNC,
    MASTER_ARITHMETIC_SIDE_EFFECT,
    MASTER_ABSURD_EXTREME_COMPARISON,
    MASTER_AS_CONVERSIONS,
    MASTER_AS_UNDERSCORE,
    MASTER_ASSERTIONS_ON_RESULTS,
    MASTER_AS_PTR_CAST_MUT,
    MASTER_ASSERTIONS_ON_CONSTANTS,
    MASTER_ASSIGN_OP_PATTERN,
    MASTER_ASSIGNING_CLONE,
    MASTER_ASYNC_YIELDS_ASYNC,
    MASTER_AWAIT_HOLDING_INVALID,
    MASTER_AWAIT_HOLDING_LOCK,
    MASTER_AWAIT_HOLDING_REFCELL_REF,
    MASTER_BAD_BIT_MASK,
    MASTER_BIG_ENDIAN_BYTES,
    MASTER_BLANKET_CLIPPY_RESTRICTION_LINTS,
];
