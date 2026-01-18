use colorkit::utils::N3;

use super::ChannelBound;
use super::ColorSpace;
use super::D65;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Srgb;

impl ColorSpace for Srgb {
    type Channels = N3;
    type WhitePoint = D65;
    const LINEAR: bool = false;
    const CHANNEL_MAX: &'static [ChannelBound] = &[ChannelBound::Included(1.0); 3];
    const CHANNEL_MIN: &'static [ChannelBound] = &[ChannelBound::Included(0.0); 3];

    #[inline(always)]
    fn channel_max(i: usize) -> ChannelBound {
        debug_assert!(i < 3);
        return ChannelBound::Included(1.0);
    }

    #[inline(always)]
    fn channel_min(i: usize) -> ChannelBound {
        debug_assert!(i < 3);
        return ChannelBound::Included(0.0);
    }
}
