use super::*;

//const NORM_ADD: u32 = root_const(127 * 5, 127, 5);
//const SUBNORM_ADD: u32 = root_const(127 * 5, 127 + 24, 5);
const NORM_ADD: u32 = root_const2(5, 0);
const SUBNORM_ADD: u32 = root_const2(5, 24);

/// Computes the quintic root or 5th root.
#[inline]
pub const fn quirtf(x: f32) -> f32 {
    let bits = x.to_bits();
    let neg = bits & 0x80000000;
    let abs = bits & 0x7fffffff;

    // Either infinity or NaN
    if abs >= F32_MSK_EXP {
        return x;
    }

    // This seems to work well and is faster.
    let mut q = abs; // Abs or mask?
    // Is the number zero or sub-normal
    if q < 0x00800000 {
        if q == 0 {
            return x;
        }
        // Essentially add 24 to the exponent
        q = (P24 * x).to_bits() & 0x7fffffff;
        q /= 5;
        q += SUBNORM_ADD;
    } else {
        q /= 5;
        q += NORM_ADD;
    }

    let a = x as f64;
    let mut x = f32::from_bits(neg | q) as f64;
    let mut i = 0;
    // Halley's method
    while i < 2 {
        // x^5
        let p = x * x;
        let p = p * p * x;

        let n1 = (2.0 / 3.0) * x;
        let n2 = ((2.5 / 3.0) * a * x) / (a + 1.5 * p);
        x = n1 + n2;
        i += 1;
    }

    // Newtons Method
    //let a = 0.2 * a;
    //while i < 4 {
    //    // x^4
    //    let p = x * x;
    //    let p = p * p;
    //
    //    x = 0.8 * x + (a / p);
    //    i += 1;
    //}
    return x as f32;
}
