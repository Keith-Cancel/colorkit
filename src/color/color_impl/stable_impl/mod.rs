mod inner;
mod macros;

use colorkit::space::ColorSpace;
use colorkit::utils::*;
use inner::ColorInner;
use macros::color_fn_new;

/// Represention of a color using [`f32`] values.
///
/// The number channels is specified by the color space
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Color<S: ColorSpace>(pub(crate) ColorInner<S::Channels>);

impl<S: ColorSpace> Color<S> {
    /// Creates a color by filling all channels with the given value.
    pub const fn repeat(value: f32) -> Self {
        let _ = Self::COLOR_ASSERT;
        color_fn_new!(repeat(value));
    }
    /// Try to create a color form a slice, copying each element
    /// from the slice.
    ///
    /// Returns `None` if the slice's length does not much the number
    /// of channels.
    #[inline]
    pub const fn try_from_slice(values: &[f32]) -> Option<Self> {
        let _ = Self::COLOR_ASSERT;
        let Some(inner) = ColorInner::<S::Channels>::try_from_slice(values) else {
            return None;
        };
        return Some(Self(inner));
    }

    /// Returns a reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&f32> {
        return self.0.get(index);
    }
    /// Returns a mutable reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        return self.0.get_mut(index);
    }
    /// View the color as slice containing all it's channels.
    #[inline]
    pub const fn as_slice(&self) -> &[f32] {
        return self.0.as_slice();
    }
    /// View the color as a mutable slice containing all it's channels.
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [f32] {
        return self.0.as_mut_slice();
    }

    /// Creates a color by filling all channels with the given value.
    #[inline]
    pub(crate) const fn crate_new(values: <S::Channels as Number>::Arr<f32>) -> Self {
        let _ = Self::COLOR_ASSERT;
        return Self(ColorInner(values));
    }
}
