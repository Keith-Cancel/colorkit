mod alpha;
mod map;
mod space;

use colorkit::layout::Layout;
use colorkit::scalar::Scalar;

#[rustfmt::skip]
pub use alpha::*;
pub use map::*;
pub use space::*;

/// This trait specifies the base information needed to handle a color.
pub trait ColorFormat: Copy + Sized + Default {
    /// Scalar type used by the Layout
    ///
    /// Mainly, convience to avoid having to always use the [`Layout::Scalar`]
    type Scalar: Scalar;
    // TODO:
    // Would be nice to do something like this:
    // type Space: ColorSpace<CHANNELS = { Self::Map::MAP.len() }>;
    // However, can't in stable at the momement. Although if that
    // ever becomes stable there probably are some nicer avenues overall
    // to restructure this trait. For now my provided color type will
    // enforce this.
    /// The color space of the color.
    type Space: ColorSpace;
    /// The underlying storage layout.
    type Layout: Layout<Scalar = Self::Scalar>;
    /// What channel is the alpha channel on.
    type Alpha: AlphaChannel;
    /// Channel Order
    type Map: ChannelMap;

    /// View of the color's underlying layout.
    fn as_layout(&self) -> &Self::Layout;

    /// Mutable view of the color's underlying layout.
    fn as_layout_mut(&mut self) -> &mut Self::Layout;

    /// Create a Color by calling `fun` for each channel index. The index
    /// should raw index of each channel, basically should matchup
    /// with the index of layout indexs for like [`ColorFormat::as_layout`]
    ///
    /// Similar to [std::array::from_fn]
    fn from_fn<F: FnMut(usize) -> Self::Scalar>(fun: F) -> Self;

    /// Creates a new color with the given layout.
    fn from_layout(layout: Self::Layout) -> Self {
        return Self::from_fn(|i| layout.get(i));
    }

    /// Remmaped layout index based off [`AlphaChannel`] and [`ChannelMap`]
    #[inline]
    fn layout_index(index: usize) -> usize {
        let mut i = Self::Map::MAP[index];
        if let Some(a) = Self::Alpha::CHANNEL {
            i = i + (i >= a) as usize;
        }
        return i;
    }
}
