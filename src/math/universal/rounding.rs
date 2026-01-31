use super::exponentf;
/// Get the integer part of the float. Truncates the fraction
/// always to zero.
pub fn truncf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);
    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    let neg = bits & 0x80000000;
    // Purely fractional so will just be zero.
    if exp < 0 {
        return f32::from_bits(neg);
    }
    todo!();
    return x;
}

#[cfg(test)]
mod test {
    use super::*;
    fn bit_eq(a: f32, b: f32) -> bool {
        return a.to_bits() == b.to_bits();
    }

    #[test]
    #[rustfmt::skip]
    fn trunc_frac() {
        assert!(bit_eq(truncf( 0.0),    0.0));
        assert!(bit_eq(truncf(-0.0),   -0.0));
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
    fn trunc_large() {
        assert!(bit_eq(truncf(8388608.0), 8388608.0));
        assert!(bit_eq(truncf(-8388608.0), -8388608.0));
        assert!(bit_eq(truncf(16777216.0), 16777216.0));
        assert!(bit_eq(truncf(-16777216.0), -16777216.0));
        assert!(bit_eq(truncf(f32::INFINITY), f32::INFINITY));
        assert!(bit_eq(truncf(f32::NEG_INFINITY), f32::NEG_INFINITY));
    }
}
