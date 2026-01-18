use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::scalar::Scalar;
use colorkit::scalar::norm_to_u32;

use super::BitUint;
use super::BitUintType;

impl<const N: u32, T: BitUintType> Scalar for BitUint<N, T> {
    const DEFAULT: Self = Self::DEFAULT;
    const SCALAR_MAX: Self = Self::MAX;
    const SCALAR_MIN: Self = Self::MIN;

    fn into_norm(self) -> crate::scalar::NormF32 {
        // TODO:
        // Value with the range of u32, u64, usize ect... are tricky.
        // Into nor TryInto is not defined for ints larger than the exact
        // range that a f32 or f64 can represent.
        // I could maybe TryInto a u64 and then u64 as f64.
        // I mainly care about the ratio here so I suppose an other option
        // cast into like a u64 and take the top 24 MSB bits of BitUINT.
        // Does mean a loss of precision, but for an image not sure why
        // there would be a channel with such high precision.
        let max = Self::MAX
            .get()
            .try_into_u32()
            .expect("BitUint range too large to normalize");
        if max >= 16777216 {
            panic!("BitUint range too large to normalize");
        }
        let max = max as f32;
        let val = self.get().try_into_u32().unwrap() as f32;
        // Safety: `v` will always be <= 1.0
        return NormF32::new_clamped(val / max);
    }

    fn from_norm_dither<D: crate::scalar::Dither>(value: NormF32, round: Rounding, dither: &mut D) -> Self {
        let max = Self::MAX
            .get()
            .try_into_u32()
            .expect("BitUint range too large to quantize");
        if max >= 16777216 {
            panic!("BitUint range too large to quantize");
        }
        let v = norm_to_u32(value, round, dither, max);
        return Self(T::try_from_u32(v).unwrap());
    }
}
