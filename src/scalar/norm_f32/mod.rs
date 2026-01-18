mod add_sub;
mod cmp;
mod div_mul;
mod macros;

use super::Dither;
use super::Rounding;
use super::Scalar;

/// Floating point value is outside the range [0.0, 1.0]
#[derive(Debug, Clone, Copy)]
pub struct NotNormalized(pub f32);

/// An f32 normalized between [0.0, 1.0]
#[derive(Copy, Debug, Default)]
#[repr(transparent)]
pub struct NormF32(f32);

impl NormF32 {
    pub const MAX: Self = Self(1.0);
    pub const MIN: Self = Self(0.0);
    pub const BITS: u32 = (size_of::<Self>() * 8) as u32;

    pub const fn new_clamped(value: f32) -> Self {
        if value.is_nan() {
            return Self::MIN;
        }
        return Self(value.clamp(0.0, 1.0));
    }

    pub const fn new(value: f32) -> Result<Self, NotNormalized> {
        if value > 1.0 || value < 0.0 || value.is_nan() || value.is_infinite() {
            return Err(NotNormalized(value));
        }
        return Ok(Self(value));
    }

    /// unsafe unchecked constructor — caller guarantees invariant
    /// (pattern similar to std::NonZero types).
    #[inline]
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        return Self(value);
    }

    pub const fn max(self, other: Self) -> Self {
        return Self(f32::max(self.0, other.0));
    }

    pub const fn min(self, other: Self) -> Self {
        return Self(f32::min(self.0, other.0));
    }

    pub const fn clamp(self, min: Self, max: Self) -> Self {
        return Self(f32::clamp(self.0, min.0, max.0));
    }

    pub const fn midpoint(self, other: Self) -> Self {
        return Self(f32::midpoint(self.0, other.0));
    }

    #[inline(always)]
    pub const fn get(self) -> f32 {
        return self.0;
    }

    // TODO remove this when no long need for rust
    // min_const_generic_args
    #[rustfmt::skip] // prevent rust::fmt removing the const { ... } on save
    #[allow(unused_braces)]
    pub const fn to_be_bytes(self) -> [u8; const { size_of::<Self>() }] {
        return self.0.to_be_bytes();
    }

    #[rustfmt::skip]
    #[allow(unused_braces)]
    pub const fn to_le_bytes(self) -> [u8; const { size_of::<Self>() }] {
        return self.0.to_le_bytes();
    }

    #[rustfmt::skip]
    #[allow(unused_braces)]
    pub const fn to_ne_bytes(self) -> [u8; const { size_of::<Self>() }] {
        return self.0.to_ne_bytes();
    }
}

impl Clone for NormF32 {
    #[inline(always)]
    fn clone(&self) -> Self {
        return *self;
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl Scalar for NormF32 {
    const DEFAULT: Self = NormF32::MIN;
    const SCALAR_MAX: Self = NormF32::MAX;
    const SCALAR_MIN: Self = NormF32::MIN;

    #[inline(always)]
    fn from_norm(value: NormF32, _: Rounding) -> Self {
        // NormF32 is already a full-precision normalized just pass through
        return value;
    }

    #[inline(always)]
    fn from_norm_dither<D: Dither>(value: NormF32, _: Rounding, _: &mut D) -> Self {
        // NormF32 is already a full-precision normalized value; quantization
        // (rounding/dither) apply only when converting to discrete scalars.
        return value;
    }

    #[inline(always)]
    fn into_norm(self) -> NormF32 {
        return self;
    }
}

// Implement From Traits
// ==================================================
impl From<NormF32> for f32 {
    #[inline(always)]
    fn from(value: NormF32) -> Self {
        return value.0;
    }
}

impl TryFrom<f32> for NormF32 {
    type Error = NotNormalized;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        return Self::new(value);
    }
}

#[cfg(test)]
mod test {
    use super::NormF32;

    #[test]
    fn check_default() {
        assert_eq!(NormF32::default().0, 0.0);
    }

    #[test]
    fn ops() {
        let a = NormF32::new(0.5).unwrap();
        let b = &a;

        let mut v = 100.0;
        v += a;
        v -= a;
        v *= a;
        v /= a;
        assert_eq!(v, 100.0);

        v += b;
        v -= b;
        v *= b;
        v /= b;
        assert_eq!(v, 100.0);

        assert_eq!(v + a, 100.5);
        assert_eq!(v + b, 100.5);
        assert_eq!(a + v, 100.5);
        assert_eq!(b + v, 100.5);
        assert_eq!(v - a, 99.5);
        assert_eq!(v - b, 99.5);
        assert_eq!(a - v, -99.5);
        assert_eq!(b - v, -99.5);
        assert_eq!(v * a, 50.0);
        assert_eq!(v * b, 50.0);
        assert_eq!(a * v, 50.0);
        assert_eq!(b * v, 50.0);
        assert_eq!(v / a, 200.0);
        assert_eq!(v / b, 200.0);
        assert_eq!(a / v, 0.005);
        assert_eq!(b / v, 0.005);

        assert_eq!(v % a, 0.0);
        assert_eq!(v % b, 0.0);
        assert_eq!(a % v, 0.5);
        assert_eq!(b % v, 0.5);

        v = 1.25;
        v %= a;
        assert_eq!(v, 0.25);

        v = 1.25;
        v %= b;
        assert_eq!(v, 0.25);
    }
}
