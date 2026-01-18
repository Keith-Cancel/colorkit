use std::ops::Index;
use std::ops::IndexMut;

pub trait ColorArray: Copy + Index<usize, Output = f32> + IndexMut<usize, Output = f32> {
    /// /// Length of the color's Channels array. should generally be the same as the number of channels.
    #[type_const]
    const LENGTH: usize;
    // Convert an array into a color.
    fn from_array(array: [f32; Self::LENGTH]) -> Self;
    // Convert a color to an array.
    fn into_array(self) -> [f32; Self::LENGTH];

    #[inline]
    fn len(&self) -> usize {
        return self.as_slice().len();
    }

    /// View the color as slice containing all it's channels.
    fn as_slice(&self) -> &[f32];

    /// View the color as a mutable slice containing all it's channels.
    fn as_mut_slice(&mut self) -> &mut [f32];

    /// Returns a reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    fn get(&self, index: usize) -> Option<&f32> {
        return self.as_slice().get(index);
    }

    /// Returns a mutable reference to the channel value at `index` or
    /// `None` if the index is out of bounds.
    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        return self.as_mut_slice().get_mut(index);
    }

    /// Try to create a view of `slice` as this color.
    ///
    /// Returns `None` if the slice length < [`ColorArray::LENGTH`] of the color.
    fn try_from_slice(slice: &[f32]) -> Option<&Self> {
        if slice.len() < Self::LENGTH {
            return None;
        }
        return Some(unsafe { Self::from_slice_unchecked(slice) });
    }

    /// Try to create a mutable view of `slice` as this color.
    ///
    /// Returns `None` if the slice length < [`ColorArray::LENGTH`] of the color.
    fn try_from_slice_mut(slice: &mut [f32]) -> Option<&mut Self> {
        if slice.len() < Self::LENGTH {
            return None;
        }
        return Some(unsafe { Self::from_slice_unchecked_mut(slice) });
    }

    /// Create a shared view from `slice` without checking length.
    ///
    /// # Safety
    /// Caller must ensure slice.len() >= [`ColorArray::LENGTH`] of the color.
    unsafe fn from_slice_unchecked(slice: &[f32]) -> &Self;

    /// Create a mutable view from `slice` without checking length.
    ///
    /// # Safety
    /// Caller must ensure slice.len() >= [`ColorArray::LENGTH`] of the color.
    unsafe fn from_slice_unchecked_mut(slice: &mut [f32]) -> &mut Self;
}
