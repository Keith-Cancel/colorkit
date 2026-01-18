use colorkit::scalar::IntoScalar;
use colorkit::scalar::NormF32;

use super::Color;
use crate::layout::Layout;
use crate::scalar::Scalar;

pub trait ColorOps: Color {
    /// LERP between two colors
    fn lerp(self, other: Self, ratio: NormF32) -> Self {
        let Some(alpha_idx) = self.try_alpha_index() else {
            return Self::lerp_naive(self, other, ratio);
        };
        let mix = ratio.get();
        let lay_0 = self.as_layout();
        let lay_1 = other.as_layout();

        let a_0 = lay_0.get(alpha_idx).into_normf32().get();
        let a_1 = lay_1.get(alpha_idx).into_normf32().get();

        return Self::from_fn(|i| {
            let (a, b) = if i == alpha_idx {
                (a_0, a_1)
            } else {
                (lay_0.get(i).into_normf32() * a_0, lay_1.get(i).into_normf32() * a_1)
            };
            // Probably can use NormF32::from_saturated()
            let c = NormF32::new(a + mix * (b - a)).expect("lerp() non-normalized result");
            return c.into_scalar();
        });
    }

    /// LERP between two colors, but treat the alpha channel as
    /// any other channel.
    fn lerp_naive(self, other: Self, ratio: NormF32) -> Self {
        let mix = ratio.get();
        let l_0 = self.as_layout();
        let l_1 = other.as_layout();

        return Self::from_fn(|i| {
            let a: NormF32 = l_0.get(i).into_scalar();
            let b: NormF32 = l_1.get(i).into_scalar();
            // Probably can use NormF32::from_saturated()
            let c = NormF32::new(a + mix * (b - a)).expect("lerp_naive() non-normalized result");
            return c.into_scalar();
        });
    }
}
