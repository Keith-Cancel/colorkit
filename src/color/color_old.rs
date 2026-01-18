use std::ops::Index;
use std::ops::IndexMut;

use colorkit::space::ChannelBound;
use colorkit::space::ColorSpace;

use super::Alpha;
use super::AlphaKind;
use super::AlphaTry;
use super::ColorArray;
use super::ColorCommon;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color<S: ColorSpace>([f32; S::CHANNELS]);

impl<S: ColorSpace> Color<S> {
    pub const fn from_array(arr: [f32; S::CHANNELS]) -> Self {
        return Self(arr);
    }

    pub const fn with_alpha(self, alpha: f32) -> Alpha<S> {
        // create new array with one extra slot
        let mut arr: [f32; S::CHANNELS_NEXT] = [0.0; S::CHANNELS_NEXT];
        let (last, slc) = arr.split_last_mut().unwrap();
        slc.copy_from_slice(&self.0);
        *last = alpha;
        return Alpha::from_array(arr);
        /* Just gonna comment this out if pre-initializing the array
        has performance issues I can always use this still.
        // Would not need unsafe if there was a const version:
        // let arr: [f32; S::CHANNELS_NEXT] = from_fn(|i| *self.0.get(i).unwrap_or(&alpha));

        let mut arr = [MaybeUninit::<f32>::uninit(); S::CHANNELS_NEXT];
        // copy existing N floats in one go
        unsafe { ptr::copy_nonoverlapping(self.0.as_ptr(), arr.as_mut_ptr() as *mut f32, S::CHANNELS) };
        arr[S::CHANNELS] = MaybeUninit::new(alpha);

        // not stable
        // let arr = unsafe { MaybeUninit::array_assume_init(arr) };
        // plus I see some talk about a differnt method transpose
        // so not sure what will be decided on, so just go back
        // to what even works in stable a union transmute.
        #[repr(C)]
        union Transmute<S: ColorSpace> {
            uninit: [MaybeUninit<f32>; S::CHANNELS_NEXT],
            init:   [f32; S::CHANNELS_NEXT],
        }
        let tran = Transmute::<S> {
            uninit: arr
        };
        return Alpha::<S>::from_array(unsafe { tran.init });*/
    }
}

impl<S: ColorSpace> Index<usize> for Color<S> {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &f32 {
        return &self.0[index];
    }
}

impl<S: ColorSpace> IndexMut<usize> for Color<S> {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        return &mut self.0[index];
    }
}

impl<S: ColorSpace> ColorArray for Color<S> {
    #[type_const]
    const LENGTH: usize = { S::CHANNELS };

    #[inline]
    fn from_array(array: [f32; S::CHANNELS]) -> Self {
        return Self::from_array(array);
    }

    #[inline]
    fn into_array(self) -> [f32; S::CHANNELS] {
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

impl<S: ColorSpace> AlphaTry for Color<S> {
    const ALPHA_KIND: Option<AlphaKind> = None;

    #[inline]
    fn try_alpha(&self) -> Option<&f32> {
        return None;
    }

    #[inline]
    fn try_alpha_mut(&mut self) -> Option<&mut f32> {
        return None;
    }
}

impl<S: ColorSpace> ColorCommon for Color<S> {
    fn channel_max(ch_num: usize) -> ChannelBound {
        return S::channel_max(ch_num);
    }

    fn channel_min(ch_num: usize) -> ChannelBound {
        return S::channel_min(ch_num);
    }
}

#[cfg(test)]
mod test {
    use colorkit::space::Srgb;

    use super::*;
    #[test]
    fn add_remove_alpha() {
        let c = Color::<Srgb>([0.2, 0.6, 1.0]);
        let a = c.with_alpha(0.5);
        assert_eq!(a[0], 0.2);
        assert_eq!(a[1], 0.6);
        assert_eq!(a[2], 1.0);
        assert_eq!(a[3], 0.5);

        let c = a.remove_alpha();
        assert_eq!(c[0], 0.2);
        assert_eq!(c[1], 0.6);
        assert_eq!(c[2], 1.0);
    }
}
