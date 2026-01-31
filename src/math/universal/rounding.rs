use super::exponentf;
/// Get the integer part of the float. Truncates the fraction always to zero.
pub const fn truncf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);
    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    // Purely fractional so will just be zero.
    if exp < 0 {
        // Only keep the sign.
        return f32::from_bits(bits & 0x80000000);
    }
    let msk = ((0xff800000u32 as i32) >> exp) as u32;
    // Otherwise just mask the fractional part out.
    return f32::from_bits(bits & msk);
}

#[cfg(test)]
mod test {
    use core::f32::consts::PI;

    use super::*;
    fn bit_eq(a: f32, b: f32) -> bool {
        return a.to_bits() == b.to_bits();
    }

    #[test]
    #[rustfmt::skip]
    fn trunc_frac() {
        assert!(bit_eq(truncf( 0.0),    0.0));
        assert!(bit_eq(truncf(-0.0),   -0.0));
        assert!(bit_eq(truncf( 0.1),   0.0));
        assert!(bit_eq(truncf(-0.1),  -0.0));
        assert!(bit_eq(truncf( 0.75),   0.0));
        assert!(bit_eq(truncf(-0.75),  -0.0));
        assert!(bit_eq(truncf( 0.5),    0.0));
        assert!(bit_eq(truncf(-0.5),   -0.0));
        assert!(bit_eq(truncf( 0.25),   0.0));
        assert!(bit_eq(truncf(-0.25),  -0.0));
        assert!(bit_eq(truncf(2e-32),   0.0));
        assert!(bit_eq(truncf(-2e-32), -0.0));
        assert!(bit_eq(truncf(2e-45),   0.0));
        assert!(bit_eq(truncf(-2e-45), -0.0));
    }
    #[test]
    #[rustfmt::skip]
    fn trunc_mixed() {
        assert!(bit_eq(truncf( 1.1),   1.0));
        assert!(bit_eq(truncf(-1.1),  -1.0));
        assert!(bit_eq(truncf( 1.25),  1.0));
        assert!(bit_eq(truncf(-1.25), -1.0));
        assert!(bit_eq(truncf( 1.9),   1.0));
        assert!(bit_eq(truncf(-1.9),  -1.0));
        assert!(bit_eq(truncf(PI),     3.0));
        assert!(bit_eq(truncf(8.125),  8.0));

        assert!(bit_eq(truncf( 4194303.25), 4194303.0));
        assert!(bit_eq(truncf(-4194304.5), -4194304.0));
        assert!(bit_eq(truncf( 8388607.5), 8388607.0));
    }
    #[test]
    fn trunc_large() {
        assert!(bit_eq(truncf(8388608.0), 8388608.0));
        assert!(bit_eq(truncf(-8388608.0), -8388608.0));
        assert!(bit_eq(truncf(16777216.0), 16777216.0));
        assert!(bit_eq(truncf(-16777216.0), -16777216.0));
        assert!(bit_eq(truncf(f32::INFINITY), f32::INFINITY));
        assert!(bit_eq(truncf(f32::NEG_INFINITY), f32::NEG_INFINITY));
    }
}
