//! Traits for uniformly working with color spaces, see [`ColorSpace`] for the primary API.
use colorkit::colors::Xyz;
use colorkit::convert::FromColorBoth;
use colorkit::convert::IntoColor;
use colorkit::math::BoundF32;
use colorkit::num_type::Number;
use colorkit::wp::WhitePoint;

mod alpha;
mod bounds;
mod layout;
mod slice;
mod wrapper;

pub use alpha::AlphaKind;
pub use alpha::AlphaMaybe;
pub use alpha::AlphaNone;
pub use bounds::ColorBounds;
pub use layout::ColorLayout;
pub use slice::ColorSlice;
pub use wrapper::ColorWrap;
pub use wrapper::WrapIdentity;

/// Information about a Color Space
pub trait ColorData: Default {
    /// Number of channels or also should be the length of the array.
    type Channels: Number;
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

    /// Number of Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::Channels::value();
    }
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
    AlphaMaybe + ColorNew + ColorSlice + ColorBounds + ColorLayout + FromColorBoth<Xyz<Self::WhitePoint>>
{
    /// Create an instance of this color from a CIE XYZ color.
    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self {
        return color.into_color();
    }
    /// Create a CIE XYZ Color from this color
    fn into_xyz(self) -> Xyz<Self::WhitePoint> {
        return self.into_color();
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
