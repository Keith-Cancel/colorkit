use colorkit::num_type::*;
use colorkit::space::*;

use super::macros::*;

#[cfg(feature = "type_const")]
type ChanArr<S> = [f32; <<S as ColorData>::Channels as Number>::N];
#[cfg(not(feature = "type_const"))]
type ChanArr<S> = <<S as ColorData>::Channels as Number>::Arr<f32>;

#[cfg(feature = "type_const")]
type ChanArrInc<S> = [f32; <<<S as ColorData>::Channels as Number>::Inc as Number>::N];
#[cfg(not(feature = "type_const"))]
type ChanArrInc<S> = <<<S as ColorData>::Channels as Number>::Inc as Number>::Arr<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>(ChanArrInc<S>);

alpha_traits!(Alpha);
impl_self_index!(Alpha<S: ColorSpace>);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AlphaPre<S: ColorSpace>(ChanArrInc<S>);

alpha_traits!(AlphaPre);
impl_self_index!(AlphaPre<S: ColorSpace>);

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
    };
}
pub(crate) use alpha_traits;
