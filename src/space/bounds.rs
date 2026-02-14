use colorkit::math::BoundF32;

use super::ColorData;

/// Numeric bounds and clamping operations on those bounds.
pub trait ColorBounds: ColorData {
    /// Clamp all channels to min and max bounds.
    fn clamp(self) -> Self;
    /// Clamp a single channel at `index`
    fn clamp_channel(self, index: usize) -> Self;
    /// Check if the color’s channels are all within the range bounds.
    fn is_clamped(&self) -> bool;
    /// Check if the channel at `index` is within the range bounds.
    fn is_channel_clamped(&self, index: usize) -> bool;
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
