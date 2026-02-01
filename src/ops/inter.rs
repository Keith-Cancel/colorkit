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
        if matches!(Self::ALPHA_KIND, AlphaKind::None) {
            return C::lerp_naive(&self, other, r);
        }
        // I really just wish I could make an of array [f32; C::CHANNELS]

        let a_0 = *self.try_alpha().unwrap_or(&1.0);
        let a_1 = *other.try_alpha().unwrap_or(&1.0);
        todo!();
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
