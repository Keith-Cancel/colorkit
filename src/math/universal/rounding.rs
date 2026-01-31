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

/// Rounds the integer less than or equal the provided value.
///
/// Similar to [`truncf`], but instead of torwards zero, it's
/// torwards negative infinity.
pub const fn floorf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);

    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    todo!();
}
