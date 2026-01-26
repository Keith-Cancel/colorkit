mod cbrt;
mod quirt;
mod sqrt;
mod ulp;

pub use cbrt::cbrtf;
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
/// * x = (e - 127)/5 + 127
/// * x = (e - 127)/5 + 635/5
/// * x = e/5 - 127/5 + 635/5
/// * x = e/5 + 508/5
///
/// 508 / 5 ~= `0x65.999999` in a fixed point u32 with 24 bit fraction.
/// Then shift right 1 and it's then `0x32cccccc`.
/// Then add 1 acount for the shifted off bit.
#[allow(unused)]
const fn root_const(minuend: u32, subtrahend: u32, divisor: u32) -> u32 {
    let dif = ((minuend - subtrahend) as u64) << 32;
    let frac = dif / divisor as u64;
    let frac = (frac >> 8) as u32;
    let rnd = frac & 1;
    return (frac >> 1) + rnd;
}

/// `n` is what root we are computing the constant for.
const fn root_const2(n: u32, shift: u32) -> u32 {
    const BIAS: u64 = 127;
    const ONE: u64 = 1 << 32;
    const LN_2: u64 = 2_977_044_472; // ln(2) * 2^32

    // Approximate 2^(1/n) can be aproximated with a Taylor series:
    // 1 + ln(2)/n + ln^2(2)/2n^2 + ln^3(2)/6n^3 + ln^4(2)/24n^4 + ...
    let x1 = LN_2 / n as u64;
    let x2 = (x1 * x1) >> 32;
    let x3 = (x2 * x1) >> 32;
    let x4 = (x2 * x2) >> 32;
    let x5 = (x3 * x2) >> 32;
    let x6 = (x3 * x3) >> 32;
    let x7 = (x4 * x3) >> 32;
    let r1 = ONE;
    let r2 = r1 + x1;
    let r3 = r2 + (x2 / 2);
    let r4 = r3 + (x3 / 6);
    let r5 = r4 + (x4 / 24);
    let r6 = r5 + (x5 / 120);
    let r7 = r6 + (x6 / 720);
    let r8 = r7 + (x7 / 5040);

    // A float is 2^pow * (1 + m) where the mantissa value is [1 to 2)
    // Since we divide the whole float by the root
    // We get 2^(pow/n) * (1 + m/n)
    // The mantissa should be (1 + m)^(1/n)
    // So lets compute the error and divide the error by n
    // The error will be worst at like 1.999... so just treat it as 2.
    let m = ONE + (ONE / n as u64);
    let d = m - r8;
    let e = (d << 32) / r8;
    // Take the error at m = 0 and m ~= 1 and average, error at 0 is zero
    // so just divide by 2
    let e = e / 2;

    let b = BIAS * (n as u64) - BIAS - (shift as u64);
    let p = (b * ONE) / (n as u64);
    let c = p - e;

    return (c >> 9) as u32;
}
