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

/// Converts CIE XYZ and into an other Color Spaces.
pub trait FromXyz: ColorData {
    /// Convert a CIE XYZ color into this color Space.
    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self;
}

/// Conversion between CIE XYZ and other Color Spaces.
pub trait XyzConvert: ColorData {
    /// Convert a color into CIE XYZ with it's white point.
    fn into_xyz(self) -> Xyz<Self::WhitePoint>;
    /// Convert a color from CIE XYZ into this color Space.
    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self;
}

/// This trait converts from one [ColorSpace] into an other.
///
/// Also unlike the [core::convert::From] it may be lossy
/// depending source or target color space.
///
/// Much like core's `From`, this is the reciprocal of [`IntoColor`].
pub trait FromColor<C: ColorData>: ColorData {
    /// Convert to this [`ColorSpace`] from the input color.
    fn from_color(color: C) -> Self;
}

/// This trait converts from one [ColorSpace] into an other.
///
/// Also unlike the [core::convert::Into] it may be lossy
/// depending source or target color space.
///
/// Much like core's `Into`, this is the reciprocal of [`FromColor`].
pub trait IntoColor<C: ColorData>: ColorData {
    /// Convert this color into the target [`ColorSpace`]
    fn into_color(self) -> C;
}

// Blanket impl of IntoColor for any implementation of FromColor
impl<C1: ColorData, C2: FromColor<C1>> IntoColor<C2> for C1 {
    fn into_color(self) -> C2 {
        return C2::from_color(self);
    }
}

/// Transformation Matrices to go between and from CIE XYZ
// TODO
// Maybe add my number item back to some these traits
// since I can't do associated const equality in stable.
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

impl<T: ColorArray + XyzMatrices> XyzConvert for T {
    fn into_xyz(self) -> Xyz<Self::WhitePoint> {
        debug_assert!(T::CHANNELS == 3);
        return Xyz::from_array(matrix_3x3_vec3_mul(&Self::INTO_XYZ, self.as_slice()));
    }

    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self {
        debug_assert!(T::CHANNELS == 3);
        let c = matrix_3x3_vec3_mul(&Self::FROM_XYZ, color.as_slice());
        return Self::from_fn(|i| c[i]);
    }
}
