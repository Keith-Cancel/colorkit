use std::array::from_fn;
use std::ops::Index;
use std::ops::IndexMut;

use colorkit::space::ColorSpace;
use colorkit::space::ChannelBound;

use colorkit::color::AlphaOps;
use colorkit::color::AlphaKind;
use colorkit::color::AlphaTry;
use colorkit::color::Color;
use colorkit::color::ColorArray;
use colorkit::color::ColorCommon;

/// A color with it's alpha premultiplied on all other channels.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlphaPre<S: ColorSpace>([f32; S::CHANNELS_NEXT]);

impl<S: ColorSpace> AlphaPre<S> {
    const CHAN_ASSERT: () = Self::channel_assert();
    pub const fn from_array(arr: [f32; S::CHANNELS_NEXT]) -> Self {
        let _ = Self::CHAN_ASSERT;
        return Self(arr);
    }

    /// Convert Alpha into a color removing the alpha
    /// channel. All other channels are uneffected.
    pub const fn into_color(self) -> Color<S> {
        let a = self.0.ch
        let mut arr: [f32; S::CHANNELS] = from_fn(|i| {
            if i == S::CHANNELS {

            }
        });
        let (_, slc) = self.0.split_last().unwrap();
        arr.copy_from_slice(slc);
        return Color::from_array(arr);
    }

    #[inline(always)]
    pub const fn alpha_max() -> f32 {
        return 1.0;
    }

    #[inline(always)]
    pub const fn alpha_min() -> f32 {
        return 0.0;
    }

    const fn channel_assert() {
        assert!(
            (S::CHANNELS + 1) == S::CHANNELS_NEXT,
            "Color Space `CHANNELS_NEXT` must be 1 larger `CHANNELS`"
        );
    }
}

impl<S: ColorSpace> Index<usize> for Alpha<S> {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        return &self.0[index];
    }
}

impl<S: ColorSpace> IndexMut<usize> for Alpha<S> {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        return &mut self.0[index];
    }
}

impl<S: ColorSpace> ColorArray for Alpha<S> {
    #[type_const]
    const LENGTH: usize = { S::CHANNELS_NEXT };

    #[inline]
    fn from_array(array: [f32; S::CHANNELS_NEXT]) -> Self {
        return Self::from_array(array);
    }

    #[inline]
    fn into_array(self) -> [f32; S::CHANNELS_NEXT] {
        return self.0;
    }

    #[inline]
    fn as_slice(&self) -> &[f32] {
        return &self.0;
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [f32] {
        return &mut self.0;
    }

    #[inline]
    unsafe fn from_slice_unchecked(slice: &[f32]) -> &Self {
        return unsafe { &*(slice.as_ptr() as *const Self) };
    }

    #[inline]
    unsafe fn from_slice_unchecked_mut(slice: &mut [f32]) -> &mut Self {
        return unsafe { &mut *(slice.as_mut_ptr() as *mut Self) };
    }
}

impl <S: ColorSpace> AlphaTry for Alpha<S> {
    const ALPHA_KIND: Option<AlphaKind> = Some(AlphaKind::Normal);

    #[inline]
    fn try_alpha(&self) -> Option<&f32> {
        return self.get(S::CHANNELS);
    }
    #[inline]
    fn try_alpha_mut(&mut self) -> Option<&mut f32> {
        return self.get_mut(S::CHANNELS);
    }
}

impl <S: ColorSpace> ColorCommon for Alpha<S> {
    fn channel_max(ch_num: usize) -> ChannelBound {
        if ch_num == S::CHANNELS {
            return ChannelBound::Included(1.0);
        }
        return S::channel_max(ch_num);
    }

    fn channel_min(ch_num: usize) -> ChannelBound {
        if ch_num == S::CHANNELS {
            return ChannelBound::Included(0.0);
        }
        return S::channel_min(ch_num);
    }

}

impl <S: ColorSpace> AlphaOps for Alpha<S> {
    #[inline]
    fn alpha(&self) -> &f32 {
        return &self.0[S::CHANNELS];
    }

    #[inline]
    fn alpha_mut(&mut self) -> &mut f32 {
        return &mut self.0[S::CHANNELS];
    }
}
