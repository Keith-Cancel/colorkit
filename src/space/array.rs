pub trait ColorArray {
    /// View color as a slice reference.
    fn as_slice(&self) -> &[f32];
    /// View color as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [f32];
}

// Hmm ideally I would provide a blanket impl like:
// but blanket cause conherance issues.
//impl<C: ColorArray> core::ops::Index<usize> for C {
//    type Output = f32;
//    fn index(&self, index: usize) -> &Self::Output {
//        return &self.as_slice()[index];
//    }
//}
