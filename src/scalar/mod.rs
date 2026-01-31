//! Types that can be a channel [`Scalar`] for a color [`Layout`](colorkit::layout::Layout)
//!
//! This module defines the [`Scalar`] trait and conversion traits
//! [`FromScalar`] / [`IntoScalar`] used to convert between scalar
//! representations used by layouts.
//!
//! Provided `Scalar` implementations:
//! - [`u8`]
//! - [`u16`]
//! - [`NormF32`]
//! - [`BitUint`]

mod bit_uint;
mod norm_f32;
mod uint16;
mod uint8;

use core::any::Any;

use colorkit::math::ceilf;
use colorkit::math::floorf;
use colorkit::math::roundevenf;
use colorkit::math::truncf;

#[rustfmt::skip]
pub use bit_uint::BitUint;
pub use bit_uint::BitUintType;
pub use norm_f32::NormF32;
pub use norm_f32::NotNormalized;

/// Deterministic rounding modes used after scaling + optional dither.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Rounding {
    Nearest = 1,
    Even,
    TowardZero,
    Floor,
    Ceil,
}

impl Rounding {
    /// Returns an iterator starting from this variant (inclusive)
    pub fn iter_from(&self) -> RoundingIter {
        return RoundingIter(Some(*self));
    }

    /// Returns an iterator over all variants
    pub fn iter_all() -> RoundingIter {
        return RoundingIter(Some(Self::Nearest));
    }
}

/// Mainly for testing. Just help iterate through each rounding mode variant.
#[derive(Copy, Clone, Debug)]
pub struct RoundingIter(Option<Rounding>);

impl Iterator for RoundingIter {
    type Item = Rounding;
    fn next(&mut self) -> Option<Self::Item> {
        let Some(cur) = self.0 else {
            return None;
        };
        let next = match cur {
            Rounding::Nearest => Some(Rounding::Even),
            Rounding::Even => Some(Rounding::TowardZero),
            Rounding::TowardZero => Some(Rounding::Floor),
            Rounding::Floor => Some(Rounding::Ceil),
            Rounding::Ceil => None,
        };
        *self = Self(next);
        return Some(cur);
    }
}

/// Dither generator trait (produces additive noise).
pub trait Dither {
    /// Returns a new value (typically `scaled + noise`) ready for rounding.
    fn sample(&mut self, value: f32) -> f32;

    /// Reset the dither state (optional).
    /// Useful for stateful dithers or when processing independent blocks.
    fn reset(&mut self) {}

    /// Advance any inteneral state (optional).
    /// For example a dither that tracks 2d posistion.
    fn advance(&mut self) {}
}

/// No-OP Dither just passes value unchanged.
#[derive(Copy, Clone, Debug)]
pub struct NoDither;
impl Dither for NoDither {
    #[inline(always)]
    fn sample(&mut self, value: f32) -> f32 {
        return value;
    }
}

/// A scalar is bounded sample of a channel.
pub trait Scalar: Copy + PartialEq + PartialOrd + 'static {
    /// Default Value
    const DEFAULT: Self;
    /// Inclusive minimum value for this channel/component.
    const SCALAR_MIN: Self;
    /// Inclusive maximum value for this channel/component.
    const SCALAR_MAX: Self;

    /// Convert the scalar into a [`NormF32`]
    fn into_norm(self) -> NormF32;
    /// Create a scalar [`NormF32`] and rounding choice, and dithering choice.
    fn from_norm_dither<D: Dither>(value: NormF32, round: Rounding, dither: &mut D) -> Self;
    /// Create a scalar from a [`NormF32`] and rounding choice.
    fn from_norm(value: NormF32, round: Rounding) -> Self {
        return Self::from_norm_dither(value, round, &mut NoDither);
    }
    /// Re-quantize a scalar to an other scalar with a specfic rounding.
    fn requantize<T: Scalar>(self, rounding: Rounding) -> T {
        return Self::requantize_dither(self, rounding, &mut NoDither);
    }
    /// Re-quantize a scalar to an other scalar with a specfic rounding and dithering choice.
    fn requantize_dither<T: Scalar, D: Dither>(self, rounding: Rounding, dither: &mut D) -> T {
        let any: &dyn Any = &self;
        // Just pass the value if T and Self are the same.
        // This avoids an unnecessary requantize round-trip losses bias from rounding or dithering
        if let Some(v) = any.downcast_ref::<T>() {
            return *v;
        }
        return T::from_norm_dither(self.into_norm(), rounding, dither);
    }

    /// Clamp this scalar to the inclusive [`SCALAR_MIN`, `SCALAR_MAX`] range.
    #[inline]
    fn clamp_scalar(self) -> Self {
        let min = Self::SCALAR_MIN;
        let max = Self::SCALAR_MAX;
        let ret = if self < min { min } else { self };
        let ret = if ret > max { max } else { ret };
        return ret;
    }
}

/// Requantize from `S` into `Self` with nearest rounding.
///
/// Similar to [`core::convert::From`] but does not imply a lossless conversion.
pub trait FromScalar<S: Scalar>: Scalar {
    fn from_scalar(s: S) -> Self;
}

impl<S1: Scalar, S2: Scalar> FromScalar<S1> for S2 {
    #[inline]
    fn from_scalar(s: S1) -> Self {
        return s.requantize(Rounding::Nearest);
    }
}
/// Requantize `self` into `S` with nearest rounding.
///
/// A blanket impl is provided when `S: FromScalar<Self>`.
pub trait IntoScalar<S: Scalar>: Scalar {
    fn into_scalar(self) -> S;
}

impl<S1: Scalar, S2: Scalar + FromScalar<S1>> IntoScalar<S1> for S2 {
    #[inline]
    fn into_scalar(self) -> S1 {
        return S1::from_scalar(self);
    }
}

// Helper functions
// ==================================================
/// Helper to convert normf32 to [0 to `max`]
#[inline]
fn norm_to_u32<D: Dither>(norm: NormF32, round: Rounding, dither: &mut D, max: u32) -> u32 {
    let max = max as f32;
    let scaled = norm * max;
    let dith = dither.sample(scaled);
    let round = match round {
        Rounding::Nearest => todo!(), //dith.round(),
        Rounding::Even => roundevenf(dith),
        Rounding::TowardZero => truncf(dith),
        Rounding::Floor => floorf(dith),
        Rounding::Ceil => ceilf(dith),
    };
    return round.clamp(0.0, max) as u32;
}
