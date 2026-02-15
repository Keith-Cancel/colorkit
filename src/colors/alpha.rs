use core::fmt::Debug;

use colorkit::convert::*;
use colorkit::layout::*;
use colorkit::math::BoundF32;
use colorkit::num_type::*;
use colorkit::scalar::*;
use colorkit::space::*;

use super::Xyz;
use super::macros::*;

#[cfg(feature = "type_const")]
type ArrInc<S, T> = [T; <<<S as ColorData>::Channels as Number>::Inc as Number>::N];
#[cfg(not(feature = "type_const"))]
type ArrInc<S, T> = <<<S as ColorData>::Channels as Number>::Inc as Number>::Arr<T>;

/// Wraps a color space with Alpha channel for transparency.
#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>(ArrInc<S, f32>);

alpha_methods!(Alpha);
alpha_traits!(Alpha);
impl_self_index!(Alpha<S: ColorSpace>);

impl<S: ColorSpace> Alpha<S> {
    const KIND: AlphaKind = AlphaKind::Normal;
    /// Create a new Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S, f32>::from_fn(|i| {
            if i == Self::INDEX { alpha } else { color[i] }
        }));
    }
    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        return S::from_fn(|i| self[i]);
    }
    /// Convert the alpha color to a pre-multiplied alpha color.
    pub const fn premultiply(mut self) -> AlphaPre<S> {
        let alpha = self.alpha();
        let slc = self.as_mut_slice();
        let mut i = 0;
        while i < Self::INDEX {
            slc[i] *= alpha;
            i += 1;
        }
        return AlphaPre(self.0);
    }
}

impl<S: ColorSpace> ColorBounds for Alpha<S> {
    fn clamp(mut self) -> Self {
        for i in 0..Self::INDEX {
            self[i] = BoundF32::clamp(S::CHANNEL_MIN[i], S::CHANNEL_MAX[i], self[i]);
        }
        self[Self::INDEX] = self[Self::INDEX].clamp(0.0, 1.0);
        return self;
    }

    fn clamp_channel(mut self, index: usize) -> Self {
        if index == Self::INDEX {
            self[Self::INDEX] = self[Self::INDEX].clamp(0.0, 1.0);
            return self;
        }
        self[index] = BoundF32::clamp(S::CHANNEL_MIN[index], S::CHANNEL_MAX[index], self[index]);
        return self;
    }

    fn is_clamped(&self) -> bool {
        for i in 0..Self::INDEX {
            if !BoundF32::in_bounds(S::CHANNEL_MIN[i], S::CHANNEL_MAX[i], self[i]) {
                return false;
            }
        }
        if self[Self::INDEX] < 0.0 || self[Self::INDEX] > 1.0 {
            return false;
        }
        return true;
    }

    fn is_channel_clamped(&self, index: usize) -> bool {
        if index == Self::INDEX {
            return self[Self::INDEX] >= 0.0 && self[Self::INDEX] <= 1.0;
        }
        return BoundF32::in_bounds(S::CHANNEL_MIN[index], S::CHANNEL_MAX[index], self[index]);
    }
}

impl<S: ColorSpace> ColorNorm for Alpha<S> {
    fn into_norm(self) -> ColorArray<Self, NormF32> {
        let a = self.alpha();
        let c = self.strip_alpha().into_norm();
        return extend::<S, _>(c, NormF32::new(a));
    }
    fn from_norm<T: AsRef<[NormF32]>>(values: T) -> Self {
        let v = values.as_ref();
        let a = v[Self::INDEX].get();
        let c = S::from_norm(&v[..v.len() - 1]);
        return Self::new(c, a);
    }
}

/// A color with it's alpha premultiplied on all other channels.
#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct AlphaPre<S: ColorSpace>(ArrInc<S, f32>);

alpha_methods!(AlphaPre);
alpha_traits!(AlphaPre);
impl_self_index!(AlphaPre<S: ColorSpace>);

impl<S: ColorSpace> AlphaPre<S> {
    const KIND: AlphaKind = AlphaKind::PreMul;
    /// Create a new premultiplied Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S, f32>::from_fn(|i| {
            if i == Self::INDEX {
                alpha
            } else {
                color[i] * alpha
            }
        }));
    }

    /// Convert to normal alpha with no premultiplication.
    pub const fn into_alpha(mut self) -> Alpha<S> {
        let alpha = self.alpha();
        // All channels will be zero, avoid division.
        if alpha == 0.0 {
            return <Alpha<S>>::new_zeroed();
        }
        let slc = self.as_mut_slice();
        let mut i = 0;
        while i < Self::INDEX {
            slc[i] /= alpha;
            i += 1;
        }
        return Alpha(self.0);
    }

    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        let alpha = self.alpha();
        if alpha == 0.0 {
            return S::from_fn(|_| 0.0);
        }
        return S::from_fn(|i| self[i] / alpha);
    }

    /// Set the alpha channel, and update all other channels.
    pub const fn update_alpha(&mut self, alpha: f32) {
        // All channels will be zero, avoid division.
        let old = self.alpha();
        if old == 0.0 {
            return self.set_alpha(alpha);
        }
        let slc = self.as_mut_slice();
        let mut i = 0;
        while i < slc.len() {
            slc[i] = (slc[i] / old) * alpha;
            i += 1;
        }
    }
}

impl<S: ColorSpace> ColorBounds for AlphaPre<S> {
    fn clamp(self) -> Self {
        return self.into_alpha().clamp().premultiply();
    }

    fn clamp_channel(self, index: usize) -> Self {
        return self.into_alpha().clamp_channel(index).premultiply();
    }

    fn is_clamped(&self) -> bool {
        return self.into_alpha().is_clamped();
    }

    fn is_channel_clamped(&self, index: usize) -> bool {
        let alpha = self.alpha();
        if index == Self::INDEX {
            return (0.0..=1.0).contains(&alpha);
        }
        let value = if alpha == 0.0 {
            self[index]
        } else {
            self[index] / alpha
        };
        return BoundF32::in_bounds(S::CHANNEL_MIN[index], S::CHANNEL_MAX[index], value);
    }
}

impl<S: ColorSpace> ColorNorm for AlphaPre<S> {
    fn into_norm(self) -> ColorArray<Self, NormF32> {
        return self.into_alpha().into_norm();
    }
    fn from_norm<T: AsRef<[NormF32]>>(values: T) -> Self {
        return Alpha::<S>::from_norm(values).premultiply();
    }
}

macro_rules! alpha_methods {
    ($name:ident) => {
        impl<S: ColorSpace> $name<S> {
            /// The index of the alpha channel.
            pub const INDEX: usize = S::Channels::N;

            /// Create alpha color that is fully opaque.
            pub fn new_opaque(color: S) -> Self {
                return Self(ArrInc::<S, f32>::from_fn(|i| {
                    if i == Self::INDEX { 1.0 } else { color[i] }
                }));
            }
            /// Split/break the alpha color into two parts.
            pub fn into_parts(self) -> (S, f32) {
                let a = self.alpha();
                return (self.strip_alpha(), a);
            }
            /// Get the colors alpha channel value.
            #[inline]
            pub const fn alpha(&self) -> f32 {
                return self.as_slice()[Self::INDEX];
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
                return as_slice::<S, _>(&self.0);
            }
            /// View the alpha color as a mutable slice
            #[inline]
            pub const fn as_mut_slice(&mut self) -> &mut [f32] {
                return as_mut_slice::<S, _>(&mut self.0);
            }
            /// Set the colors alpha channel, leaving all other channels uneffected.
            #[inline]
            pub const fn set_alpha(&mut self, alpha: f32) {
                self.as_mut_slice()[Self::INDEX] = alpha;
            }

            /// Create the alpha color with all channels equal to `0.0`.
            pub const fn new_zeroed() -> Self {
                return Self(repeat::<S, _>(0.0));
            }

            /// Convert the wrapped color space while preserving the alpha channel.
            pub fn into_color_alpha<S1: ColorSpace>(self) -> Alpha<S1>
            where
                S: IntoColor<S1>,
            {
                let alpha = self.alpha();
                let color: S1 = self.strip_alpha().into_color();
                return Alpha::<S1>::new(color, alpha);
            }
        }
    };
}
pub(crate) use alpha_methods;

macro_rules! alpha_traits {
    ($name:ident) => {
        // derive copy and clone put a copy bound
        // and clone bound on S, but unnessicaly
        // restricts copy and clone
        impl<S: ColorSpace> Copy for $name<S> {}

        #[allow(clippy::non_canonical_clone_impl)]
        impl<S: ColorSpace> Clone for $name<S> {
            #[inline]
            fn clone(&self) -> Self {
                return *self;
            }
            #[inline]
            fn clone_from(&mut self, src: &Self) {
                *self = *src;
            }
        }

        impl<S: ColorSpace> AsRef<[f32]> for $name<S> {
            #[inline]
            fn as_ref(&self) -> &[f32] {
                return (&self.0).as_ref();
            }
        }
        impl<S: ColorSpace> AsMut<[f32]> for $name<S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [f32] {
                return (&mut self.0).as_mut();
            }
        }

        impl<S: ColorSpace> Default for $name<S> {
            fn default() -> Self {
                return Self::new(S::default(), 1.0);
            }
        }

        impl<S: ColorSpace> AlphaMaybe for $name<S> {
            type AlphaWrap = AlphaWrap;
            const ALPHA_KIND: AlphaKind = Self::KIND;
            const ALPHA_INDEX: Option<usize> = Some(Self::INDEX);
            fn strip_alpha(self) -> S {
                return self.strip_alpha();
            }
            #[inline]
            fn opacity(&self) -> f32 {
                return self.alpha();
            }
            #[inline]
            fn try_alpha_ref(&self) -> Option<&f32> {
                return Some(&self[Self::INDEX]);
            }
            #[inline]
            fn try_alpha_mut(&mut self) -> Option<&mut f32> {
                return Some(&mut self[Self::INDEX]);
            }
        }

        impl<S: ColorSpace> ColorData for $name<S> {
            type Channels = <S::Channels as Number>::Inc;
            type WhitePoint = S::WhitePoint;
            const LINEAR: bool = S::LINEAR;
            const CHANNEL_MAX: ArrInc<S, BoundF32> =
                extend::<S, _>(S::CHANNEL_MAX, BoundF32::Include(1.0));
            const CHANNEL_MIN: ArrInc<S, BoundF32> =
                extend::<S, _>(S::CHANNEL_MIN, BoundF32::Include(0.0));
        }

        impl<S: ColorSpace> ColorNew for $name<S> {
            fn from_fn<F: FnMut(usize) -> f32>(fun: F) -> Self {
                return Self(ArrInc::<S, f32>::from_fn(fun));
            }
        }

        impl<S: ColorSpace> ColorWrap<$name<S>> for AlphaWrap {
            type Inner = S;
            fn unwrap_inner(wrapper: $name<S>) -> S {
                return wrapper.strip_alpha();
            }
            fn wrap_inner(self, inner: S) -> $name<S> {
                return $name::<S>::new(inner, self.0);
            }
        }

        impl<S: ColorSpace> ColorLayout for $name<S> {
            fn from_layout<L: Layout>(layout: L) -> Self {
                let alpha = layout.get_norm(Self::INDEX).get();
                let color = S::from_layout(layout);
                return Self::new(color, alpha);
            }
            fn into_layout<L: Layout<Channels = Self::Channels>>(self, round: Rounding) -> L {
                let n = self.into_norm();
                return L::from_fn_norm(|i| n[i], round);
            }
            fn into_layout_dither<L: Layout<Channels = Self::Channels>, D: Dither>(
                self,
                round: Rounding,
                dither: &mut D,
            ) -> L {
                let n = self.into_norm();
                return L::from_fn_norm_dither(|i| n[i], round, dither);
            }
        }

        impl<S: ColorSpace> FromColor<Xyz<S::WhitePoint>> for $name<S> {
            fn from_color(color: Xyz<S::WhitePoint>) -> Self {
                let c: S = color.into_color();
                return Self::new(c, 1.0);
            }
        }

        impl<S: ColorSpace> FromColor<$name<S>> for Xyz<S::WhitePoint> {
            fn from_color(color: $name<S>) -> Self {
                let c = color.strip_alpha();
                return c.into_color();
            }
        }

        impl<S: ColorSpace> ColorSlice for $name<S> {}
        impl<S: ColorSpace> ColorSpace for $name<S> {}
    };
}
pub(crate) use alpha_traits;

impl<S: ColorSpace> FromColor<Alpha<S>> for AlphaPre<S> {
    fn from_color(color: Alpha<S>) -> Self {
        return color.premultiply();
    }
}

impl<S: ColorSpace> FromColor<AlphaPre<S>> for Alpha<S> {
    fn from_color(color: AlphaPre<S>) -> Self {
        return color.into_alpha();
    }
}

const fn extend<S: ColorSpace, T: Copy + Debug + PartialEq>(
    array: ColorArray<S, T>,
    value: T,
) -> ArrInc<S, T> {
    #[cfg(feature = "type_const")]
    {
        let mut ret = [value; <<S::Channels as Number>::Inc as Number>::N];
        let mut i = 0;
        while i < array.len() {
            ret[i] = array[i];
            i += 1;
        }
        return ret;
    }
    // Safety:
    // Arr and ArrInc can only be a an array, either because
    // they are Number::Arr or a type const array.
    #[cfg(not(feature = "type_const"))]
    unsafe {
        let mut ret: ArrInc<S, T> = narr_repeat(value);
        let dst = narr_as_mut_slice(&mut ret);
        let src = narr_as_slice(&array);
        let mut i = 0;
        while i < src.len() {
            dst[i] = src[i];
            i += 1;
        }
        return ret;
    }
}

const fn repeat<S: ColorSpace, T: Copy + Debug + PartialEq>(value: T) -> ArrInc<S, T> {
    #[cfg(feature = "type_const")]
    return [value; <<S::Channels as Number>::Inc as Number>::N];
    // Safety:
    // Arr and ArrInc can only be a an array, either because
    // they are Number::Arr or a type const array.
    #[cfg(not(feature = "type_const"))]
    return unsafe { narr_repeat(value) };
}

#[inline]
const fn as_slice<S: ColorSpace, T: Copy + Debug + PartialEq>(array: &ArrInc<S, T>) -> &[T] {
    #[cfg(feature = "type_const")]
    return array;
    // Safety:
    // The inner type is an Number::Arr or a type const array
    #[cfg(not(feature = "type_const"))]
    return unsafe { narr_as_slice(array) };
}

#[inline]
const fn as_mut_slice<S: ColorSpace, T: Copy + Debug + PartialEq>(array: &mut ArrInc<S, T>) -> &mut [T] {
    #[cfg(feature = "type_const")]
    return array;
    // Safety:
    // The inner type is an Number::Arr or a type const array
    #[cfg(not(feature = "type_const"))]
    return unsafe { narr_as_mut_slice(array) };
}

/// Implements [`ColorWrap`] for [`Alpha`] and [`AlphaPre`]
///
/// The inner [`f32`] is used as the alpha channel for [`ColorWrap::wrap_inner`].
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct AlphaWrap(pub f32);

impl_from_inner!(f32, AlphaWrap);
impl_typ_as_self!(AlphaWrap, f32);
impl_typ_as_self!(AlphaWrap, NormF32);

impl From<NormF32> for AlphaWrap {
    #[inline]
    fn from(value: NormF32) -> Self {
        return Self(value.get());
    }
}

impl From<AlphaWrap> for NormF32 {
    #[inline]
    fn from(value: AlphaWrap) -> Self {
        return NormF32::new(value.0);
    }
}

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
        assert_eq!(a[3], 0.25);
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
        assert_eq!(c[0], 0.125);
        assert_eq!(c[1], 0.25);
        assert_eq!(c[2], 0.375);
        assert_eq!(c[3], 0.5);
    }

    #[test]
    fn get_norm() {
        let a0 = Alpha::new(Srgb::new_u8(64, 128, 192), 0.75);
        let a1 = a0.premultiply();
        let n0 = a0.into_norm();
        let n1 = a1.into_norm();

        assert_eq!(n0, n1);
    }
}
