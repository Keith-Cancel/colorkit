//! Conversion Traits between color spaces.
use colorkit::colors::Xyz;
use colorkit::math::matrix_3x3_vec3_mul;
use colorkit::space::ColorArray;
use colorkit::space::ColorData;
use colorkit::space::ColorSpace;

/// This marker trait marks that a color can be
/// transmuted into an array of [f32; [`ColorArray::CHANNELS`]]
///
/// Essentially `size_of::<Self>() / size_of::<f32>()` should
/// equal [`ColorArray::CHANNELS`], plus other constraints like
/// alignment ect...
pub unsafe trait ColorTransmute: ColorSpace {}

/// This trait converts from one [ColorSpace] into an other.
///
/// Also unlike the [core::convert::From] it may be lossy
/// depending source or target color space.
///
/// Much like core's `From`, this is the reciprocal of [`IntoColor`].
pub trait FromColor<C>: Sized {
    /// Convert to this [`ColorSpace`] from the input color.
    fn from_color(color: C) -> Self;
}

// Blanket implentation for Self.
impl<C> FromColor<C> for C {
    #[inline]
    fn from_color(color: C) -> Self {
        return color;
    }
}

/// This trait converts from one [ColorSpace] into an other.
///
/// Also unlike the [core::convert::Into] it may be lossy
/// depending source or target color space.
///
/// Much like core's `Into`, this is the reciprocal of [`FromColor`].
pub trait IntoColor<C> {
    /// Convert this color into the target [`ColorSpace`]
    fn into_color(self) -> C;
}

// Blanket impl of IntoColor for any implementation of FromColor
impl<C1, C2: FromColor<C1>> IntoColor<C2> for C1 {
    fn into_color(self) -> C2 {
        return C2::from_color(self);
    }
}

/// Marker trait stating conversion from `Self` <-> `C` exists both ways.
pub trait FromColorBoth<C>: FromColor<C> + private::FromBound<C, Other: FromColor<Self>, Other = C> {}

// Blanket Implentation if From is defined for both color spaces.
impl<C1: FromColor<C2>, C2: FromColor<C1>> FromColorBoth<C2> for C1 {}

// After seearching for awhile on how get rid of redundant where clauses
// I found the following:
// https://github.com/rust-lang/rust/issues/44491#issuecomment-2496196742
// This allow me to avoid to drag a where clause around everywhere, but still
// in my color space trait specify Xyz<Wp>: From<Self> instead of just Into
mod private {
    pub trait FromBound<C> {
        type Other;
    }
    impl<C1, C2> FromBound<C2> for C1 {
        type Other = C2;
    }
}

/// Transformation Matrices to go between and from CIE XYZ
///
/// Expects that color space has a channel count of 3.
// TODO
// Maybe add my number item back to some these traits
// since I can't do associated const equality in stable.
// Mainly the length thing.
// Ideally pub trait XyzMatrices: ColorData<CHANNELS = 3>
pub trait XyzMatrices: ColorData {
    // Looks like people generally represent these as a transformation matrix.
    // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
    /// 3x3 Matrix to convert into XYZ
    const INTO_XYZ: [f32; 9];
    /// 3x3 Matrix to convert from XYZ
    const FROM_XYZ: [f32; 9];
}

impl<C: ColorArray + XyzMatrices> FromColor<Xyz<C::WhitePoint>> for C {
    fn from_color(color: Xyz<C::WhitePoint>) -> Self {
        debug_assert!(C::CHANNELS == 3);
        let c = matrix_3x3_vec3_mul(&Self::FROM_XYZ, color.as_slice());
        return Self::from_fn(|i| c[i]);
    }
}

impl<C: ColorArray + XyzMatrices> FromColor<C> for Xyz<C::WhitePoint> {
    fn from_color(color: C) -> Self {
        debug_assert!(C::CHANNELS == 3);
        return Xyz::from_array(matrix_3x3_vec3_mul(&C::INTO_XYZ, color.as_slice()));
    }
}
