use colorkit::convert::ColorTransmute;
use colorkit::convert::FromColor;
use colorkit::convert::IntoColor;
use colorkit::layout::Layout;
use colorkit::layout::LayoutMap;
use colorkit::math::BoundF32;
use colorkit::num_type::Number;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::space::*;

use super::Xyz;
use super::macros::*;

/// Wraps a color space with Alpha channel for transparency.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace + ColorTransmute>(S, f32);

base_funcs!(Alpha);
impl_self_index!(Alpha<S: ColorSpace + ColorTransmute>);
impl_self_as_typ!(S, Alpha<S: ColorSpace + ColorTransmute>);

impl<S: ColorSpace + ColorTransmute> Alpha<S> {
    const KIND: AlphaKind = AlphaKind::Normal;
    /// Create a new Alpha color with a color and alpha channel value.
    pub const fn new(color: S, alpha: f32) -> Self {
        return Self(color, alpha);
    }
    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        return self.0;
    }
    /// Set the colors alpha channel value.
    pub const fn set_alpha(&mut self, alpha: f32) {
        self.1 = alpha;
    }
    /// View [`Alpha`] as a reference to the underylying colorspace.
    #[inline]
    pub const fn as_color(&self) -> &S {
        return &self.0;
    }
    /// View [`Alpha`] as a mutable reference to the underylying colorspace.
    #[inline]
    pub const fn as_mut_color(&mut self) -> &mut S {
        return &mut self.0;
    }
    /// Convert to premultiplied alpha.
    pub fn into_premul_alpha(self) -> AlphaPre<S> {
        return AlphaPre::new(self.0, self.1);
    }
    /// Convert color space data while leaving alpha channel.
    pub fn into_color_alpha<S1: ColorSpace + ColorTransmute + FromColor<S>>(self) -> Alpha<S1> {
        return Alpha(self.0.into_color(), self.1);
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<Xyz<S::WhitePoint>> for Alpha<S> {
    fn from_color(color: Xyz<S::WhitePoint>) -> Self {
        return Self(color.into_color(), 1.0);
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<Alpha<S>> for Xyz<S::WhitePoint> {
    fn from_color(color: Alpha<S>) -> Self {
        return color.0.into_color();
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<Alpha<S>> for AlphaPre<S> {
    fn from_color(color: Alpha<S>) -> Self {
        return color.into_premul_alpha();
    }
}

/// A color with it's alpha premultiplied on all other channels.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlphaPre<S: ColorSpace + ColorTransmute>(S, f32);

base_funcs!(AlphaPre);
impl_self_index!(AlphaPre<S: ColorSpace + ColorTransmute>);

impl<S: ColorSpace + ColorTransmute> AlphaPre<S> {
    const KIND: AlphaKind = AlphaKind::PreMul;
    /// Create a new premultiplied Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        let mut c = color;
        for v in c.as_mut_slice() {
            *v = *v * alpha;
        }
        return Self(c, alpha);
    }
    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        let a = self.into_alpha();
        return a.strip_alpha();
    }
    /// Set the colors alpha channel, but this will not update the other channels.
    pub const fn set_alpha(&mut self, alpha: f32) {
        self.1 = alpha;
    }
    /// Set the alpha channel, and update all other channels.
    pub fn update_alpha(&mut self, alpha: f32) {
        // All channels will be zero, avoid division.
        if self.1 == 0.0 {
            return self.set_alpha(alpha);
        }
        for v in self.0.as_mut_slice() {
            *v = (*v / self.1) * alpha;
        }
    }
    /// Convert to normal alpha with no premultiplication.
    pub fn into_alpha(self) -> Alpha<S> {
        let mut color = self.0;
        let alpha = self.1;
        // All channels will be zero, avoid division.
        if alpha == 0.0 {
            return Alpha::new(color, alpha);
        }
        for v in color.as_mut_slice() {
            *v = *v / alpha;
        }
        return Alpha::new(color, alpha);
    }
    /// Convert color space data while leaving alpha channel.
    pub fn into_color_alpha<S1: ColorSpace + ColorTransmute + FromColor<S>>(self) -> AlphaPre<S1> {
        let a = self.into_alpha().into_color_alpha::<S1>();
        return a.into_premul_alpha();
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<Xyz<S::WhitePoint>> for AlphaPre<S> {
    fn from_color(color: Xyz<S::WhitePoint>) -> Self {
        return Self::new(color.into_color(), 1.0);
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<AlphaPre<S>> for Xyz<S::WhitePoint> {
    fn from_color(color: AlphaPre<S>) -> Self {
        let a = color.into_alpha();
        return a.into_color();
    }
}

impl<S: ColorSpace + ColorTransmute> FromColor<AlphaPre<S>> for Alpha<S> {
    fn from_color(color: AlphaPre<S>) -> Self {
        return color.into_alpha();
    }
}

macro_rules! base_funcs {
    ($name:ident) => {
        impl<S: ColorSpace + ColorTransmute> $name<S> {
            /// Get the colors alpha channel value.
            pub const fn alpha(&self) -> f32 {
                return self.1;
            }
            /// Maximum Alpha Channel value
            #[inline(always)]
            pub const fn alpha_max() -> f32 {
                return 1.0;
            }
            /// Minimum Alpha Channel value
            #[inline(always)]
            pub const fn alpha_min() -> f32 {
                return 0.0;
            }
            /// View the alpha color as a slice reference.
            #[inline]
            pub const fn as_slice(&self) -> &[f32] {
                // Safety:
                // The ColorSpace S is marked as transmuteable
                // We also mark the alpha wrapper as repr(C)
                // If min_const_generic_args is stabilized this
                // can be replaced with safe constructs.
                let p = self as *const _ as *const f32;
                return unsafe { core::slice::from_raw_parts(p, <Self as ColorData>::Channels::N) };
            }
            /// View the alpha color as a mutable slice
            #[inline]
            pub const fn as_mut_slice(&mut self) -> &mut [f32] {
                let p = self as *mut _ as *mut f32;
                // Safety:
                // The ColorSpace S is marked as transmuteable
                // We also mark the alpha wrapper as repr(C)
                // If min_const_generic_args is stabilized this
                // can be replaced with safe constructs.
                return unsafe { core::slice::from_raw_parts_mut(p, <Self as ColorData>::Channels::N) };
            }
        }

        impl<S: ColorSpace + ColorTransmute> AsRef<[f32]> for $name<S> {
            #[inline]
            fn as_ref(&self) -> &[f32] {
                return self.as_slice();
            }
        }

        impl<S: ColorSpace + ColorTransmute> AsMut<[f32]> for $name<S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorSpace + ColorTransmute> core::borrow::Borrow<[f32]> for $name<S> {
            #[inline]
            fn borrow(&self) -> &[f32] {
                return self.as_slice();
            }
        }

        impl<S: ColorSpace + ColorTransmute> core::borrow::BorrowMut<[f32]> for $name<S> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorSpace + ColorTransmute> Default for $name<S> {
            fn default() -> Self {
                return Self(S::DEFAULT, 1.0);
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorData for $name<S> {
            type WhitePoint = S::WhitePoint;
            type Channels = <S::Channels as Number>::Inc;
            const DEFAULT: Self = Self(S::DEFAULT, 1.0);
            const LINEAR: bool = S::LINEAR;
            const CHANNEL_MAX: &'static [BoundF32] = { Self::MAX.split_at(Self::Channels::N).0 };
            const CHANNEL_MIN: &'static [BoundF32] = { Self::MIN.split_at(Self::Channels::N).0 };
        }

        impl<S: ColorSpace + ColorTransmute> ColorNew for $name<S> {
            fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self {
                let mut f = f;
                let c = S::from_fn(|i| f(i));
                return Self(c, f(S::Channels::N));
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorLayout for $name<S> {
            fn from_layout<L: Layout>(layout: L) -> Self {
                debug_assert!(<L::Channels as Number>::N >= <Self as ColorData>::Channels::N);
                let a = layout.get_norm(S::Channels::N).get();
                return Self(S::from_layout(layout), a);
            }

            fn from_layout_map<L: Layout, M: LayoutMap<Channels = L::Channels>>(layout: L) -> Self {
                debug_assert!(<L::Channels as Number>::N >= <Self as ColorData>::Channels::N);
                let a = layout.get_norm(M::map(S::Channels::N)).get();
                return Self(S::from_layout_map::<L, M>(layout), a);
            }

            fn into_layout<L: Layout>(self, round: Rounding) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm(|i| self.get_norm(i), round);
            }

            fn into_layout_map<L: Layout, M: LayoutMap>(self, round: Rounding) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm(|i| self.get_norm(M::unmap(i)), round);
            }

            fn into_layout_dither<L: Layout, D: crate::scalar::Dither>(self, round: Rounding, dither: &mut D) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm_dither(|i| self.get_norm(i), round, dither);
            }

            fn into_layout_dither_map<L: Layout, D: crate::scalar::Dither, M: LayoutMap>(
                self,
                round: Rounding,
                dither: &mut D,
            ) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm_dither(|i| self.get_norm(M::unmap(i)), round, dither);
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorMaybeAlpha for $name<S> {
            type NoAlpha = S;
            const ALPHA_KIND: AlphaKind = Self::KIND;
            const ALPHA_INDEX: Option<usize> = Some(S::Channels::N);
            #[inline]
            fn opacity(&self) -> f32 {
                return self.1;
            }
            #[inline]
            fn strip_alpha(self) -> Self::NoAlpha {
                return self.0;
            }
            #[inline]
            fn try_alpha_mut(&mut self) -> Option<&mut f32> {
                return Some(&mut self.1);
            }
            #[inline]
            fn try_alpha_ref(&self) -> Option<&f32> {
                return Some(&self.1);
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorSlice for $name<S> {}

        impl<S: ColorSpace + ColorTransmute> ColorSpace for $name<S> {
            fn get_norm(&self, index: usize) -> NormF32 {
                if index == S::Channels::N {
                    return NormF32::new(self.1);
                }
                return self.0.get_norm(index);
            }
        }

        // Private constants
        impl<S: ColorSpace + ColorTransmute> $name<S> {
            const MAX: &'static [BoundF32] = &const {
                // Just make this larger than likely needed can't use
                // S or Self in the len of an array =(
                let mut max = [BoundF32::Unbounded; 16];
                let mut i = 0;
                while i < S::CHANNEL_MAX.len() {
                    max[i] = S::CHANNEL_MAX[i];
                    i += 1;
                }
                max[i] = BoundF32::Include(1.0);
                max
            };
            const MIN: &'static [BoundF32] = &const {
                let mut arr = [BoundF32::Unbounded; 16];
                let mut i = 0;
                while i < S::CHANNEL_MIN.len() {
                    arr[i] = S::CHANNEL_MIN[i];
                    i += 1;
                }
                arr[i] = BoundF32::Include(0.0);
                arr
            };
        }
    };
}
pub(crate) use base_funcs;

#[cfg(test)]
mod test {
    use colorkit::colors::OkLab;
    use colorkit::colors::Srgb;
    use colorkit::colors::Xyz;
    use colorkit::wp::D65;

    use super::*;

    #[test]
    fn alpha_pre() {
        // All fraction powers of 2 so results should be exact.
        let mut a = AlphaPre::new(Srgb::new(0.75, 0.5, 0.25), 0.5);
        assert_eq!(a[0], 0.375);
        assert_eq!(a[1], 0.25);
        assert_eq!(a[2], 0.125);
        assert_eq!(a[3], 0.5);

        let b = a.into_alpha();
        assert_eq!(b[0], 0.75);
        assert_eq!(b[1], 0.5);
        assert_eq!(b[2], 0.25);
        assert_eq!(b[3], 0.5);

        a.update_alpha(0.25);
        assert_eq!(a[0], 0.1875);
        assert_eq!(a[1], 0.125);
        assert_eq!(a[2], 0.0625);
        assert_eq!(a[3], 0.5);
    }

    #[test]
    fn min_max() {
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MIN.len(), 4);

        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[0], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[1], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[2], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[3], BoundF32::Include(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[0], BoundF32::Include(1.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[1], BoundF32::Include(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[2], BoundF32::Include(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[3], BoundF32::Include(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[0], BoundF32::Include(0.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[1], BoundF32::Include(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[2], BoundF32::Include(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[3], BoundF32::Include(0.0));
    }

    #[test]
    fn from_fn() {
        let arr = [0.125f32, 0.25, 0.375, 0.5];
        let c = <Alpha<Srgb>>::from_fn(|i| arr[i]);
        let r = &c;

        assert_eq!(c[0], 0.125);
        assert_eq!(c[1], 0.25);
        assert_eq!(c[2], 0.375);
        assert_eq!(c[3], 0.5);

        assert_eq!(r.as_color().red(), 0.125);
    }
}
