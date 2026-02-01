//! Traits for uniformly working with color spaces, see [`ColorSpace`] for the primary API.
use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::ops::Index;
use core::ops::IndexMut;

use colorkit::colors::Xyz;
use colorkit::convert::FromColorBoth;
use colorkit::convert::IntoColor;
use colorkit::layout::Layout;
use colorkit::layout::LayoutMap;
use colorkit::math::BoundF32;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::wp::WhitePoint;

/// Information about a Color Space
pub trait ColorData: Default {
    /// Number of channels or also should be the length of the array.
    const CHANNELS: usize;
    /// Default color, should be black.
    const DEFAULT: Self;
    /// Color Spaces White Point
    type WhitePoint: WhitePoint;
    /// Are the Channels Linear
    const LINEAR: bool;
    /// Upper or max bound of each channel.
    const CHANNEL_MAX: &'static [BoundF32];
    /// Lower or min bound of each channel.
    const CHANNEL_MIN: &'static [BoundF32];

    // what else to add?
    // primaries?
}

/// Trait to let Color Spaces be handled mostly like an array/slice.
pub trait ColorArray:
    Copy
    + AsRef<[f32]>
    + AsMut<[f32]>
    + Borrow<[f32]>
    + BorrowMut<[f32]>
    + Index<usize, Output = f32>
    + IndexMut<usize, Output = f32>
{
    /// Construct the Color calling `f(i)` for each index (same semantics as [`core::array::from_fn`]).
    fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self;
    /// View color as a slice reference.
    fn as_slice(&self) -> &[f32];
    /// View color as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [f32];
    /// Try to get a reference as an array.
    ///
    /// If `N` is greater than [`ColorData::CHANNELS`] returns [`None`]`
    fn try_as_array<const N: usize>(&self) -> Option<&[f32; N]> {
        let slc = self.as_slice();
        if N > slc.len() {
            return None;
        }
        let (slc, _) = slc.split_at(N);
        return slc.try_into().ok();
    }
    /// Try to get a reference as an mutable array.
    ///
    /// If `N` is greater than [`ColorData::CHANNELS`] returns [`None`]`
    fn try_as_mut_array<const N: usize>(&mut self) -> Option<&mut [f32; N]> {
        let slc = self.as_mut_slice();
        if N > slc.len() {
            return None;
        }
        let (slc, _) = slc.split_at_mut(N);
        return slc.try_into().ok();
    }

    /// Get channel value reference or `None`.
    fn get_ref(&self, index: usize) -> Option<&f32> {
        return self.as_slice().get(index);
    }

    /// Get a mutable channel value reference or `None`.
    fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        return self.as_mut_slice().get_mut(index);
    }
}

/// Allows a [`ColorSpace`] converted to and from various [`Layout`].
pub trait ColorLayout: Sized {
    /// Construct a color from a [`Layout].
    ///
    /// Channel count of the the [`Layout::Channels`] should be greater
    /// than or equal to the color space channels.
    fn from_layout<L: Layout>(layout: L) -> Self;
    /// Construct a color from a [`Layout`], and a [`LayoutMap`]
    ///
    /// This function is similar to [`ColorLayout::from_layout`],
    /// other than the map should be used to index the layout.
    /// For example the layout is ARGB, RGBA ect...
    ///
    /// Channel count of the the [`Layout::Channels`] should be greater
    /// than or equal to the color space channels.
    fn from_layout_map<L: Layout, M: LayoutMap<Channels = L::Channels>>(layout: L) -> Self;
    /// Construct a [`Layout`] from a given color.
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout<L: Layout>(self, round: Rounding) -> L;
}

/// The main ColorSpace Trait
pub trait ColorSpace: ColorArray + ColorData + ColorLayout + FromColorBoth<Xyz<Self::WhitePoint>> {
    /// Number Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::CHANNELS;
    }

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

    /// Check if the color’s channels are all within the range bounds.
    fn within_bounds(&self) -> bool {
        for (i, &v) in self.as_slice().iter().enumerate() {
            if let BoundF32::Include(max) = Self::CHANNEL_MAX[i]
                && v > max
            {
                return false;
            }
            if let BoundF32::Include(min) = Self::CHANNEL_MIN[i]
                && v < min
            {
                return false;
            }
        }
        return true;
    }

    /// Clamp all channels to min and max
    fn clamp(self) -> Self {
        let slc = self.as_slice();
        return Self::from_fn(|i| {
            let mut v = slc[i];
            if let BoundF32::Include(max) = Self::CHANNEL_MAX[i]
                && v > max
            {
                v = max;
            }
            if let BoundF32::Include(min) = Self::CHANNEL_MIN[i]
                && v < min
            {
                v = min;
            }
            v
        });
    }
    /// Create an instance of this color from a CIE XYZ color.
    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self {
        return color.into_color();
    }
    /// Create a CIE XYZ Color from this color
    fn into_xyz(self) -> Xyz<Self::WhitePoint> {
        return self.into_color();
    }
    /// Get a channel of the color space, normalized `0.0` and `1.0`.
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
    fn get_norm_bounds(&self, index: usize, min: f32, max: f32) -> NormF32 {
        let val = self[index];
        let rng = max - min;
        if (min >= max) || !rng.is_finite() {
            panic!("Invalid normalization range: require finite min < max");
        }
        let n = (val - min) / rng;
        return NormF32::new(n);
    }
}

/// Trait with common operations for RGB like color spaces.
pub trait RgbLike: ColorSpace {
    /// Create a new color from RGB values.
    #[inline]
    fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        let arr = [r, g, b];
        return Self::from_fn(|i| arr[i]);
    }
    /// Get the Color's red channel value.
    #[inline]
    fn red(&self) -> f32 {
        return self[0];
    }
    /// Get the Color's blue channel value.
    #[inline]
    fn blue(&self) -> f32 {
        return self[1];
    }
    /// Get the Color's green channel value.
    #[inline]
    fn green(&self) -> f32 {
        return self[2];
    }
    /// Set the Color's red channel value.
    #[inline]
    fn set_red(&mut self, value: f32) {
        self[0] = value;
    }
    /// Set the Color's blue channel value.
    #[inline]
    fn set_blue(&mut self, value: f32) {
        self[1] = value;
    }

    /// Set the Color's green channel value.
    #[inline]
    fn set_green(&mut self, value: f32) {
        self[2] = value;
    }
}
