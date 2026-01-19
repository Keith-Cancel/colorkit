//! Color Spaces
mod srgb;
mod white_point;

use colorkit::utils::Number;

#[rustfmt::skip]
pub use srgb::Srgb;
pub use white_point::*;

/// Defines the a bound on a color space channel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelBound {
    Included(f32),
    Unbounded,
}

pub trait ColorSpace: Copy {
    /// Number of Channels
    type Channels: Number;
    /// Color Spaces White Point
    type WhitePoint: WhitePoint;
    /// Are the Channels Linear
    const LINEAR: bool;
    /// Upper or max bound of each channel.
    const CHANNEL_MAX: &'static [ChannelBound];
    /// Lower or min bound of each channel.
    const CHANNEL_MIN: &'static [ChannelBound];

    /*
    /// Color Spaces White point
    const WHITE_POINT: Option<WhitePoint>;

    // what else to add?
    // white point?
    // primaries?*/

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
