//use super::FromLayout;
//use super::GrowLayout;
use colorkit::scalar::BitUint;
use colorkit::scalar::Dither;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::scalar::Scalar;

use super::Layout;
use super::LayoutStorage;
//use super::TruncateLayout;
//use super::private::LayoutSealed;

/// Packed 5-6-5 channel layout in a [u16].
///
/// The channel bit-widths are:
/// * Channel 0: 5 Bits
/// * Channel 1: 6 Bits
/// * Channel 2: 5 Bits
///
/// # Note
/// The type is `repr(transparent)`, and all bit patterns
/// are valid.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Packed565(u16);

impl Packed565 {
    pub const MSK_5: u16 = 0x1f;
    pub const MSK_6: u16 = 0x3f;
    pub const MSK_C0: u16 = !Self::MSK_5;
    pub const MSK_C1: u16 = !(Self::MSK_6 << 5);
    pub const MSK_C2: u16 = !(Self::MSK_5 << 11);

    pub const fn new(c0: u16, c1: u16, c2: u16) -> Self {
        let mut v = c0 & Self::MSK_5;
        v |= (c1 & Self::MSK_6) << 5;
        v |= (c2 & Self::MSK_5) << 11;
        return Self(v);
    }

    pub fn get(&self, index: usize) -> u16 {
        return match index {
            0 => self.0 & Self::MSK_5,
            1 => (self.0 >> 5) & Self::MSK_6,
            2 => (self.0 >> 11) & Self::MSK_5,
            _ => panic!(
                "index out of bounds: max index for get_raw() is 3, but index is {}",
                index
            ),
        };
    }

    pub fn set(&mut self, index: usize, value: u16) {
        let (s, v) = match index {
            0 => (self.0 & Self::MSK_C0, value),
            1 => (self.0 & Self::MSK_C1, value << 5),
            2 => (self.0 & Self::MSK_C2, value << 11),
            _ => panic!(
                "index out of bounds: max index for set_raw() is 3, but index is {}",
                index
            ),
        };
        self.0 = s | v;
    }
}

impl LayoutStorage for Packed565 {
    type Storage = u16;
    #[inline]
    fn as_storage(&self) -> &Self::Storage {
        return &self.0;
    }
    #[inline]
    fn as_storage_mut(&mut self) -> &mut Self::Storage {
        return &mut self.0;
    }
}

impl Layout for Packed565 {
    const DEFAULT: Self = Self(0);
    const CHANNELS: usize = 3;
    type ChannelType = u8;

    fn from_fn_raw<F: FnMut(usize) -> u8>(fun: F) -> Self {
        let mut fun = fun;
        let mut ret = Self(0);
        for i in 0..3 {
            ret.set(i, fun(i) as u16);
        }
        return ret;
    }

    fn get_raw(&self, index: usize) -> u8 {
        return self.get(index) as u8;
    }

    fn set_raw(&mut self, index: usize, value: u8) {
        return self.set(index, value as u16);
    }

    fn get_norm(&self, index: usize) -> NormF32 {
        let val = self.get(index);
        if index == 1 {
            return BitUint::<6, u16>::new_masked(val).into_norm();
        }
        return BitUint::<5, u16>::new_masked(val).into_norm();
    }

    fn set_norm(&mut self, value: NormF32, index: usize, round: Rounding) {
        let val = if index == 1 {
            BitUint::<6, u16>::from_norm(value, round).get()
        } else {
            BitUint::<5, u16>::from_norm(value, round).get()
        };
        self.set(index, val);
    }

    fn set_norm_dither<D: Dither>(&mut self, value: NormF32, index: usize, round: Rounding, dither: &mut D) {
        let val = if index == 1 {
            BitUint::<6, u16>::from_norm_dither(value, round, dither).get()
        } else {
            BitUint::<5, u16>::from_norm_dither(value, round, dither).get()
        };
        self.set(index, val);
    }

    fn from_fn_norm<F: FnMut(usize) -> NormF32>(fun: F, round: Rounding) -> Self {
        let mut fun = fun;
        let mut ret = Self(0);
        for i in 0..3 {
            ret.set_norm(fun(i), i, round);
        }
        return ret;
    }

    fn from_fn_norm_dither<F: FnMut(usize) -> NormF32, D: Dither>(fun: F, round: Rounding, dither: &mut D) -> Self {
        let mut fun = fun;
        let mut ret = Self(0);
        for i in 0..3 {
            ret.set_norm_dither(fun(i), i, round, dither);
        }
        return ret;
    }
}

impl From<u16> for Packed565 {
    #[inline]
    fn from(value: u16) -> Self {
        return Self(value);
    }
}

impl From<Packed565> for u16 {
    #[inline]
    fn from(value: Packed565) -> Self {
        return value.0;
    }
}
/*
impl FromLayout<Packed565> for Packed565 {
    #[inline]
    fn from_layout(layout: Packed565) -> Self {
        return layout;
    }
}

impl GrowLayout<Packed565> for Packed565 {
    #[inline]
    fn grow_layout(self) -> Packed565 {
        return self;
    }
}

impl TruncateLayout<Packed565> for Packed565 {
    #[inline]
    fn truncate_layout(self) -> Packed565 {
        return self;
    }
}
*/
#[cfg(test)]
mod test {
    use super::Layout;
    use super::Packed565;
    use super::Rounding;

    fn to_float(val: u16, max: u16) -> f32 {
        return (val as f32) / (max as f32);
    }

    fn from_float(val: f32, max: u16) -> u16 {
        let f = val * (max as f32);
        return f.round() as u16;
    }

    #[test]
    fn check_packed565() {
        for i in 0..32u16 {
            let mut v = Packed565::new(i, i, i);

            let x = to_float(i, 31);
            let y = v.get_norm(0);
            assert_eq!(x, y.get());

            let x = from_float(x, 31);
            v.set_norm(y, 0, Rounding::Nearest);

            assert_eq!(v.get(0), i);
            assert_eq!(v.get(0), x);
        }

        for i in 0..64u16 {
            let mut v = Packed565::new(i, i, i);

            let x = to_float(i, 63);
            let y = v.get_norm(1);
            assert_eq!(x, y.get());

            let x = from_float(x, 63);
            v.set_norm(y, 1, Rounding::Nearest);

            assert_eq!(v.get(1), i);
            assert_eq!(v.get(1), x);
        }
    }

    #[test]
    fn set_get_packed565() {
        let mut v = Packed565::default();
        assert_eq!(v.get(0), 0);
        assert_eq!(v.get(1), 0);
        assert_eq!(v.get(2), 0);

        v.set(0, 29);
        v.set(1, 32);
        v.set(2, 31);
        assert_eq!(v.get(0), 29);
        assert_eq!(v.get(1), 32);
        assert_eq!(v.get(2), 31);

        v.set(0, u16::MAX);
        v.set(1, u16::MAX);
        v.set(2, u16::MAX);

        assert_eq!(v.get(0), 31);
        assert_eq!(v.get(1), 63);
        assert_eq!(v.get(2), 31);
    }
}
