use colorkit::space::AlphaKind;
use colorkit::space::ColorSpace;

/// Extension trait for interpolation operations on colors
pub trait Interpolation {
    /// Linearly interpolate between two colors.
    ///
    /// # Parameters
    /// - `self`: Starting color (t = 0)
    /// - `other`: Ending color (t = 1)
    /// - `ratio`: Interpolation factor (0.0 to 1.0)
    ///
    /// Further, performs channel-wise linear interpolation, this
    /// works in any color space, but the perceptual result depends
    /// on the space's linearity.
    fn lerp(&self, other: &Self, ratio: f32) -> Self;
    /// Linearly interpolate between two colors.
    ///
    /// # Parameters
    /// - `self`: Starting color (t = 0)
    /// - `other`: Ending color (t = 1)
    /// - `ratio`: Interpolation factor (0.0 to 1.0)
    ///
    /// # Note
    /// It naively treats the alpha channel as any other channel.
    ///
    /// Further, performs channel-wise linear interpolation, this
    /// works in any color space, but the perceptual result depends
    /// on the space's linearity.
    fn lerp_naive(&self, other: &Self, ratio: f32) -> Self;
}

impl<C: ColorSpace> Interpolation for C {
    fn lerp(&self, other: &Self, ratio: f32) -> Self {
        let r = ratio.clamp(0.0, 1.0);
        // If the color type stores premultiplied channels naive lerp is correct.
        if matches!(Self::ALPHA_KIND, AlphaKind::PreMul) {
            return C::lerp_naive(&self, other, r);
        }
        // If there is no alpha channel, fall back to naive per-component lerp.
        let Some(a_idx) = Self::ALPHA_INDEX else {
            return C::lerp_naive(self, other, r);
        };
        let alp_0 = self[a_idx];
        let alp_1 = other[a_idx];
        let alp_2 = alp_0 + r * (alp_1 - alp_0);
        // Fully transparent result: avoid division, preserve channels
        if alp_2 == 0.0 {
            return C::lerp_naive(self, other, r);
        }

        return Self::from_fn(|i| {
            let a = self[i];
            let b = other[i];
            if i == a_idx {
                return alp_2;
            }
            let a = a * alp_0;
            let b = b * alp_1;
            let c = a + r * (b - a);
            return c / alp_2;
        });
    }

    fn lerp_naive(&self, other: &Self, ratio: f32) -> Self {
        let r = ratio.clamp(0.0, 1.0);
        return Self::from_fn(|i| {
            let a = self[i];
            let b = other[i];
            return a + r * (b - a);
        });
    }
}
