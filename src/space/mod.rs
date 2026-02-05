//! Traits for uniformly working with color spaces, see [`ColorSpace`] for the primary API.
use colorkit::colors::Xyz;
use colorkit::convert::FromColorBoth;
use colorkit::convert::IntoColor;
use colorkit::layout::Layout;
use colorkit::layout::LayoutMap;
use colorkit::math::BoundF32;
use colorkit::num_type::Number;
use colorkit::scalar::Dither;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::wp::WhitePoint;

mod bounds;
mod slice;

pub use bounds::ColorBounds;
pub use slice::ColorSlice;

/// Information about a Color Space
pub trait ColorData: Default {
    /// Number of channels or also should be the length of the array.
    type Channels: Number;
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

/// The type of Alpha the color space is using.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlphaKind {
    None = 1,
    Normal,
    PreMul,
}

/// Market trait stating the color does not have an alpha channel.
///
/// If implemented a default blanket implention of [`ColorMaybeAlpha`]
/// is provided.
//pub trait ColorNoAlpha {}

pub trait ColorMaybeAlpha {
    /// The kinda of Alpha Channel the color space has
    const ALPHA_KIND: AlphaKind;
    /// If the color has an alpha channel the index of the channel.
    const ALPHA_INDEX: Option<usize>;
    /// The color's type with no alpha channel.
    ///
    /// This should is generally just equal to `Self`.
    /// except in the case of wrapper types like
    /// [`Alpha`](colorkit::colors::Alpha)
    /// and [`AlphaPre`](colorkit::colors::AlphaPre)
    // Need Colorspace bound so after stripping we keep color space operations
    type NoAlpha: ColorSpace;
    /// Remove the alpha channel if present.
    ///
    /// Otherwise this should just return `Self`
    fn strip_alpha(self) -> Self::NoAlpha;
    /// Try to use the alpha channel if present, otherwise default to `1.0`
    /// for fully opaque.
    fn opacity(&self) -> f32;
    /// Try returning a reference to the alpha channel, if present.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha_ref(&self) -> Option<&f32>;
    /// Try to returning a mutable reference to the alpha channel, if present.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha_mut(&mut self) -> Option<&mut f32>;
}
/*
impl<T: ColorNoAlpha> ColorMaybeAlpha for T
where
    T: ColorMaybeAlpha<NoAlpha = T>, // equality constraint
{
    type NoAlpha = Self;
    const ALPHA_KIND: AlphaKind = AlphaKind::None;
    const ALPHA_INDEX: Option<usize> = None;
    #[inline]
    fn opacity(&self) -> f32 {
        return 1.0;
    }
    #[inline]
    fn strip_alpha(self) -> Self::NoAlpha {
        return self;
    }
    #[inline]
    fn try_alpha_mut(&mut self) -> Option<&mut f32> {
        return None;
    }
    #[inline]
    fn try_alpha_ref(&self) -> Option<&f32> {
        return None;
    }
}*/

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
    /// Construct a [`Layout`] from a given color and [`LayoutMap`].
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout_map<L: Layout, M: LayoutMap>(self, round: Rounding) -> L;
    /// Construct a [`Layout`] from a given color and [`Dither`]
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout_dither<L: Layout, D: Dither>(self, round: Rounding, dither: &mut D) -> L;
    /// Construct a [`Layout`] from a given color, [`Dither`] and [`LayoutMap`]
    ///
    /// Channel count of the the [`Layout::Channels`] should
    /// equal the color space channels.
    fn into_layout_dither_map<L: Layout, D: Dither, M: LayoutMap>(self, round: Rounding, dither: &mut D) -> L;
}

/// Trait for creating a color.
pub trait ColorNew: ColorData + Sized {
    /// Construct the Color calling `f(i)` for each index
    /// (same semantics as [`core::array::from_fn`]).
    fn from_fn<F: FnMut(usize) -> f32>(fun: F) -> Self;
    /// Construct the color from an array.
    #[cfg(feature = "type_const")]
    fn from_array(array: [f32; <Self::Channels as Number>::N]) -> Self {
        return Self::from_fn(|i| array[i]);
    }
    #[cfg(not(feature = "type_const"))]
    fn from_array(array: <Self::Channels as Number>::Arr<f32>) -> Self {
        return Self::from_fn(|i| array[i]);
    }
    /// Creates a color by repeatedly copying the value to each channel.
    ///
    /// This is similar to [`core::array::repeat`] except since [`f32`] is
    /// copiable so no need to clone.
    fn repeat(value: f32) -> Self {
        return Self::from_fn(|_| value);
    }
}

/// The main ColorSpace Trait
pub trait ColorSpace:
    ColorNew + ColorSlice + ColorLayout + ColorMaybeAlpha + FromColorBoth<Xyz<Self::WhitePoint>>
{
    /// Number Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::Channels::value();
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
    /// Return channel at `index` normalized to the range `[0.0, 1.0]`.
    ///
    /// The normalization bounds are color-space specific - see the particular
    /// color space's documentation for details. For explicit control over the
    /// bounds, use [`ColorSpace::get_norm_bounds`].
    ///
    /// Further, not all color spaces are bounded on every channel. So
    /// implementations may pick practical bounds and return
    /// a best effort normalization based on those.
    ///
    /// # Panics
    /// May panic if `index` is out of bounds.
    fn get_norm(&self, index: usize) -> NormF32;
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
