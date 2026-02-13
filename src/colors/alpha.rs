use colorkit::convert::*;
use colorkit::layout::Layout;
use colorkit::math::BoundF32;
use colorkit::num_type::*;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::space::*;

use super::Xyz;
use super::macros::*;

#[cfg(feature = "type_const")]
type ArrInc<S, T> = [T; <<<S as ColorData>::Channels as Number>::Inc as Number>::N];
#[cfg(not(feature = "type_const"))]
type ArrInc<S, T> = <<<S as ColorData>::Channels as Number>::Inc as Number>::Arr<T>;

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

impl<S: ColorSpace + ColorTransmute> ColorBounds for Alpha<S> {
    fn clamp(self) -> Self {
        let alpha = self.1.clamp(0.0, 1.0);
        let color = self.0.clamp();
        return Self::new(color, alpha);
    }
    fn clamp_channel(self, index: usize) -> Self {
        if index == S::Channels::N {
            return Self::new(self.0, self.1.clamp(0.0, 1.0));
        }
        return Self::new(self.0.clamp_channel(index), self.1);
    }
    fn is_clamped(&self) -> bool {
        if self.1 < 0.0 || self.1 >= 1.0 {
            return false;
        }
        return self.0.is_clamped();
    }
    fn is_channel_clamped(&self, index: usize) -> bool {
        if index == S::Channels::N {
            let a = self.1;
            return a >= 0.0 && a <= 1.0;
        }
        return self.0.is_channel_clamped(index);
    }
    fn get_norm(&self, index: usize) -> NormF32 {
        if index == S::Channels::N {
            return NormF32::new(self.1);
        }
        return self.0.get_norm(index);
    }
    fn get_norm_bounds(&self, index: usize) -> (f32, f32) {
        if index == S::Channels::N {
            return (0.0, 1.0);
        }
        return self.0.get_norm_bounds(index);
    }
    fn get_norm_bounded(&self, index: usize, min: f32, max: f32) -> NormF32 {
        let value = self.as_slice()[index];
        return NormF32::with_bounds(value, min, max);
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

impl<S: ColorSpace + ColorTransmute> ColorBounds for AlphaPre<S> {
    fn clamp(self) -> Self {
        let color = self.into_alpha().clamp();
        return color.into_premul_alpha();
    }
    fn clamp_channel(self, index: usize) -> Self {
        let color = self.into_alpha().clamp_channel(index);
        return color.into_premul_alpha();
    }
    fn is_clamped(&self) -> bool {
        if self.1 < 0.0 || self.1 >= 1.0 {
            return false;
        }
        for i in 0..S::Channels::N {
            if !self.is_channel_clamped(i) {
                return false;
            }
        }
        return true;
    }
    fn is_channel_clamped(&self, index: usize) -> bool {
        let mut value = self.as_slice()[index];
        if self.1 != 0.0 && index != S::Channels::N {
            value /= self.1;
        }
        if !BoundF32::in_bounds(S::CHANNEL_MIN[index], S::CHANNEL_MAX[index], value) {
            return false;
        }
        return true;
    }
    fn get_norm(&self, index: usize) -> NormF32 {
        if index == S::Channels::N {
            return NormF32::new(self.1);
        }
        let b = self.get_norm_bounds(index);
        return self.get_norm_bounded(index, b.0, b.1);
    }
    fn get_norm_bounds(&self, index: usize) -> (f32, f32) {
        if index == S::Channels::N {
            return (0.0, 1.0);
        }
        return self.0.get_norm_bounds(index);
    }
    fn get_norm_bounded(&self, index: usize, min: f32, max: f32) -> NormF32 {
        let mut value = self.as_slice()[index];
        // Don't divide by zero.
        if self.1 == 0.0 {
            return NormF32::with_bounds(0.0, min, max);
        }
        if index != S::Channels::N {
            value /= self.1;
        }
        return NormF32::with_bounds(value, min, max);
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AlphaWrap(f32);

impl From<f32> for AlphaWrap {
    #[inline]
    fn from(value: f32) -> Self {
        return Self(value);
    }
}

impl From<NormF32> for AlphaWrap {
    #[inline]
    fn from(value: NormF32) -> Self {
        return Self(value.get());
    }
}

impl From<AlphaWrap> for f32 {
    #[inline]
    fn from(value: AlphaWrap) -> Self {
        return value.0;
    }
}

impl From<AlphaWrap> for NormF32 {
    #[inline]
    fn from(value: AlphaWrap) -> Self {
        return NormF32::new(value.0);
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
                return Self(S::default(), 1.0);
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorData for $name<S> {
            type WhitePoint = S::WhitePoint;
            type Channels = <S::Channels as Number>::Inc;
            const LINEAR: bool = S::LINEAR;
            const CHANNEL_MAX: ArrInc<S, BoundF32> = Self::MAX;
            const CHANNEL_MIN: ArrInc<S, BoundF32> = Self::MIN;
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

            fn into_layout<L: Layout>(self, round: Rounding) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm(|i| self.get_norm(i), round);
            }

            fn into_layout_dither<L: Layout, D: crate::scalar::Dither>(
                self,
                round: Rounding,
                dither: &mut D,
            ) -> L {
                debug_assert!(<L::Channels as Number>::N == <Self as ColorData>::Channels::N);
                return L::from_fn_norm_dither(|i| self.get_norm(i), round, dither);
            }
        }

        impl<S: ColorSpace + ColorTransmute> ColorWrap<$name<S>> for AlphaWrap {
            type Inner = S;
            fn into_inner(wrapper: $name<S>) -> S {
                return wrapper.0;
            }
            fn from_inner(self, inner: S) -> $name<S> {
                return $name::<S>::new(inner, self.0);
            }
        }

        impl<S: ColorSpace + ColorTransmute> AlphaMaybe for $name<S> {
            type AlphaWrap = AlphaWrap;
            const ALPHA_KIND: AlphaKind = Self::KIND;
            const ALPHA_INDEX: Option<usize> = Some(S::Channels::N);

            #[inline]
            fn opacity(&self) -> f32 {
                return self.1;
            }

            #[inline]
            fn strip_alpha(self) -> S {
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
        impl<S: ColorSpace + ColorTransmute> ColorSpace for $name<S> {}

        // Private constants
        impl<S: ColorSpace + ColorTransmute> $name<S> {
            const MAX: ArrInc<S, BoundF32> = const {
                // Safety:
                // The NumArray is from a number so it can only be an array.
                let max_src = S::CHANNEL_MAX;
                let mut max_dst: ArrInc<S, BoundF32> = unsafe { narr_repeat(BoundF32::Include(1.0)) };

                let src = unsafe { narr_as_slice(&max_src) };
                let dst = unsafe { narr_as_mut_slice(&mut max_dst) };

                let mut i = 0;
                while i < src.len() {
                    dst[i] = src[i];
                    i += 1;
                }
                max_dst
            };
            const MIN: ArrInc<S, BoundF32> = const {
                // Safety:
                // The NumArray is from a number so it can only be an array.
                let min_src = S::CHANNEL_MIN;
                let mut min_dst: ArrInc<S, BoundF32> = unsafe { narr_repeat(BoundF32::Include(0.0)) };

                let src = unsafe { narr_as_slice(&min_src) };
                let dst = unsafe { narr_as_mut_slice(&mut min_dst) };

                let mut i = 0;
                while i < src.len() {
                    dst[i] = src[i];
                    i += 1;
                }
                min_dst
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

    #[test]
    fn get_norm() {
        let a0 = Alpha::new(Srgb::new_u8(64, 128, 192), 0.75);
        let a1 = a0.into_premul_alpha();

        assert_eq!(a0.get_norm(3), a1.get_norm(3));
        assert_eq!(a0.get_norm(0), a1.get_norm(0));
        assert_eq!(a0.get_norm(1), a1.get_norm(1));
        assert_eq!(a0.get_norm(2), a1.get_norm(2));
    }
}
