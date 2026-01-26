use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::ops::Index;
use core::ops::IndexMut;

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
    /// Try to gets a reference as an array.
    ///
    /// If `N` is greater than [`ColorArray::CHANNELS`] returns [`None`]`
    fn try_as_array<const N: usize>(&self) -> Option<&[f32; N]>;

    /// Number Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::CHANNELS;
    }
}

pub trait ColorSpace: ColorArray + Default {
    const DEFAULT: Self;
}
