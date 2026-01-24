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
// Got this idea of adding to the exponent by looking at some cbrt
// implementations to get back precision when working with
// a subnormal.
/// A float the value of 2^24
const P24: f32 = f32::from_bits(0x4b800000); // the exponent is 24

/// Compute the const to add the floating point number after dividing
/// for the intial guess.
///
/// For example if we divid by 5 for the 5th root.
/// x = (e - 127)/5 + 127
/// x = (e - 127)/5 + 635/5
/// x = e/5 - 127/5 + 635/5
/// x = e/5 + 508/5
/// 508 / 5 ~= 0x65.999999 in a fixed point u32 with 24 bit fraction
/// shift right 1 and it's then 0x32cccccc
/// and add 1 acount for the shifted off bit.
const fn root_const(minuend: u32, subtrahend: u32, divisor: u32) -> u32 {
    let dif = ((minuend - subtrahend) as u64) << 32;
    let frac = dif / divisor as u64;
    let frac = (frac >> 8) as u32;
    let rnd = frac & 1;
    return (frac >> 1) + rnd;
}
