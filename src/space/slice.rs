use core::ops::Index;
use core::ops::IndexMut;

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
