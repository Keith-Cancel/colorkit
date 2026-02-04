//! Conversion Traits between color spaces.
use colorkit::colors::Xyz;
use colorkit::math::matrix_3x3_vec3_mul;
use colorkit::num_type::N3;
use colorkit::num_type::Number;
use colorkit::space::ColorArray;
use colorkit::space::ColorData;

// Traits for converting between different color spaces.
// ============================================================================

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

/// Marker trait stating conversion from `Self` <-> `C` exists both ways.
pub trait FromColorBoth<C>: FromColor<C> + private::FromBound<C, Other: FromColor<Self>, Other = C> {}

/// Transformation Matrices to go between and from CIE XYZ
///
/// This trait expects that [`ColorData::Channels`] is equal
/// to *3*, and slices/arrays gotten via [`ColorArray`] are also
/// length 3.
///
/// If you implement this trait [`FromColor`] will be implemented
/// both ways between [`Xyz`] and Self.
pub trait XyzMatrices: ColorData {
    // TODO
    // Maybe add my number type trick back to some these traits
    // since I can't do associated const equality in stable.
    // Mainly the length thing.
    // Ideally pub trait XyzMatrices: ColorData<CHANNELS = 3>

    // Looks like people generally represent these as a transformation matrix.
    // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
    /// 3x3 Matrix to convert into XYZ
    const INTO_XYZ: [f32; 9];
    /// 3x3 Matrix to convert from XYZ
    const FROM_XYZ: [f32; 9];
}

// Impls for color conversion
// ============================================================================

// Blanket implentation for Self.
impl<C> FromColor<C> for C {
    #[inline]
    fn from_color(color: C) -> Self {
        return color;
    }
}

// Blanket impl of IntoColor for any implementation of FromColor
impl<C1, C2: FromColor<C1>> IntoColor<C2> for C1 {
    fn into_color(self) -> C2 {
        return C2::from_color(self);
    }
}

// Blanket Implentation if From is defined for both color spaces.
impl<C1: FromColor<C2>, C2: FromColor<C1>> FromColorBoth<C2> for C1 {}

impl<C: ColorArray + XyzMatrices<Channels = N3>> FromColor<Xyz<C::WhitePoint>> for C {
    fn from_color(color: Xyz<C::WhitePoint>) -> Self {
        debug_assert!(C::Channels::N == 3);
        let c = matrix_3x3_vec3_mul(&Self::FROM_XYZ, color.as_slice());
        return Self::from_fn(|i| c[i]);
    }
}

impl<C: ColorArray + XyzMatrices<Channels = N3>> FromColor<C> for Xyz<C::WhitePoint> {
    fn from_color(color: C) -> Self {
        debug_assert!(C::Channels::N == 3);
        return Xyz::from_array(matrix_3x3_vec3_mul(&C::INTO_XYZ, color.as_slice()));
    }
}

// Traits for arrays and slice types
// ============================================================================

/// This marker trait marks that a color can be transmuted
/// into and from array of [f32; [`ColorData::Channels`]]
///
/// Essentially `size_of::<Self>() / size_of::<f32>()` should
/// equal [`ColorData::Channels`], plus other constraints like
/// alignment ect...
pub unsafe trait ColorTransmute: ColorData {}

/// Failable reference to color conversion.
///
/// Similar to [`AsRef`], but only for colors and fail-able.
pub trait AsColorRef<C: ColorData> {
    /// Try to view self as a reference to this color type.
    ///
    /// If self can't be viewed as the color returns [`None`]
    fn as_color(&self) -> Option<&C>;
}

/// Failable mutable reference to color conversion.
///
/// Similar to [`AsRef`], but only for colors and fail-able.
pub trait AsColorMut<C: ColorData> {
    /// Try to view a self as a mutable reference to this color type.
    ///
    /// If self can't be viewed as the color returns [`None`]
    fn as_mut_color(&mut self) -> Option<&mut C>;
}

// Impls for arrays and slice types
// ============================================================================

impl<T: AsColorRef<C>, C: ColorData> AsColorRef<C> for &T {
    #[inline]
    fn as_color(&self) -> Option<&C> {
        return T::as_color(*self);
    }
}

impl<T: AsColorRef<C>, C: ColorData> AsColorRef<C> for &mut T {
    #[inline]
    fn as_color(&self) -> Option<&C> {
        return T::as_color(*self);
    }
}

impl<T: AsColorMut<C>, C: ColorData> AsColorMut<C> for &mut T {
    #[inline]
    fn as_mut_color(&mut self) -> Option<&mut C> {
        return T::as_mut_color(*self);
    }
}

impl<C: ColorData + ColorTransmute> AsColorRef<C> for [f32] {
    /// Try to view a slice as a reference to this color type.
    ///
    /// If the slice's legnth is not equal to number channels in the color
    /// this may return [`None`]`
    #[inline]
    fn as_color(&self) -> Option<&C> {
        if self.len() != <C::Channels as Number>::N {
            return None;
        }
        let ptr = self.as_ptr() as *const C;
        // Safety:
        // ColorTransmute was implemnted so it's safe
        // to treat a slice or array of the same length
        // as channels as an instance of that color.
        return Some(unsafe { &*ptr });
    }
}

impl<C: ColorData + ColorTransmute> AsColorMut<C> for [f32] {
    /// Try to view a slice as a mutable reference to this color type.
    ///
    /// If the slice's legnth is not equal to number channels in the color
    /// this may return [`None`]`
    #[inline]
    fn as_mut_color(&mut self) -> Option<&mut C> {
        if self.len() != <C::Channels as Number>::N {
            return None;
        }
        let ptr = self.as_mut_ptr() as *mut C;
        // Safety:
        // ColorTransmute was implemnted so it's safe
        // to treat a slice or array of the same length
        // as channels as an instance of that color.
        return Some(unsafe { &mut *ptr });
    }
}

impl<C: ColorData + ColorTransmute, const N: usize> AsColorRef<C> for [f32; N] {
    /// Try to view an array as a reference to this color type.
    ///
    /// If the array's legnth is not equal to number channels in the color
    /// this may return [`None`]`
    #[inline]
    fn as_color(&self) -> Option<&C> {
        if N != <C::Channels as Number>::N {
            return None;
        }
        let ptr = self.as_ptr() as *const C;
        // Safety:
        // ColorTransmute was implemnted so it's safe
        // to treat a slice or array of the same length
        // as channels as an instance of that color.
        return Some(unsafe { &*ptr });
    }
}

impl<C: ColorData + ColorTransmute, const N: usize> AsColorMut<C> for [f32; N] {
    /// Try to view a array as a mutable reference to this color type.
    ///
    /// If the array's legnth is not equal to number channels in the color
    /// this may return [`None`]`
    #[inline]
    fn as_mut_color(&mut self) -> Option<&mut C> {
        if N != <C::Channels as Number>::N {
            return None;
        }
        let ptr = self.as_mut_ptr() as *mut C;
        // Safety:
        // ColorTransmute was implemnted so it's safe
        // to treat a slice or array of the same length
        // as channels as an instance of that color.
        return Some(unsafe { &mut *ptr });
    }
}

// Sealed/Private conversion traits
// ============================================================================

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
