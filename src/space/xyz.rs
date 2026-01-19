use std::marker::PhantomData;

use colorkit::utils::N3;

use super::ChannelBound;
use super::ColorSpace;
use super::WhitePoint;

/// CIE 1931 XYZ color space
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Xyz<W: WhitePoint>(PhantomData<W>);

impl<W: WhitePoint> ColorSpace for Xyz<W> {
    type Channels = N3;
    type WhitePoint = W;
    const LINEAR: bool = true;
    const CHANNEL_MAX: &'static [ChannelBound] = &[ChannelBound::Unbounded; 3];
    const CHANNEL_MIN: &'static [ChannelBound] = &[ChannelBound::Included(0.0); 3];
}
