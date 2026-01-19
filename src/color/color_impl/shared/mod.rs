mod xyz;
use std::ops::Index;
use std::ops::IndexMut;

use colorkit::space::*;
use colorkit::utils::*;

use super::Color;

impl<S: ColorSpace> Color<S> {
    pub(crate) const COLOR_ASSERT: () = Self::color_assert();
    /// Number of channels for the color.
    #[inline]
    pub const fn channels(&self) -> usize {
        return S::Channels::N;
    }

    /// Get Max value for a given channel in the color space
    pub const fn channel_max(ch_num: usize) -> ChannelBound {
        return S::CHANNEL_MAX[ch_num];
    }

    /// Get min value for a given channel in the color space
    pub const fn channel_min(ch_num: usize) -> ChannelBound {
        return S::CHANNEL_MIN[ch_num];
    }

    /// Check if the color’s channels are all within the range bounds.
    #[rustfmt::skip]
    pub const fn is_within_bounds(&self) -> bool {
        let slc = self.as_slice();
        let len = slc.len();
        let mut i = 0;
        while i < len {
            let v = slc[i];
            if let ChannelBound::Included(max) = S::CHANNEL_MAX[i] && v > max {
                return false;
            }
            if let ChannelBound::Included(min) = S::CHANNEL_MIN[i] && v < min {
                return false;
            }
            i += 1;
        }
        return true;
    }

    /// Clamp all channels to min and max
    #[rustfmt::skip]
    pub const fn clamp(mut self) -> Self {
        let slc = self.as_mut_slice();
        let len = slc.len();
        let mut i = 0;
        while i < len {
            let v = slc[i];
            let v = if let ChannelBound::Included(max) = S::CHANNEL_MAX[i] && v > max { max } else { v };
            slc[i] = if let ChannelBound::Included(min) = S::CHANNEL_MIN[i] && v < min { min } else { v };
            i += 1;
        }
        return self;
    }

    const fn color_assert() {
        let ch = <S::Channels as Number>::N;
        if ch != S::CHANNEL_MAX.len() {
            panic!("ColorSpace::CHANNEL_MAX does not match number of channels");
        }
        if ch != S::CHANNEL_MIN.len() {
            panic!("ColorSpace::CHANNEL_MIN does not match number of channels");
        }
    }
}

impl<S: ColorSpace> Index<usize> for Color<S> {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<S: ColorSpace> IndexMut<usize> for Color<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

#[cfg(test)]
mod test {
    use colorkit::space::Srgb;

    use super::Color;
    #[test]
    fn clamp() {
        let c = <Color<Srgb>>::try_from_slice(&[2.0, -1.25, 0.25]).unwrap();
        assert_eq!(c[0], 2.0);
        assert_eq!(c[1], -1.25);
        assert_eq!(c[2], 0.25);
        assert_eq!(c.is_within_bounds(), false);
        let c = c.clamp();
        assert_eq!(c[0], 1.0);
        assert_eq!(c[1], 0.0);
        assert_eq!(c[2], 0.25);
        assert_eq!(c.is_within_bounds(), true);
    }
}
