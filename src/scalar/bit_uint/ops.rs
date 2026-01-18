use super::BitUint;
use super::BitUintType;
use super::macros::*;

bit_uint_binop!(Add::add);
bit_uint_binop_ref!(Add::add);
with_ints!(bit_uint_assign_op!(AddAssign::add_assign,));

bit_uint_binop!(Sub::sub);
bit_uint_binop_ref!(Sub::sub);
with_ints!(bit_uint_assign_op!(SubAssign::sub_assign,));

bit_uint_binop!(Mul::mul);
bit_uint_binop_ref!(Mul::mul);
with_ints!(bit_uint_assign_op!(MulAssign::mul_assign,));

bit_uint_binop!(Div::div);
bit_uint_binop_ref!(Div::div);
with_ints!(bit_uint_assign_op!(DivAssign::div_assign,));

bit_uint_binop!(Rem::rem);
bit_uint_binop_ref!(Rem::rem);
with_ints!(bit_uint_assign_op!(RemAssign::rem_assign,));

bit_uint_binop!(BitAnd::bitand);
bit_uint_binop_ref!(BitAnd::bitand);
with_ints!(bit_uint_assign_op!(BitAndAssign::bitand_assign,));

bit_uint_binop!(BitOr::bitor);
bit_uint_binop_ref!(BitOr::bitor);
with_ints!(bit_uint_assign_op!(BitOrAssign::bitor_assign,));

bit_uint_binop!(BitXor::bitxor);
bit_uint_binop_ref!(BitXor::bitxor);
with_ints!(bit_uint_assign_op!(BitXorAssign::bitxor_assign,));

#[cfg(test)]
mod test {
    use super::BitUint;

    macro_rules! bin_ops {
        ( $($ops:tt)* ) => {
            $({
                let a = <BitUint<5>>::new_masked(0x13);
                let b = <BitUint<5>>::new_masked(0x3);
                let &c = &a;
                let &d = &b;
                let e = 13u32;
                let f = &e;

                let x = a.get() $ops b.get();

                let y = a $ops b;
                assert_eq!(x, y);

                let y = c $ops b;
                assert_eq!(x, y);

                let y = a $ops d;
                assert_eq!(x, y);

                let y = c $ops d;
                assert_eq!(x, y);

                let x = a.get() $ops e;

                let y = a $ops e;
                assert_eq!(x, y);

                let y = c $ops e;
                assert_eq!(x, y);

                let y = a $ops f;
                assert_eq!(x, y);

                let y = c $ops f;
                assert_eq!(x, y);
            })*

        };
    }

    macro_rules! assign_ops {
        ( $($ops:tt)* ) => {
            $({
                let a = <BitUint<5, u16>>::new_masked(0x13);
                let b = &a;

                let mut x = 99u16;
                x $ops a.get();

                let mut y = 99u16;
                y $ops a;
                assert_eq!(x, y);

                let mut y = 99u16;
                y $ops b;
                assert_eq!(x, y);
            })*
        };
    }

    #[test]
    fn bin_ops() {
        bin_ops!(+ - * / % | ^ &);
        assign_ops!(+= -= *= /= %= |= ^= &=);
    }
}
