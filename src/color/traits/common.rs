use std::array::from_fn;

use super::ColorArray;
use super::AlphaValueTry;
use colorkit::space::ChannelBound;


pub trait ColorCommon: ColorArray + AlphaValueTry {
    /// Get Max value for a given channel in the color space
    fn channel_max(ch_num: usize) -> ChannelBound;

    /// Get min value for a given channel in the color space
    fn channel_min(ch_num: usize) -> ChannelBound;

    /// Check if the color’s channels are all within the range bounds.
    fn is_within_bounds(&self) -> bool {
        for (i, &v) in self.as_slice().iter().enumerate() {
            if let ChannelBound::Included(max) = Self::channel_max(i) && v > max {
                return false;
            }
            if let ChannelBound::Included(min) = Self::channel_min(i) && v < min {
                return false;
            }
        }
        return true;
    }

    /// Clamp all channels to min and max
    fn clamp(self) -> Self {
        let arr = self.into_array();
        return Self::from_array(from_fn(|i|{
            let v = arr[i];
            let v = if let ChannelBound::Included(max) = Self::channel_max(i) && v > max { max } else { v };
            if let ChannelBound::Included(min) = Self::channel_min(i) && v < min { min } else { v }
        }));
    }
}
