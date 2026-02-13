use core::fmt::Debug;

use colorkit::math::BoundF32;
use colorkit::num_type::*;
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
    /// Create a new Alpha color with a color and alpha channel value.
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S, f32>::from_fn(|i| {
            if i >= S::Channels::N { alpha } else { color[i] }
        }));
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
}

alpha_methods!(AlphaPre);
alpha_traits!(AlphaPre);
impl_self_index!(AlphaPre<S: ColorSpace>);

macro_rules! alpha_methods {
    ($name:ident) => {
        impl<S: ColorSpace> $name<S> {
            /// The index of the alpha channel.
            pub const ALPHA_INDEX: usize = S::Channels::N;
            /// Get the colors alpha channel value.
            #[inline]
            pub const fn alpha(&self) -> f32 {
                return self.as_slice()[Self::ALPHA_INDEX];
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
                self.as_mut_slice()[Self::ALPHA_INDEX] = alpha;
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
