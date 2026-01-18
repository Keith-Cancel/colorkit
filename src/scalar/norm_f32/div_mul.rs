use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Rem;
use core::ops::RemAssign;

use super::NormF32;
use super::macros::*;

op_binary_wrapped!(NormF32, f32, Mul::mul);
op_binary_ref!(f32, NormF32, Mul::mul);
op_binary_ref!(NormF32, f32, Mul::mul);
op_binary_ref!(NormF32, NormF32, Mul::mul);

op_assign_wrapped!(NormF32, f32, MulAssign::mul_assign);
op_assign_ref!(f32, NormF32, MulAssign::mul_assign);

op_binary_wrapped!(NormF32, f32, Div::div);
op_binary_ref!(f32, NormF32, Div::div);
op_binary_ref!(NormF32, f32, Div::div);
op_binary_ref!(NormF32, NormF32, Div::div);

op_assign_wrapped!(NormF32, f32, DivAssign::div_assign);
op_assign_ref!(f32, NormF32, DivAssign::div_assign);

op_binary_wrapped!(NormF32, f32, Rem::rem);
op_binary_ref!(f32, NormF32, Rem::rem);
op_binary_ref!(NormF32, f32, Rem::rem);
op_binary_ref!(NormF32, NormF32, Rem::rem);

op_assign_wrapped!(NormF32, f32, RemAssign::rem_assign);
op_assign_ref!(f32, NormF32, RemAssign::rem_assign);
