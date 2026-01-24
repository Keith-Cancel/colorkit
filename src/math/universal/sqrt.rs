use super::*;

const NORM_ADD: u32 = root_const(127 * 2, 127, 2);
const SUBNORM_ADD: u32 = root_const(127 * 2, 127 + 24, 2);

/// Computes the square root
#[inline]
pub const fn sqrtf(x: f32) -> f32 {
    // Copying the logic over from my quirtf()
    // There might be a faster way but this works,
    // since I am sure lot of effort out there has
    // been put into at sqrt() that I could reference.

    let bits = x.to_bits();
    let neg = bits & 0x80000000;
    let abs = bits & 0x7fffffff;

    // Either inifnity or NaN
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
        q /= 2;
        q += SUBNORM_ADD;
    } else {
        q /= 2;
        q += NORM_ADD;
    }

    let a = x as f64;
    let mut x = f32::from_bits(neg | q) as f64;
    let mut i = 0;
    // Halley's method
    let a = a * (1.0 / 3.0);
    while i < 2 {
        let p2 = x * x;
        let p3 = p2 * x;

        let n1 = 3.0 * x;
        let n2 = ((8.0 / 3.0) * p3) / (a + p2);
        x = n1 - n2;
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
