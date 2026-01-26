mod white_point;

use core::borrow::Borrow;
use core::borrow::BorrowMut;
use core::ops::Index;
use core::ops::IndexMut;

#[rustfmt::skip]
pub use white_point::*;

/// Defines the a bound on a color space channel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelBound {
    Included(f32),
    Unbounded,
}

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
    /// Try to get a reference as an array.
    ///
    /// If `N` is greater than [`ColorArray::CHANNELS`] returns [`None`]`
    fn try_as_array<const N: usize>(&self) -> Option<&[f32; N]>;
    /// Try to get a reference as an mutable array.
    ///
    /// If `N` is greater than [`ColorArray::CHANNELS`] returns [`None`]`
    fn try_as_mut_array<const N: usize>(&self) -> Option<&mut [f32; N]>;
}

pub trait ColorSpace: ColorArray + Default {
    /// Default color, should be black.
    const DEFAULT: Self;
    /// Color Spaces White Point
    type WhitePoint: WhitePoint;
    /// Are the Channels Linear
    const LINEAR: bool;
    /// Upper or max bound of each channel.
    const CHANNEL_MAX: &'static [ChannelBound];
    /// Lower or min bound of each channel.
    const CHANNEL_MIN: &'static [ChannelBound];

    // what else to add?
    // primaries?

    /// Number Channels
    #[inline]
    fn channels(&self) -> usize {
        return Self::CHANNELS;
    }

    /// Get Max value for a given channel in the color space
    #[inline(always)]
    fn channel_max(ch_num: usize) -> ChannelBound {
        return Self::CHANNEL_MAX[ch_num];
    }

    /// Get min value for a given channel in the color space
    #[inline(always)]
    fn channel_min(ch_num: usize) -> ChannelBound {
        return Self::CHANNEL_MIN[ch_num];
    }
}
