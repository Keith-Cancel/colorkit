use core::ops::Index;
use core::ops::IndexMut;

pub trait ColorSlice: AsRef<[f32]> + AsMut<[f32]> + Index<usize, Output = f32> + IndexMut<usize, Output = f32> {
    /// View color as a slice reference.
    fn as_slice(&self) -> &[f32];
    /// View color as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [f32];
    /// Length of the slice
    fn len(&self) -> usize {
        return self.as_slice().len();
    }
    /// Swaps two channels in the color.
    fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
    /// Swaps two channels in the color, but without checking bounds.
    ///
    /// # Safety
    ///
    /// Agruments `a` and `b` must with bounds. Failure to do so results
    /// *undefined behavior*.
    /// Esentially, `a < self.len()` and `b < self.len()` must be true
    unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        let slc = self.as_mut_slice();
        debug_assert!(a < slc.len() && b < slc.len());
        let ptr = slc.as_mut_ptr();
        // Safety:
        // caller ensures `a < slcc.len()` and `b < slc.len()`
        unsafe {
            core::ptr::swap(ptr.add(a), ptr.add(b));
        }
    }
}

// Hmm ideally I would provide a blanket impl like:
// but blanket cause conherance issues.
//impl<C: ColorArray> core::ops::Index<usize> for C {
//    type Output = f32;
//    fn index(&self, index: usize) -> &Self::Output {
//        return &self.as_slice()[index];
//    }
//}
