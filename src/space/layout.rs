use colorkit::layout::Layout;
use colorkit::scalar::Dither;
use colorkit::scalar::Rounding;

use super::ColorData;

/// Allows a [`ColorSpace`](colorkit::space::ColorSpace) converted to and from various [`Layout`].
pub trait ColorLayout: ColorData {
    /// Construct a color from a [`Layout].
    ///
    /// Channel count of the the [`Layout::Channels`] should be greater
    /// than or equal to the color space channels.
    fn from_layout<L: Layout, T: AsRef<L>>(layout: &T) -> Self;
    /// Construct a [`Layout`] from a given color.
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout<L: Layout<Channels = Self::Channels>>(self, round: Rounding) -> L;
    /// Construct a [`Layout`] from a given color and [`Dither`]
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout_dither<L: Layout<Channels = Self::Channels>, D: Dither>(
        self,
        round: Rounding,
        dither: &mut D,
    ) -> L;
}
