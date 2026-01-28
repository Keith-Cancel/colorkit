use colorkit::space2::ChannelBound;
use colorkit::space2::ColorArray;
use colorkit::space2::ColorData;
use colorkit::space2::ColorSpace;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>(S, f32);

impl<S: ColorSpace> Alpha<S> {
    const CHANNELS: usize = size_of::<Alpha<S>>() / size_of::<f32>();
    const MAX: &'static [crate::space2::ChannelBound] = &const {
        // Just make this larger than likely needed can't use
        // S or Self in the len of an array =(
        let mut max = [ChannelBound::Unbounded; 16];
        let mut i = 0;
        while i < S::CHANNEL_MAX.len() {
            max[i] = S::CHANNEL_MAX[i];
            i += 1;
        }
        max[i] = ChannelBound::Included(1.0);
        max
    };
    const MIN: &'static [crate::space2::ChannelBound] = &const {
        let mut arr = [ChannelBound::Unbounded; 16];
        let mut i = 0;
        while i < S::CHANNEL_MIN.len() {
            arr[i] = S::CHANNEL_MIN[i];
            i += 1;
        }
        arr[i] = ChannelBound::Included(0.0);
        arr
    };
}

impl<S: ColorSpace> Default for Alpha<S> {
    fn default() -> Self {
        return Self(S::DEFAULT, 1.0);
    }
}

impl<S: ColorSpace> ColorData for Alpha<S> {
    type WhitePoint = S::WhitePoint;
    const DEFAULT: Self = Self(S::DEFAULT, 1.0);
    const LINEAR: bool = S::LINEAR;
    const CHANNEL_MAX: &'static [ChannelBound] = { Self::MAX.split_at(Self::CHANNELS).0 };
    const CHANNEL_MIN: &'static [ChannelBound] = { Self::MIN.split_at(Self::CHANNELS).0 };
}

#[cfg(test)]
mod test {
    use super::*;
    use colorkit::colors::OkLab;
    use colorkit::colors::Srgb;
    use colorkit::colors::Xyz;
    use colorkit::space2::ColorData;
    use colorkit::wp::D65;

    #[test]
    fn min_max() {
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MIN.len(), 4);

        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[0], ChannelBound::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[1], ChannelBound::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[2], ChannelBound::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[3], ChannelBound::Included(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[0], ChannelBound::Included(1.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[1], ChannelBound::Included(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[2], ChannelBound::Included(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[3], ChannelBound::Included(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[0], ChannelBound::Included(0.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[1], ChannelBound::Included(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[2], ChannelBound::Included(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[3], ChannelBound::Included(0.0));
    }
}
