/// Call `$call!($int_typ, ....)` for every integer type.
/// Usage:
///   with_ints!(my_macro!(extra, args, ));
/// expands to:
///   my_macro!(extra, args, u8);
///   my_macro!(extra, args, u16);
///   ...
macro_rules! with_ints {
    ($call:ident ! ( $($args:tt)* )) => {
        $call!($($args)* u8);
        $call!($($args)* u16);
        $call!($($args)* u32);
        $call!($($args)* u64);
        $call!($($args)* usize);
        $call!($($args)* i8);
        $call!($($args)* i16);
        $call!($($args)* i32);
        $call!($($args)* i64);
        $call!($($args)* isize);
    };

    // convenience form: allow calling without parens:
    ($call:ident !) => {
        with_ints!($call!());
    };
}
pub(crate) use with_ints;

macro_rules! bit_uint_binop_lhs {
    ($trait:ident :: $method:ident, $lhs:ident) => {
        impl<const N: u32> core::ops::$trait<BitUint<N, $lhs>> for $lhs {
            type Output = $lhs;

            #[inline]
            fn $method(self, rhs: BitUint<N, $lhs>) -> Self::Output {
                return $lhs::$method(self, rhs.get());
            }
        }
    };
}
pub(crate) use bit_uint_binop_lhs;

macro_rules! bit_uint_binop {
    ($trait:ident :: $method:ident) => {
        impl<const N: u32, T: BitUintType> core::ops::$trait for BitUint<N, T> {
            type Output = <T as core::ops::$trait>::Output;
            #[inline]
            fn $method(self, rhs: BitUint<N, T>) -> Self::Output {
                return T::$method(self.get(), rhs.get());
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<T> for BitUint<N, T> {
            type Output = <T as core::ops::$trait>::Output;
            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                return T::$method(self.get(), rhs);
            }
        }

        with_ints!(bit_uint_binop_lhs!($trait::$method,));
    };
}
pub(crate) use bit_uint_binop;

macro_rules! bit_uint_binop_ref_lhs {
    ($trait:ident :: $method:ident, $lhs:ident) => {
        impl<const N: u32> core::ops::$trait<BitUint<N, $lhs>> for &$lhs {
            type Output = <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::Output;
            #[inline]
            fn $method(self, rhs: BitUint<N, $lhs>) -> Self::Output {
                return <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::$method(*self, rhs);
            }
        }

        impl<const N: u32> core::ops::$trait<&BitUint<N, $lhs>> for $lhs {
            type Output = <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::Output;
            #[inline]
            fn $method(self, rhs: &BitUint<N, $lhs>) -> Self::Output {
                return <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::$method(self, *rhs);
            }
        }

        impl<const N: u32> core::ops::$trait<&BitUint<N, $lhs>> for &$lhs {
            type Output = <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::Output;
            #[inline]
            fn $method(self, rhs: &BitUint<N, $lhs>) -> Self::Output {
                return <$lhs as core::ops::$trait<BitUint<N, $lhs>>>::$method(*self, *rhs);
            }
        }
    };
}
pub(crate) use bit_uint_binop_ref_lhs;

macro_rules! bit_uint_binop_ref {
    ($trait:ident :: $method:ident) => {
        impl<const N: u32, T: BitUintType> core::ops::$trait<BitUint<N, T>> for &BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::Output;
            #[inline]
            fn $method(self, rhs: BitUint<N, T>) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::$method(*self, rhs);
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<&BitUint<N, T>> for BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::Output;
            #[inline]
            fn $method(self, rhs: &BitUint<N, T>) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::$method(self, *rhs);
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<&BitUint<N, T>> for &BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::Output;
            #[inline]
            fn $method(self, rhs: &BitUint<N, T>) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<BitUint<N, T>>>::$method(*self, *rhs);
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<T> for &BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<T>>::Output;
            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<T>>::$method(*self, rhs);
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<&T> for BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<T>>::Output;
            #[inline]
            fn $method(self, rhs: &T) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<T>>::$method(self, *rhs);
            }
        }

        impl<const N: u32, T: BitUintType> core::ops::$trait<&T> for &BitUint<N, T> {
            type Output = <BitUint<N, T> as core::ops::$trait<T>>::Output;
            #[inline]
            fn $method(self, rhs: &T) -> Self::Output {
                return <BitUint<N, T> as core::ops::$trait<T>>::$method(*self, *rhs);
            }
        }

        with_ints!(bit_uint_binop_ref_lhs!($trait::$method,));
    };
}
pub(crate) use bit_uint_binop_ref;

macro_rules! bit_uint_assign_op {
    ($trait:ident :: $method:ident, $typ:ident) => {
        impl<const N: u32> core::ops::$trait<BitUint<N, $typ>> for $typ {
            #[inline]
            fn $method(&mut self, rhs: BitUint<N, $typ>) {
                return $typ::$method(self, rhs.get());
            }
        }

        impl<const N: u32> core::ops::$trait<&BitUint<N, $typ>> for $typ {
            #[inline]
            fn $method(&mut self, rhs: &BitUint<N, $typ>) {
                return $typ::$method(self, rhs.get());
            }
        }
    };
}
pub(crate) use bit_uint_assign_op;
