use colorkit::math::BoundF32;
use colorkit::scalar::NormF32;

use super::ColorData;

/// Bounds and clamping operations on a Color.
pub trait ColorBounds: ColorData {
    /// Clamp all channels to min and max bounds.
    fn clamp(self) -> Self;
    /// Clamp a single channel at `index`
    fn clamp_channel(self, index: usize) -> Self;
    /// Check if the color’s channels are all within the range bounds.
    fn is_clamped(&self) -> bool;
    /// Check if the channel at `index` is within the range bounds.
    fn is_channel_clamped(&self, index: usize) -> bool;
    /// Return channel at `index` normalized to the range `[0.0, 1.0]`.
    ///
    /// The normalization bounds are color-space specific - see the particular
    /// color space's documentation for details. For explicit control over the
    /// bounds, use [`ColorBounds::get_norm_bounds`].
    ///
    /// Further, not all color spaces are bounded on every channel. So
    /// implementations may pick practical bounds and return
    /// a best effort normalization based on those.
    ///
    /// # Panics
    /// May panic if `index` is out of bounds.
    fn get_norm(&self, index: usize) -> NormF32;
    /// Get `(min, max)` bounds to compute a [`NormF32`] for the given `index`.
    ///
    /// * This function should be consistent with [`ColorBounds::get_norm`]
    ///   using the returned `(min, max)` should produce the same [``NormF32``]
    ///
    /// * As noted in [`ColorBounds::get_norm`] not every color space is bounded
    ///   on every channel so (min, max) may be reasonable or best effort values
    ///   for the color space.
    fn get_norm_bounds(&self, index: usize) -> (f32, f32);
    /// Return channel at `index` normalized to the range `[0.0, 1.0]`.
    ///
    /// # Note
    /// Not all color spaces are bounded on all channels, what bounds
    /// to use depends on the color space. You might choose a practical
    /// min and max that may be never reached in practice. Or in the case
    /// of something like CIE XYZ use something based of the white point
    /// ect...
    ///
    /// # Panics
    /// * If `min >= max` or either bound is not finite (this would produce a
    ///   division by zero or an invalid normalization range).
    /// * If `index` is out of bounds for the color (same behavior as indexing).
    fn get_norm_bounded(&self, index: usize, min: f32, max: f32) -> NormF32;
    /// Get Max value for a given channel in the color space
    #[inline(always)]
    fn channel_max(ch_num: usize) -> BoundF32 {
        return Self::CHANNEL_MAX[ch_num];
    }
    /// Get min value for a given channel in the color space
    #[inline(always)]
    fn channel_min(ch_num: usize) -> BoundF32 {
        return Self::CHANNEL_MIN[ch_num];
    }
}
