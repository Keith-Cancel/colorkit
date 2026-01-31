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
    let neg = bits & 0x80000000;
    // Purely fractional so will just be zero or -1
    if exp < 0 {
        // preserve zero
        if (bits << 1) == 0 {
            return x;
        }
        // Minus one or zero.
        let ret = if neg > 0 { 0xbf800000 } else { 0 };
        return f32::from_bits(ret);
    }

    let msk = ((0xff800000u32 as i32) >> exp) as u32;
    let new = bits & msk;

    // If negative and there was a fractional part, subtract 1.0.
    if neg > 0 && bits != new {
        let add = 1u32 << (23 - exp);
        // This correctly carries into he exponent if necessary
        return f32::from_bits(new.wrapping_add(add));
    }
    return f32::from_bits(new);
}

/// Rounds the integer greater than or equal to the provided value.
///
/// Similar to [`truncf`], but instead of torwards zero, it's
/// torwards positive infinity.
pub const fn ceilf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);

    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    let neg = bits & 0x80000000;
    // Purely fractional so will just be zero or one
    if exp < 0 {
        // preserve zero
        if (bits << 1) == 0 {
            return x;
        }
        // One or negative zero.
        let ret = if neg > 0 { neg } else { 0x3f800000 };
        return f32::from_bits(ret);
    }
    let msk = ((0xff800000u32 as i32) >> exp) as u32;
    let new = bits & msk;

    // If not negative and there was a fractional part, add 1.0.
    if neg == 0 && bits != new {
        let add = 1u32 << (23 - exp);
        // This correctly carries into he exponent if necessary
        return f32::from_bits(new.wrapping_add(add));
    }
    return f32::from_bits(new);
}

/// Rounds to the nearest integer to the provided value.
///
/// In the event the value is exactly in the middle it
/// will round to the nearest even integer.
pub const fn round_ties_evenf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);

    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    todo!();
    let neg = bits & 0x80000000;
    // Purely fractional so will just be zero or one
    if exp < 0 {
        // preserve zero
        if (bits << 1) == 0 {
            return x;
        }
        // One or negative zero.
        let ret = if neg > 0 { neg } else { 0x3f800000 };
        return f32::from_bits(ret);
    }
    let msk = ((0xff800000u32 as i32) >> exp) as u32;
    let new = bits & msk;

    // If not negative and there was a fractional part, add 1.0.
    if neg == 0 && bits != new {
        let add = 1u32 << (23 - exp);
        // This correctly carries into he exponent if necessary
        return f32::from_bits(new.wrapping_add(add));
    }
    return f32::from_bits(new);
}
