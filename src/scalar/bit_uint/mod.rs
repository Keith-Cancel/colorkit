mod cmp;
mod macros;
mod ops;
mod scalar;
mod uint_type;

use macros::with_ints;
pub use uint_type::BitUintType;

/// Value `T` is outside the range of the given [`BitUint`]
#[derive(Debug)]
pub struct RangeError<T>(T);

/// A variable bit-width unsigned integer.
#[derive(Copy, Debug, Eq)]
#[repr(transparent)]
pub struct BitUint<const BITS: u32, T: BitUintType = u32>(T);

impl<const BITS: u32, T: BitUintType> BitUint<BITS, T> {
    /// Default value, which is Zero.
    pub const DEFAULT: Self = Self(T::ZERO);
    /// Number of BITs the Uint uses
    pub const BITS: u32 = BITS;
    /// Max value, it also functions as the bit MASK.
    pub const MAX: Self = { Self(T::MASKS[BITS as usize]) };
    /// Min value
    pub const MIN: Self = Self(T::ZERO);

    /// Create new [`BitUint`] from `value` while checking `value` is in range.
    ///
    /// Returns [`None`] if `value` to large for the bitwidth or negative.
    pub fn new(value: T) -> Option<Self> {
        if value < Self::MIN.get() || value > Self::MAX.get() {
            return None;
        }
        return Some(Self(value));
    }

    /// Create a new [`BitUint`], but clamped by [`BitUint::MAX`] and [`BitUint::MIN`]
    pub fn new_clamped(value: T) -> Self {
        if value > Self::MAX.get() {
            return Self::MAX;
        }
        if value < Self::MIN.get() {
            return Self::MIN;
        }
        return Self(value);
    }

    /// Create a new [`BitUint`], but masked by [`BitUint::MAX`]
    pub fn new_masked(value: T) -> Self {
        return Self(value & Self::MAX.get());
    }

    pub const unsafe fn new_unchecked(value: T) -> Self {
        return Self(value);
    }

    /// Get the value as the underlying storage type.
    pub const fn get(self) -> T {
        return self.0;
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let v = self.0.checked_add(rhs.0)?;
        return Self::new(v);
    }

    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let v = self.0.checked_sub(rhs.0)?;
        return Self::new(v);
    }

    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let v = self.0.checked_mul(rhs.0)?;
        return Self::new(v);
    }

    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        let v = self.0.checked_div(rhs.0)?;
        return Self::new(v);
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        let v = self.0.checked_rem(rhs.0)?;
        return Self::new(v);
    }
}

impl<T: BitUintType, const BITS: u32> Default for BitUint<BITS, T> {
    fn default() -> Self {
        return Self::DEFAULT;
    }
}

impl<T: BitUintType, const BITS: u32> Clone for BitUint<BITS, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        return *self;
    }

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

macro_rules! impl_from {
    ($typ:ident) => {
        impl<const BITS: u32> From<BitUint<BITS, $typ>> for $typ {
            fn from(value: BitUint<BITS, $typ>) -> $typ {
                return value.0;
            }
        }

        impl<const BITS: u32> TryFrom<$typ> for BitUint<BITS, $typ> {
            type Error = RangeError<$typ>;
            fn try_from(value: $typ) -> Result<Self, RangeError<$typ>> {
                if value < Self::MIN.get() || value > Self::MAX.get() {
                    return Err(RangeError(value));
                }
                return Ok(Self(value));
            }
        }
    };
}
with_ints!(impl_from!);

#[cfg(test)]
mod test {
    use super::BitUint;
    #[test]
    fn check_max() {
        assert_eq!(<BitUint::<0, u8>>::MAX.get(), 0);
        assert_eq!(<BitUint::<1, u8>>::MAX.get(), 1);
        assert_eq!(<BitUint::<2, u8>>::MAX.get(), 3);
        assert_eq!(<BitUint::<4, u8>>::MAX.get(), 15);
        assert_eq!(<BitUint::<7, u8>>::MAX.get(), 127);
        assert_eq!(<BitUint::<8, u8>>::MAX.get(), 255);
        assert_eq!(<BitUint::<9, u16>>::MAX.get(), 511);

        assert_eq!(<BitUint::<7, i8>>::MAX.get(), i8::MAX);

        assert_eq!(<BitUint::<32>>::MAX.get(), u32::MAX);
    }

    #[test]
    fn check_new() {
        assert!(<BitUint::<3>>::new(8).is_none());
        assert_eq!(<BitUint::<3>>::new_clamped(8).get(), 7);
        assert_eq!(<BitUint::<3>>::new_masked(8).get(), 0);
    }
}
