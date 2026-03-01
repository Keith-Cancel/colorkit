use core::any::Any;
use core::any::TypeId;

use super::Dither;
use super::NormF32;
use super::Rounding;
use super::Scalar;
use super::norm_to_u32;

impl Scalar for u16 {
    const DEFAULT: Self = 0;
    const SCALAR_MAX: Self = u16::MAX;
    const SCALAR_MIN: Self = u16::MIN;

    #[inline]
    fn into_norm(self) -> NormF32 {
        let v = (self as f32) / 65535.0;
        // Safety: `v` will always be <= 1.0
        return unsafe { NormF32::new_unchecked(v) };
    }

    fn from_norm_dither<D: Dither>(value: NormF32, round: Rounding, dither: &mut D) -> Self {
        return norm_to_u32(value, round, dither, Self::MAX.into()) as Self;
    }

    /// Re-quantize a scalar to an other scalar with a specific rounding choice.
    fn requantize<T: Scalar>(self, rounding: Rounding) -> T {
        let any: &dyn Any = &self;
        // Just pass the value if T and Self are the same.
        // Otherwise just can add additional error or bias.
        if let Some(v) = any.downcast_ref::<T>() {
            return *v;
        }

        // Specializtion without speciliztion
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let v = (self as u32) * 255;
            let v = match rounding {
                Rounding::Nearest | Rounding::Even => v + 32767,
                Rounding::TowardZero | Rounding::Floor => v,
                Rounding::Ceil => v + 65534,
            };
            let v = (v / 65535) as u8;
            let any: &dyn Any = &v;
            return *(any.downcast_ref::<T>().unwrap());
        }

        return T::from_norm(self.into_norm(), rounding);
    }

    #[inline(always)]
    fn clamp_scalar(self) -> Self {
        return self;
    }
}

#[cfg(test)]
mod test {
    use super::Rounding;
    use super::Scalar;
    use crate::scalar::NormF32;

    #[test]
    #[cfg(not(miri))] // Quite slow under miri so don't run on miri.
    fn u16_check() {
        for i in 0..=u16::MAX {
            let norm = i.into_norm();

            for r in Rounding::iter_all() {
                // v0 is the generic impl.
                let v0 = u8::from_norm(norm, r);
                let v1: u8 = i.requantize(r);
                assert_eq!(v0, v1);

                let n: NormF32 = i.requantize(r);
                assert_eq!(n, norm);
            }
        }
    }
}
