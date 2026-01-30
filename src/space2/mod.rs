use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::ops::Index;
use core::ops::IndexMut;

use colorkit::colors::Xyz;
use colorkit::math::BoundF32;
use colorkit::math::matrix_3x3_vec3_mul;
use colorkit::wp::WhitePoint;

/// This marker trait marks that a color can be
/// transmuted into an array of [f32; [`ColorArray::CHANNELS`]]
///
/// Essentially `size_of::<Self>() / size_of::<f32>()` should
/// equal [`ColorArray::CHANNELS`], plus other constraints like
/// alignment ect...
pub unsafe trait ColorTransmute: ColorSpace {}

/// Trait to let Color Spaces be handled mostly like an array/slice.
pub trait ColorArray:
    Copy
    + AsRef<[f32]>
    + AsMut<[f32]>
    + Borrow<[f32]>
    + BorrowMut<[f32]>
    + Index<usize, Output = f32>
    + IndexMut<usize, Output = f32>
{
    /// Number of channels or also should be the length of the array.
    const CHANNELS: usize;
    /// Construct the Color calling `f(i)` for each index (same semantics as [`core::array::from_fn`]).
    fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self;
    /// View color as a slice reference.
    fn as_slice(&self) -> &[f32];
    /// View color as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [f32];
    /// Try to get a reference as an array.
    ///
    /// If `N` is greater than [`ColorArray::CHANNELS`] returns [`None`]`
    fn try_as_array<const N: usize>(&self) -> Option<&[f32; N]> {
        if N > Self::CHANNELS {
            return None;
        }
        let (slc, _) = self.as_slice().split_at(N);
        return slc.try_into().ok();
    }
    /// Try to get a reference as an mutable array.
    ///
    /// If `N` is greater than [`ColorArray::CHANNELS`] returns [`None`]`
    fn try_as_mut_array<const N: usize>(&mut self) -> Option<&mut [f32; N]> {
        if N > Self::CHANNELS {
            return None;
        }
        let (slc, _) = self.as_mut_slice().split_at_mut(N);
        return slc.try_into().ok();
    }

    /// Get channel value reference or `None`.
    fn get_ref(&self, index: usize) -> Option<&f32> {
        return self.as_slice().get(index);
    }

    /// Get a mutable channel value reference or `None`.
    fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        return self.as_mut_slice().get_mut(index);
    }
}

/// Information about a Color Space
pub trait ColorData: Default {
    /// Default color, should be black.
    const DEFAULT: Self;
    /// Color Spaces White Point
    type WhitePoint: WhitePoint;
    /// Are the Channels Linear
    const LINEAR: bool;
    /// Upper or max bound of each channel.
    const CHANNEL_MAX: &'static [BoundF32];
    /// Lower or min bound of each channel.
    const CHANNEL_MIN: &'static [BoundF32];

    // what else to add?
    // primaries?
}

/// The main ColorSpace Trait
pub trait ColorSpace: ColorArray + ColorData + XyzConvert {
    /// Number Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::CHANNELS;
    }

    /// Get Max value for a given channel in the color space
    #[inline(always)]
    fn channel_max(ch_num: usize) -> BoundF32 {
        return Self::CHANNEL_MAX[ch_num];
    }

    /// Get min value for a given channel in the color space
    #[inline(always)]
    fn channel_min(ch_num: usize) -> BoundF32 {
        return Self::CHANNEL_MIN[ch_num];
    }

    /// Check if the color’s channels are all within the range bounds.
    fn within_bounds(&self) -> bool {
        for (i, &v) in self.as_slice().iter().enumerate() {
            if let BoundF32::Include(max) = Self::CHANNEL_MAX[i]
                && v > max
            {
                return false;
            }
            if let BoundF32::Include(min) = Self::CHANNEL_MIN[i]
                && v < min
            {
                return false;
            }
        }
        return true;
    }

    /// Clamp all channels to min and max
    fn clamp(self) -> Self {
        let slc = self.as_slice();
        return Self::from_fn(|i| {
            let mut v = slc[i];
            if let BoundF32::Include(max) = Self::CHANNEL_MAX[i]
                && v > max
            {
                v = max;
            }
            if let BoundF32::Include(min) = Self::CHANNEL_MIN[i]
                && v < min
            {
                v = min;
            }
            v
        });
    }
}

/// Trait with common operations for RGB like color spaces.
pub trait RgbLike: ColorSpace {
    /// Create a new color from RGB values.
    #[inline]
    fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        let arr = [r, g, b];
        return Self::from_fn(|i| arr[i]);
    }
    /// Get the Color's red channel value.
    #[inline]
    fn red(&self) -> f32 {
        return self[0];
    }
    /// Get the Color's blue channel value.
    #[inline]
    fn blue(&self) -> f32 {
        return self[1];
    }
    /// Get the Color's green channel value.
    #[inline]
    fn green(&self) -> f32 {
        return self[2];
    }
    /// Set the Color's red channel value.
    #[inline]
    fn set_red(&mut self, value: f32) {
        self[0] = value;
    }
    /// Set the Color's blue channel value.
    #[inline]
    fn set_blue(&mut self, value: f32) {
        self[1] = value;
    }

    /// Set the Color's green channel value.
    #[inline]
    fn set_green(&mut self, value: f32) {
        self[2] = value;
    }
}

/// Conversion between CIE XYZ and other Color Spaces.
pub trait XyzConvert: ColorData {
    /// Convert a color into CIE XYZ with it's white point.
    fn into_xyz(self) -> Xyz<Self::WhitePoint>;
    /// Convert a color from CIE XYZ into this color Space.
    fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self;
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
