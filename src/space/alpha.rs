use super::ColorData;
use super::ColorWrap;
use super::WrapIdentity;

/// Market trait stating the color does not have an alpha channel.
///
/// If implemented a default blanket implention of [`ColorMaybeAlpha`]
/// is provided.
pub trait AlphaNone {}

/// Access to an optional alpha channel information in a color.
///
/// Colors that implement this trait may or may not contain an alpha channel.
///
/// For colors without an alpha channel, the methods should behave as:
/// - `strip_alpha()` is a no‑op (returns `self`)
/// - `opacity()` should return `1.0`
/// - `try_alpha_ref()` and `try_alpha_mut()` should return `None`
pub trait AlphaMaybe: ColorData {
    /// The color's wrapper type to get the color without an Alpha
    /// channel if present.
    ///
    /// This is generally just [`WrapIdentity`] except in the case when
    /// self is a wrapper type like [`Alpha`](colorkit::colors::Alpha)
    /// [`AlphaPre`](colorkit::colors::AlphaPre)
    type StripAlpha: ColorWrap<Self>;
    /// Remove the alpha channel if present.
    ///
    /// Otherwise this should just return `Self`
    fn strip_alpha(self) -> <Self::StripAlpha as ColorWrap<Self>>::Inner {
        return <Self::StripAlpha as ColorWrap<Self>>::into_inner(self);
    }
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
