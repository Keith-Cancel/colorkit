use colorkit::space::ColorSpace;
use colorkit::utils::Number;

/// Represention of a color using [`f32`] values.
///
/// The number channels is specified by the color space
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Color<S: ColorSpace>(pub(crate) [f32; <S::Channels as Number>::N]);

impl<S: ColorSpace> Color<S> {
    /// Create a Color from an array. Only available with "type_const" feature.
    #[inline]
    pub const fn from_array(values: [f32; <S::Channels as Number>::N]) -> Self {
        let _ = Self::COLOR_ASSERT;
        return Self(values);
    }
    /// Convert a [`Color`] into an array. Only available with "type_const" feature.
    #[inline]
    pub const fn into_array(self) -> [f32; <S::Channels as Number>::N] {
        return self.0;
    }
    /// Creates a color by filling all channels with the given value.
    #[inline]
    pub const fn repeat(value: f32) -> Self {
        let _ = Self::COLOR_ASSERT;
        return Self([value; <S::Channels as Number>::N]);
    }
    /// Try to create a color form a slice, copying each element
    /// from the slice.
    ///
    /// Returns `None` if the slice's length does not much the number
    /// of channels.
    #[inline]
    pub const fn try_from_slice(values: &[f32]) -> Option<Self> {
        let _ = Self::COLOR_ASSERT;
        if values.len() != <S::Channels as Number>::N {
            return None;
        }
        let ptr = values.as_ptr() as *const [f32; <S::Channels as Number>::N];
        // Safety:
        // Made certain slice length matches array length.
        return Some(Self(unsafe { ptr.read() }));
    }
    /// Returns a reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&f32> {
        if index >= self.0.len() {
            return None;
        }
        return Some(&self.0[index]);
    }
    /// Returns a mutable reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        if index >= self.0.len() {
            return None;
        }
        return Some(&mut self.0[index]);
    }
    /// View the color as slice containing all it's channels.
    #[inline]
    pub const fn as_slice(&self) -> &[f32] {
        return &self.0;
    }
    /// View the color as a mutable slice containing all it's channels.
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [f32] {
        return &mut self.0;
    }

    #[inline]
    pub(crate) const fn crate_new(values: [f32; <S::Channels as Number>::N]) -> Self {
        return Self::from_array(values);
    }

    #[inline]
    pub(crate) const fn crate_inner(self) -> [f32; <S::Channels as Number>::N] {
        return self.0;
    }
}
