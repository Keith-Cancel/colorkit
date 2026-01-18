use core::ops::*;

use super::macros::with_ints;

#[rustfmt::skip]
pub trait BitUintType:
    'static
    + Sized
    + Copy
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Add
    + Sub
    + Mul
    + Div
    + Rem
    + BitAnd<Output = Self>
    + BitOr
    + BitXor
{
    const ZERO: Self;
    const ONE: Self;
    const BITS: u32;
    const UNSIGNED: bool;
    /// Bit masks masking the least signagant bits starting from zero up to
    /// the number of bits.
    /// This is kinda a work around for const traits not being stable.
    const MASKS: &'static [Self]; // Work around since const traits are not stable ugg.

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    fn try_from_u32(value: u32) -> Option<Self>;
    fn try_into_u32(self) -> Option<u32>;
}

#[rustfmt::skip] // Prevent const {...} from being deleted
macro_rules! impl_int_like {
    ($typ:ident) => {
        impl BitUintType for $typ {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const BITS: u32 = $typ::BITS;
            #[allow(unused_comparisons)]
            const UNSIGNED: bool = $typ::MIN >= 0;

            const MASKS: &'static [Self] = &{
                let mut msk = [0; const { (Self::BITS as usize) + (Self::UNSIGNED as usize) }];
                let mut v: Self = 0;
                let mut i = 0;
                while i < msk.len() {
                    msk[i] = v;
                    i += 1;
                    v <<= 1;
                    v |= 1;
                }
                msk
            };

            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                return self.checked_add(rhs);
            }
            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                return self.checked_sub(rhs);
            }
            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                return self.checked_mul(rhs);
            }
            #[inline]
            fn checked_div(self, rhs: Self) -> Option<Self> {
                return self.checked_div(rhs);
            }
            #[inline]
            fn checked_rem(self, rhs: Self) -> Option<Self> {
                return self.checked_rem(rhs);
            }

            fn try_from_u32(value: u32) -> Option<Self> {
                return Self::try_from(value).ok();
            }

            fn try_into_u32(self) -> Option<u32> {
                return u32::try_from(self).ok();
            }
        }
    };
}
with_ints!(impl_int_like!);
