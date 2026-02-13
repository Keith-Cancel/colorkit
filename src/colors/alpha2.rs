use colorkit::math::BoundF32;
use colorkit::num_type::*;
use colorkit::space::*;

use super::macros::*;

#[cfg(feature = "type_const")]
type Arr<S> = [f32; <<S as ColorData>::Channels as Number>::N];
#[cfg(not(feature = "type_const"))]
type Arr<S> = <<S as ColorData>::Channels as Number>::Arr<f32>;

#[cfg(feature = "type_const")]
type ArrInc<S> = [f32; <<<S as ColorData>::Channels as Number>::Inc as Number>::N];
#[cfg(not(feature = "type_const"))]
type ArrInc<S> = <<<S as ColorData>::Channels as Number>::Inc as Number>::Arr<f32>;

/// Wraps a color space with Alpha channel for transparency.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>(ArrInc<S>);

impl<S: ColorSpace> Alpha<S> {
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S>::from_fn(|i| {
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
pub struct AlphaPre<S: ColorSpace>(ArrInc<S>);

impl<S: ColorSpace> AlphaPre<S> {
    pub fn new(color: S, alpha: f32) -> Self {
        return Self(ArrInc::<S>::from_fn(|i| {
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
            /// Get the colors alpha channel value.
            pub fn alpha(&self) -> f32 {
                return self.0[S::Channels::N];
            }

            const MIN_MAX: ([BoundF32; 16], [BoundF32; 16]) = {
                use BoundF32::*;
                // Just make this larger than likely needed can't use
                // S or Self in the len of an array in stable =(
                let mut out = ([Unbounded; 16], [Unbounded; 16]);
                let mut i = 0;
                while i < S::CHANNEL_MAX.len() {
                    out.0[i] = S::CHANNEL_MIN[i];
                    out.1[i] = S::CHANNEL_MAX[i];
                    i += 1;
                }
                out.0[i] = Include(0.0);
                out.1[i] = Include(1.0);
                out
            };
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
            const CHANNEL_MAX: &'static [BoundF32] = { Self::MIN_MAX.1.split_at(Self::Channels::N).0 };
            const CHANNEL_MIN: &'static [BoundF32] = { Self::MIN_MAX.0.split_at(Self::Channels::N).0 };
        }

        impl<S: ColorSpace> ColorNew for $name<S> {
            fn from_fn<F: FnMut(usize) -> f32>(fun: F) -> Self {
                return Self(ArrInc::<S>::from_fn(fun));
            }
        }

        impl<S: ColorSpace> ColorSlice for $name<S> {}
    };
}
pub(crate) use alpha_traits;
