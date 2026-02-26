use core::f32::consts::PI;

use colorkit::math::*;
use colorkit::num_type::N3;
use colorkit::space::*;
use colorkit::wp::D65;

use super::macros::*;

/// Representation of an OkLch color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OkLch([f32; 3]);

impl_color_new!([f32; 3], OkLch);
impl_self_index!(OkLch);
impl_from_tup3!(OkLch);
impl_typ_as_self!(OkLch, [f32; 3]);
impl_self_as_typ!([f32], OkLch);
impl_self_as_typ!([f32; 3], OkLch);
impl_from_inner!([f32; 3], OkLch);

impl Default for OkLch {
    fn default() -> Self {
        return Self([1.0, 0.0, 0.0]);
    }
}

impl ColorData for OkLch {
    type Channels = N3;
    type WhitePoint = D65;

    const LINEAR: bool = true;

    const CHANNEL_MAX: [BoundF32; 3] = [
        BoundF32::Include(1.0),
        BoundF32::Include(PI),
        BoundF32::Include(PI),
    ];

    const CHANNEL_MIN: [BoundF32; 3] = [
        BoundF32::Include(0.0),
        BoundF32::Include(-PI),
        BoundF32::Include(-PI),
    ];
}
