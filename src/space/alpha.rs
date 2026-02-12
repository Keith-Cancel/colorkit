use super::ColorData;
use super::ColorSpace;
use super::ColorWrap;
use super::WrapIdentity;

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
pub trait AlphaNone {}

/// Access to an optional alpha channel information in a color.
///
/// Colors that implement this trait may or may not contain an alpha channel.
///
/// For colors without an alpha channel, the methods should behave as:
/// - [`strip_alpha()`](AlphaMaybe::strip_alpha) is a no‑op (e.g returns `Self`)
/// - [`opacity()`](AlphaMaybe::opacity) should return `1.0`
/// - [`try_alpha_ref()`](AlphaMaybe::try_alpha_ref) and
///   [`try_alpha_mut()`](AlphaMaybe::try_alpha_mut) should return `None`
///
/// If a color implements [`AlphaNone`] an implemntation of this trait
/// will be provided these properties already.
pub trait AlphaMaybe: ColorData {
    /// The kinda of Alpha Channel the color space has
    const ALPHA_KIND: AlphaKind;
    /// If the color has an alpha channel the index of the channel.
    const ALPHA_INDEX: Option<usize>;
    /// The color's wrapper type to get the color without an Alpha
    /// channel if present.
    ///
    /// This is generally just [`WrapIdentity`] except in the case when
    ///
    /// For colors that do not have an alpha channel, this should be
    /// [`WrapIdentity`]. The conversion then becomes a no-op, and
    /// `Inner` is simply `Self`.
    ///
    /// Colors like [`Alpha`](colorkit::colors::Alpha) and
    /// [`AlphaPre`](colorkit::colors::AlphaPre) will have a differnt
    /// marker type providing methods to wrap and unwrap.
    type AlphaWrap: ColorWrap<Self>;
    /// Remove the alpha channel if present.
    ///
    /// Otherwise this should just return `Self`
    fn strip_alpha(self) -> <Self::AlphaWrap as ColorWrap<Self>>::Inner {
        return <Self::AlphaWrap as ColorWrap<Self>>::into_inner(self);
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

impl<S: ColorSpace + AlphaNone> AlphaMaybe for S {
    type AlphaWrap = WrapIdentity;
    const ALPHA_INDEX: Option<usize> = None;
    const ALPHA_KIND: AlphaKind = AlphaKind::None;
    #[inline]
    fn opacity(&self) -> f32 {
        return 1.0;
    }
    fn try_alpha_mut(&mut self) -> Option<&mut f32> {
        return None;
    }
    fn try_alpha_ref(&self) -> Option<&f32> {
        return None;
    }
}
