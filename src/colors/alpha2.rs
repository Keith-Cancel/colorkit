use core::fmt::Debug;

use colorkit::math::BoundF32;
use colorkit::num_type::*;
use colorkit::scalar::NormF32;
use colorkit::space::*;

use super::macros::*;

#[cfg(feature = "type_const")]
type Arr<S, T> = [T; <<S as ColorData>::Channels as Number>::N];
#[cfg(not(feature = "type_const"))]
type Arr<S, T> = <<S as ColorData>::Channels as Number>::Arr<T>;

#[cfg(feature = "type_const")]
type ArrInc<S, T> = [T; <<<S as ColorData>::Channels as Number>::Inc as Number>::N];
#[cfg(not(feature = "type_const"))]
type ArrInc<S, T> = <<<S as ColorData>::Channels as Number>::Inc as Number>::Arr<T>;

/// Wraps a color space with Alpha channel for transparency.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>(ArrInc<S, f32>);

impl<S: ColorSpace> Alpha<S> {
    const KIND: AlphaKind = AlphaKind::Normal;
    /// Create a new Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S, f32>::from_fn(|i| {
            if i >= S::Channels::N { alpha } else { color[i] }
        }));
    }
    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        return S::from_fn(|i| self.0[i]);
    }
}

alpha_methods!(Alpha);
alpha_traits!(Alpha);
impl_self_index!(Alpha<S: ColorSpace>);

/// A color with it's alpha premultiplied on all other channels.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AlphaPre<S: ColorSpace>(ArrInc<S, f32>);

impl<S: ColorSpace> AlphaPre<S> {
    const KIND: AlphaKind = AlphaKind::PreMul;
    /// Create a new premultiplied Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S, f32>::from_fn(|i| {
            if i >= S::Channels::N {
                alpha
            } else {
                color[i] * alpha
            }
        }));
    }

    /// Remove the alpha channel.
    pub fn strip_alpha(self) -> S {
        let alpha = self.alpha();
        if alpha == 0.0 {
            return S::from_fn(|_| 0.0);
        }
        return S::from_fn(|i| self.0[i] / alpha);
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

alpha_methods!(AlphaPre);
alpha_traits!(AlphaPre);
impl_self_index!(AlphaPre<S: ColorSpace>);

macro_rules! alpha_methods {
    ($name:ident) => {
        impl<S: ColorSpace> $name<S> {
            /// The index of the alpha channel.
            pub const INDEX: usize = S::Channels::N;
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
                // Safety:
                // The inner type is an Number::Arr or a type const array.
                return unsafe { narr_as_slice(&self.0) };
            }
            /// View the alpha color as a mutable slice
            #[inline]
            pub const fn as_mut_slice(&mut self) -> &mut [f32] {
                // Safety:
                // The inner type is an Number::Arr or a type const array.
                return unsafe { narr_as_mut_slice(&mut self.0) };
            }
            /// Set the colors alpha channel, leaving all other channels uneffected.
            #[inline]
            pub const fn set_alpha(&mut self, alpha: f32) {
                self.as_mut_slice()[Self::INDEX] = alpha;
            }

            /// Create the alpha color with all channels equal to `0.0`.
            pub const fn new_zeroed() -> Self {
                // Safety:
                // Arr and ArrInc can only be a an array, either because
                // they are Number::Arr or a type const array.
                return Self(unsafe { narr_repeat(0.0) });
            }
        }
    };
}
pub(crate) use alpha_methods;

macro_rules! alpha_traits {
    ($name:ident) => {
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
            fn into_inner(wrapper: $name<S>) -> S {
                return wrapper.strip_alpha();
            }
            fn from_inner(self, inner: S) -> $name<S> {
                return $name::<S>::new(inner, self.0);
            }
        }

        impl<S: ColorSpace> ColorSlice for $name<S> {}
    };
}
pub(crate) use alpha_traits;

const fn extend<S: ColorSpace, T: Copy + Debug + PartialEq>(array: Arr<S, T>, value: T) -> ArrInc<S, T> {
    // Safety:
    // Arr and ArrInc can only be a an array, either because
    // they are Number::Arr or a type const array.
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

/// Implements [`ColorWrap`] for [`Alpha`] and [`AlphaPre`]
///
/// The inner [`f32`] is used as the alpha channel for [`ColorWrap::from_inner`].
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
