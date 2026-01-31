use super::F32_MSK_ABS;
use super::F32_MSK_SIGN;
use super::exponentf;

const HALF: u32 = (0.5f32).to_bits();
const POS_ONE: u32 = (1.0f32).to_bits();
const NEG_ONE: u32 = (-1.0f32).to_bits();
const SHIFT_MSK: i32 = 0xff80_0000u32 as i32;

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
        return f32::from_bits(bits & F32_MSK_SIGN);
    }
    let msk = (SHIFT_MSK >> exp) as u32;
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
    let neg = bits & F32_MSK_SIGN;
    // Purely fractional so will just be zero or -1
    if exp < 0 {
        // preserve zero
        if (bits << 1) == 0 {
            return x;
        }
        // Minus one or zero.
        let ret = if neg > 0 { NEG_ONE } else { 0 };
        return f32::from_bits(ret);
    }

    let msk = (SHIFT_MSK >> exp) as u32;
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
    let neg = bits & F32_MSK_SIGN;
    // Purely fractional so will just be zero or one
    if exp < 0 {
        // preserve zero
        if (bits << 1) == 0 {
            return x;
        }
        // One or negative zero.
        let ret = if neg > 0 { neg } else { POS_ONE };
        return f32::from_bits(ret);
    }
    let msk = (SHIFT_MSK >> exp) as u32;
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
pub const fn roundevenf(x: f32) -> f32 {
    let bits = x.to_bits();
    let exp = exponentf(bits);

    // Exponent is too large to have a fraction so just return x
    if exp >= 23 {
        return x;
    }
    let neg = bits & F32_MSK_SIGN;
    let abs = bits & F32_MSK_ABS;
    // Purely fractional so it will always goto zero if at or below +- 0.5
    if exp < 0 {
        if abs <= HALF {
            return f32::from_bits(neg);
        }
        // Over the half-way point so round up
        return f32::from_bits(neg | POS_ONE);
    }
    let add = 1u32 << (23 - exp);
    let half = 1u32 << (23 - exp - 1);
    // decide:
    // - if frac > half: round up (add)
    // - if frac < half: trunc
    // - if frac == half: tie -> round to even: add only if the integer bit is 1.

    if (bits & add) != 0 { // odd
        if (bits & half) == 0 { // less than half

        }
        // Hmm i feel like this is method might be to branchy
        // let see if the int part is even I always truncate unless we already half-way
        // if it's odd we always truncate for less then half-way
        // so regardless of parity if under half truncate
        // if we always add half and truncate this will be correct for all intial odd parity.
        // if I do the same for even parity it always be correct except if halfway.
        // So for even parity we don't want to add half if at exactly halfway
        // Hmm seems like there should be away to combine that in one check.
    }

    todo!();
    let msk = (SHIFT_MSK >> exp) as u32;
    let new = bits & msk;

    // If not negative and there was a fractional part, add 1.0.
    if neg == 0 && bits != new {
        let add = 1u32 << (23 - exp);
        // This correctly carries into he exponent if necessary
        return f32::from_bits(new.wrapping_add(add));
    }
    return f32::from_bits(new);
}
