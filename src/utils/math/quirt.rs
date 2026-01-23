const F32_BIAS: i32 = 127;
const F32_MSK_EXP: u32 = 0x7f800000;
//const F32_MSK_MAN: u32 = 0x007fffff;

#[allow(unused)]
#[inline]
const fn exponent(bits: u32) -> i8 {
    let e = ((bits >> 23) & 0xff) as i32 - F32_BIAS;
    return e as i8;
}

/// Computes the quintic root or 5th root.
#[inline]
pub const fn quirt_f32(x: f32) -> f32 {
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
        // Got this idea of adding to the exponent by looking at some cbrt
        // implementations to get back precision when working with
        // a subnormal.
        const P24: f32 = f32::from_bits(0x4b800000); // the exponent is 24
        // Essentially add 24 to the exponent
        q = (P24 * x).to_bits() & 0x7fffffff;
        // So we need to:
        // x = (e - 127 - 24)/5 + 127
        // x = (e - 127 - 24)/5 + 635/5
        // x = e/5 - 151/5 + 635/5
        // x = e/5 + 484/5
        // 484 / 5  ~= 0x60.cccccc in fix point u32 with 24 bit fraction
        // shift right 1 and its then 0x30666666
        // no bit shifted off so do not need to add 1
        q /= 5;
        q += 0x30666666;
    } else {
        // So we need to:
        // x = (e - 127)/5 + 127
        // x = (e - 127)/5 + 635/5
        // x = e/5 - 127/5 + 635/5
        // x = e/5 + 508/5
        // 508 / 5 ~= 0x65.999999 in a fixed point u32 with 24 bit fraction
        // shift right 1 and it's then 0x32cccccc
        // and add 1 acount for the shifted off bit.
        q /= 5;
        q += 0x32cccccc + 1;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_exponent() {
        let mut v: f32 = 1.0;
        assert_eq!(exponent(v.to_bits()), 0);

        for i in 1..=128i32 {
            v *= 2.0;
            assert_eq!(exponent(v.to_bits()), i as i8);
        }

        v = 1.0;
        for i in 1..=127 {
            v *= 0.5;
            assert_eq!(exponent(v.to_bits()), (-i));
        }
    }
}
