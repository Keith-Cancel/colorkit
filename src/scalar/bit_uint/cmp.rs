use core::cmp::Ordering;
use core::cmp::PartialEq;
use core::cmp::PartialOrd;

use super::BitUint;
use super::BitUintType;
use super::macros::with_ints;

// Implement PartialEq Traits
// ==================================================
impl<T: BitUintType, const BITS: u32> PartialEq for BitUint<BITS, T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        return T::eq(&self.0, &rhs.0);
    }
}

impl<T: BitUintType, const BITS: u32> PartialEq<T> for BitUint<BITS, T> {
    #[inline]
    fn eq(&self, rhs: &T) -> bool {
        return T::eq(&self.0, rhs);
    }
}

macro_rules! impl_lhs_eq {
    ($typ:ident) => {
        impl<const BITS: u32> PartialEq<BitUint<BITS, $typ>> for $typ {
            #[inline]
            fn eq(&self, rhs: &BitUint<BITS, $typ>) -> bool {
                return $typ::eq(self, &rhs.0);
            }
        }
    };
}
// Can't a generic like this:
// impl<T: IntLike, const BITS: u32> PartialEq<BitUint<BITS, T>> for T {
// so just manually spell out each primitive.
with_ints!(impl_lhs_eq!);

// Implement PartialOrd Traits
// ==================================================
impl<T: BitUintType, const BITS: u32> PartialOrd for BitUint<BITS, T> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        return T::partial_cmp(&self.0, &rhs.0);
    }
    #[inline]
    fn lt(&self, rhs: &Self) -> bool {
        return T::lt(&self.0, &rhs.0);
    }
    #[inline]
    fn le(&self, rhs: &Self) -> bool {
        return T::le(&self.0, &rhs.0);
    }
    #[inline]
    fn gt(&self, rhs: &Self) -> bool {
        return T::gt(&self.0, &rhs.0);
    }
    #[inline]
    fn ge(&self, rhs: &Self) -> bool {
        return T::ge(&self.0, &rhs.0);
    }
}

impl<T: BitUintType, const BITS: u32> PartialOrd<T> for BitUint<BITS, T> {
    #[inline]
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        return T::partial_cmp(&self.0, rhs);
    }
    #[inline]
    fn lt(&self, rhs: &T) -> bool {
        return T::lt(&self.0, rhs);
    }
    #[inline]
    fn le(&self, rhs: &T) -> bool {
        return T::le(&self.0, rhs);
    }
    #[inline]
    fn gt(&self, rhs: &T) -> bool {
        return T::gt(&self.0, rhs);
    }
    #[inline]
    fn ge(&self, rhs: &T) -> bool {
        return T::ge(&self.0, rhs);
    }
}

macro_rules! impl_lhs_ord {
    ($typ:ident) => {
        impl<const BITS: u32> PartialOrd<BitUint<BITS, $typ>> for $typ {
            #[inline]
            fn partial_cmp(&self, rhs: &BitUint<BITS, $typ>) -> Option<Ordering> {
                return $typ::partial_cmp(self, &rhs.0);
            }
            #[inline]
            fn lt(&self, rhs: &BitUint<BITS, $typ>) -> bool {
                return $typ::lt(self, &rhs.0);
            }
            #[inline]
            fn le(&self, rhs: &BitUint<BITS, $typ>) -> bool {
                return $typ::le(self, &rhs.0);
            }
            #[inline]
            fn gt(&self, rhs: &BitUint<BITS, $typ>) -> bool {
                return $typ::gt(self, &rhs.0);
            }
            #[inline]
            fn ge(&self, rhs: &BitUint<BITS, $typ>) -> bool {
                return $typ::ge(self, &rhs.0);
            }
        }
    };
}
with_ints!(impl_lhs_ord!);
