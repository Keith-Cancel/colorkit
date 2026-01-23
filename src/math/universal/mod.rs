mod quirt;
mod sqrt;
mod ulp;

pub use quirt::quirtf;
pub use sqrt::sqrtf;
pub use ulp::ulp_int_diff_f32;

/// The mask to get the floating point biased exponent.
pub const F32_MSK_EXP: u32 = 0x7f800000;
/// The bias amount for an f32 exponent.
pub const F32_BIAS: i32 = 127;
