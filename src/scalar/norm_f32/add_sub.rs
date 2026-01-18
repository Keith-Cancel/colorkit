use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

use super::NormF32;
use super::macros::*;

op_binary_wrapped!(NormF32, f32, Add::add);
op_binary_ref!(f32, NormF32, Add::add);
op_binary_ref!(NormF32, f32, Add::add);
op_binary_ref!(NormF32, NormF32, Add::add);

op_assign_wrapped!(NormF32, f32, AddAssign::add_assign);
op_assign_ref!(f32, NormF32, AddAssign::add_assign);

op_binary_wrapped!(NormF32, f32, Sub::sub);
op_binary_ref!(f32, NormF32, Sub::sub);
op_binary_ref!(NormF32, f32, Sub::sub);
op_binary_ref!(NormF32, NormF32, Sub::sub);

op_assign_wrapped!(NormF32, f32, SubAssign::sub_assign);
op_assign_ref!(f32, NormF32, SubAssign::sub_assign);
