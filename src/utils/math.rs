const F32_BIAS: i32 = 127;

const fn exponent(bits: u32) -> i8 {
    let e = ((bits >> 23) & 0xff) as i32 - F32_BIAS;
    return e as i8;
}

/// Computes the quintic root or 5th root.
pub const fn quirt(x: f32) {
    let bits = x.to_bits();
    let e_i = exponent(bits);
    let e_r = (e_i / 5) + 127; // Hmm this is ((e - 127) / 5) + 127 can I do this more effectively?
    // TODO
    // for rough first guess
    // * divide f32 exponent by 5
    // * apply some linear/cheap approx to m^(1/5) of the mantissa.
    // * This is because a f32 is basiclly m * 2^k and (m * 2^k)^(1/5)
    //   is m^(1/5) * 2^(k/5)
    // Apply some number of rounds of Newton's or Halley's method
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
