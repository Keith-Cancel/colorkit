//! Marker types that act as a number, mainly used for [`Layout`](colorkit::layout::Layout).
use core::array::from_fn;
use core::fmt::Debug;
use core::ops::Index;
use core::ops::IndexMut;

mod private {
    pub trait NumberSealed {}
}
use private::NumberSealed;

#[cfg(feature = "type_const")]
macro_rules! type_const {
    ($name:ident : $ty:ty { $val:expr }) => {
        type const $name: $ty = const { $val };
    };
    ($name:ident : $ty:ty) => {
        type const $name: $ty;
    };
}

#[cfg(not(feature = "type_const"))]
macro_rules! type_const {
    ($name:ident : $ty:ty { $val:expr }) => {
        /// Value of the number
        const $name: $ty = { $val };
    };
    ($name:ident : $ty:ty) => {
        /// Value of the number
        const $name: $ty;
    };
}

pub trait NumArray<T>:
    AsRef<[T]>
    + AsMut<[T]>
    + Copy
    + Debug
    + Index<usize, Output = T>
    + IndexMut<usize, Output = T>
    + PartialEq
{
    const LEN: usize;
    fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self;
}

impl<T: Copy + Debug + PartialEq, const N: usize> NumArray<T> for [T; N] {
    const LEN: usize = N;
    fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
        return from_fn(f);
    }
}

/// This allows me to work with numbers, as types albeit up to a limited N
///
/// Till min_generic_const_args is stabilized or at least less crashy.
///
/// It being a type lets me get around associated const equality since
/// I can perform associated type equality instead.
pub trait Number: NumberSealed + Copy {
    type_const!(N: usize);
    /// Number increased by 1
    type Inc: Number<Dec = Self>;
    /// Number decreased by 1
    type Dec: Number<Inc = Self>;
    /// Array the same length as the number.
    type Arr<T: Copy + Debug + PartialEq>: NumArray<T>;
    fn value() -> usize {
        return Self::N;
    }
}

/// Marker type that is paired with [`ToNumber`]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Num<const N: usize>;

/// Mainly to constrain things like `<const N: usize>` to the range of defined numbers.
///
/// For example:
/// ```
/// use colorkit::num_type::{Num, ToNumber};
/// pub trait Foo {}
///
/// impl <const N: usize> Foo for [u8; N]
/// where Num<N>: ToNumber {}
/// ```
/// `Foo`` will only be defined up to the numbers I have defined.
pub trait ToNumber {
    type Number: Number;
}

macro_rules! impl_num {
    ($name:ident, $n:expr, $dec:ident, $inc:ident) => {
        #[doc = concat!("A marker type denoting the number `", stringify!($n), "`")]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl NumberSealed for $name {}
        impl Number for $name {
            type_const!(N : usize { $n } );
            type Arr<T: Copy + Debug + PartialEq> = [T; $n];

            type Inc = $inc;
            type Dec = $dec;
        }

        impl ToNumber for Num<$n> {
            type Number = $name;
        }
    };
}

impl_num!(N0, 0, N10, N1);
impl_num!(N1, 1, N0, N2);
impl_num!(N2, 2, N1, N3);
impl_num!(N3, 3, N2, N4);
impl_num!(N4, 4, N3, N5);
impl_num!(N5, 5, N4, N6);
impl_num!(N6, 6, N5, N7);
impl_num!(N7, 7, N6, N8);
impl_num!(N8, 8, N7, N9);
impl_num!(N9, 9, N8, N10);
impl_num!(N10, 10, N9, N0);

#[allow(unused)]
macro_rules! with_nums {
    ($call:ident ! ( $($args:tt)* )) => {
        $call!($($args)* N0);
        $call!($($args)* N1);
        $call!($($args)* N2);
        $call!($($args)* N3);
        $call!($($args)* N4);
        $call!($($args)* N5);
        $call!($($args)* N6);
        $call!($($args)* N7);
        $call!($($args)* N8);
        $call!($($args)* N9);
        $call!($($args)* N10);
    };

    // convenience form: allow calling without parens:
    ($call:ident !) => {
        with_nums!($call!());
    };
}

#[allow(unused)]
pub(crate) use with_nums;
