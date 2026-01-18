use std::ops::Index;
use std::ops::IndexMut;

use colorkit::space::ColorSpace;
use colorkit::space::ChannelBound;

use colorkit::color::AlphaKindNormal;
use colorkit::color::AlphaStore;
use colorkit::color::AlphaKindInfo;
use colorkit::color::ColorArray;
use colorkit::color::ColorCommon;

/// A color with an alpha channel.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Alpha<S: ColorSpace>([f32; S::CHANNELS_NEXT]);

impl<S: ColorSpace> Alpha<S> {
    const CHAN_ASSERT: () = Self::channel_assert();
    pub const fn from_array(arr: [f32; S::CHANNELS_NEXT]) -> Self {
        let _ = Self::CHAN_ASSERT;
        return Self(arr);
    }

    /// Convert Alpha into a color removing the alpha
    /// channel. All other channels are uneffected.
    //pub const fn into_color(self) -> Color<S> {
    //    let mut arr: [f32; S::CHANNELS] = [0.0; S::CHANNELS];
    //    let (_, slc) = self.0.split_last().unwrap();
    //    arr.copy_from_slice(slc);
    //    return Color::from_array(arr);
    //}

    /// Get the alpha channel
    #[inline]
    pub const fn alpha(&self) -> f32 {
        return self.0[S::CHANNELS];
    }

    /// Get a mutable reference to the alpha channel.
    #[inline]
    pub const fn alpha_mut(&mut self) -> &mut f32 {
        return &mut self.0[S::CHANNELS];
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

impl <S:ColorSpace> AlphaKindInfo for Alpha<S> {
    type Kind = AlphaKindNormal;
}

impl <S: ColorSpace> AlphaStore for Alpha<S> {
    #[inline]
    fn alpha_ref(&self) -> &f32 {
        return &self[S::CHANNELS];
    }

    #[inline]
    fn alpha_mut(&mut self) -> &mut f32 {
        return &mut self[S::CHANNELS]
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

use colorkit::color::AlphaValueTry;
use colorkit::color::AlphaValue;
use colorkit::color::AlphaStoreTry;

fn test<C: ColorSpace>(mut a: Alpha<C>) {
    let _ = a.alpha();
    let _ = a.try_alpha();
    let _ = a.alpha_set(99.0);
    let _ = a.try_alpha_set(100.0);

    let _ = a.alpha_ref();
    let _ = a.try_alpha_ref();
    let _ = a.alpha_mut();
    let _ = a.try_alpha_mut();
}
