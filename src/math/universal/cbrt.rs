use super::*;

const NORM_ADD: u32 = root_const(127 * 3, 127, 3);
const SUBNORM_ADD: u32 = root_const(127 * 3, 127 + 24, 3);

/// Computes the cube root or 3rd root.
#[inline]
pub const fn cbrtf(x: f32) -> f32 {
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
        q /= 3;
        q += SUBNORM_ADD;
    } else {
        q /= 3;
        q += NORM_ADD;
    }

    let a = x as f64;
    let mut x = f32::from_bits(neg | q) as f64;
    let mut i = 0;
    // Halley's method
    let a = (1.0 / 2.0) * a;
    while i < 2 {
        // x^3
        let p = x * x;
        let p = p * x;

        let n1 = (1.0 / 2.0) * x;
        let n2 = ((3.0 / 2.0) * a * x) / (a + p);
        x = n1 + n2;
        i += 1;
    }
    return x as f32;
}
