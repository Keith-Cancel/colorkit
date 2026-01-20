const F32_BIAS: i32 = 127;
const F32_MSK_EXP: u32 = 0x7f800000;

#[inline]
const fn exponent(bits: u32) -> i8 {
    let e = ((bits >> 23) & 0xff) as i32 - F32_BIAS;
    return e as i8;
}

/// Mask the floats exponent and divide it by.
#[inline]
const fn exponent_div_5(bits: u32) -> u32 {
    let e = bits & F32_MSK_EXP;
    // ((e - 127) / 5) + 127 = (e / 5) + (508 / 5)
    let e = e / 5;
    // 508 / 5 ~= 0x65.999999 in a fixed point u32 with 24 bit fraction
    // Shift right 1 and it's 0x32cccccc
    // Neg
    // let e = e + 0x32cccccc + 0x666666 + 2;
    let e = e + 0x32cccccc + 1;
    return e >> 23;
}

/// Computes the quintic root or 5th root.
pub const fn quirt(x: f32) -> f32 {
    let bits = x.to_bits();
    let abs = bits & 0x7fffffff;

    // Either inifnity or NaN
    if abs >= F32_MSK_EXP {
        return x;
    }

    // TODO
    // for rough first guess
    // * divide f32 exponent by 5
    // * apply some linear/cheap approx to m^(1/5) of the mantissa.
    // * This is because a f32 is basiclly m * 2^k and (m * 2^k)^(1/5)
    //   is m^(1/5) * 2^(k/5)
    // Apply some number of rounds of Newton's or Halley's method
    todo!();
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

    //#[test]
    fn exp_div_5() {
        let mut v: f32 = 1.0;
        for _ in 0..128 {
            let bits = v.to_bits();
            let e_i = exponent(bits) as i32;
            let e_r1 = ((e_i / 5) + 127) as u32;
            let e_r2 = exponent_div_5(bits);
            assert_eq!(e_r1, e_r2);
            v *= 2.0;
        }

        // hmmm gonna go for a walk and think a little.
        v = 1.0;
        for _ in 0..127 {
            let bits = v.to_bits();
            //println!("{}", exponent(bits));
            let e_i = exponent(bits) as i32;
            let e_r1 = ((e_i / 5) + 127) as u32;
            //println!("\nr1: {} {:x}", (e_r1 as i32) - 127, e_r1 << 23);
            let e_r2 = exponent_div_5(bits);
            //println!("\nr2: {} {:x}", (e_r2 as i32) - 127, e_r2 << 23);
            assert_eq!(e_r1, e_r2);

            v *= 0.5;
        }
    }
}
