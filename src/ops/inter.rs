use colorkit::scalar::NormF32;
use colorkit::space::ColorSpace;

/// Extension trait for interpolation operations on colors
pub trait Interpolation {
    fn lerp_naive(&self, other: &Self, ratio: NormF32) -> Self;
}

impl<C: ColorSpace> Interpolation for C {
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
    fn lerp_naive(&self, other: &Self, ratio: NormF32) -> Self {
        return Self::from_fn(|i| {
            let a = self[i];
            let b = other[i];
            return a + ratio * (b - a);
        });
    }
}
