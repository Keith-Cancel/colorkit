use core::ops::Index;
use core::ops::IndexMut;

use colorkit::convert::ColorTransmute;
use colorkit::num_type::Number;
use colorkit::space::ColorData;

/// Trait to let Color Spaces be handled like slices.
pub trait ColorSlice: AsRef<[f32]> + AsMut<[f32]> + Index<usize, Output = f32> + IndexMut<usize, Output = f32> {
    /// View color as a slice reference.
    fn as_slice(&self) -> &[f32] {
        return self.as_ref();
    }
    /// View color as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [f32] {
        return self.as_mut();
    }
    /// Get a channel value reference or [`None`].
    ///
    /// Similar semantics as [`slice::get()`]
    fn get(&self, index: usize) -> Option<&f32> {
        return self.as_ref().get(index);
    }
    /// Get a mutable channel value reference or [`None`].
    ///
    /// Similar semantics as [`slice::get_mut()`]
    fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        return self.as_mut().get_mut(index);
    }
    /// Length of the slice
    fn len(&self) -> usize {
        return self.as_ref().len();
    }
    /// Try to get a reference as an array.
    ///
    /// If `N` not equal to the number of channels returns [`None`]
    fn as_array<const N: usize>(&self) -> Option<&[f32; N]> {
        return self.as_ref().as_array();
    }
    /// Try to get a reference as an mutable array.
    ///
    /// If `N` not equal to the number of channels returns [`None`]
    fn as_mut_array<const N: usize>(&mut self) -> Option<&mut [f32; N]> {
        return self.as_mut().as_mut_array();
    }
    /// Swaps two channels in the color.
    fn swap(&mut self, a: usize, b: usize) {
        self.as_mut().swap(a, b);
    }
    /// Swaps two channels in the color, but without checking bounds.
    ///
    /// # Safety
    ///
    /// Agruments `a` and `b` must with bounds. Failure to do so results
    /// *undefined behavior*.
    /// Esentially, `a < self.len()` and `b < self.len()` must be true
    unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let slc = self.as_mut();
        debug_assert!(a < slc.len() && b < slc.len());
        let ptr = slc.as_mut_ptr();
        // Safety:
        // caller ensures `a < slcc.len()` and `b < slc.len()`
        unsafe {
            core::ptr::swap(ptr.add(a), ptr.add(b));
        }
    }
}

/// Trait to let Color Spaces be handled mostly like an array.
pub trait ColorArray: ColorSlice {
    /// Construct the Color calling `f(i)` for each index (same semantics as [`core::array::from_fn`]).
    fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self;
}

// Traits for arrays and slice types
// ============================================================================
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
