use core::fmt::Debug;

mod private {
    pub trait NumberSealed {}
}
use private::NumberSealed;

/// This allows me to work with numbers, as types albeit up to a limited N
///
/// Till min_generic_const_args is stabilized or at least less crashy.
///
/// It being a type lets me get around associated const equality since
/// I can perform associated type equality instead.
pub trait Number: NumberSealed + Copy {
    /// Value of the number
    const N: usize;
    /// Number increased by 1
    type Inc: Number<Dec = Self>;
    /// Number decreased by 1
    type Dec: Number<Inc = Self>;
}

/// Marker type that is paired with [`ToNumber`]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Num<const N: usize>;

/// Mainly to constrain things like `<const N: usize>` to the range of defined numbers.
///
/// For example:
/// ```
/// use colorkit::utils::{Num, ToNumber};
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
            // use cfg instead of cfg_atrr
            // currently ICEs
            // https://github.com/rust-lang/rust/issues/151273
            #[cfg(feature = "type_const")]
            #[type_const]
            const N: usize = $n;
            #[cfg(not(feature = "type_const"))]
            const N: usize = $n;

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
