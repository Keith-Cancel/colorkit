use core::f32::consts::PI;

use colorkit::convert::*;
use colorkit::math::*;
use colorkit::num_type::N3;
use colorkit::space::*;
use colorkit::wp::D65;

use super::macros::*;
use super::*;

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
        BoundF32::Include(0.70710677),
        BoundF32::Include(PI),
    ];

    const CHANNEL_MIN: [BoundF32; 3] = [
        BoundF32::Include(0.0),
        BoundF32::Include(0.0),
        BoundF32::Include(-PI),
    ];
}

impl FromColor<OkLab> for OkLch {
    fn from_color(color: OkLab) -> Self {
        let d = color.into_array();
        let c = sqrtf(d[1] * d[1] + d[2] + d[2]);
        let h = atan2f(d[2], d[1]);
        return Self([d[0], c, h]);
    }
}

impl FromColor<OkLch> for OkLab {
    fn from_color(color: OkLch) -> Self {
        let d = color.0;
        let h = f32::clamp(d[2], -PI, PI);
        let a = d[1] * cosf_on_pi(h);
        let b = d[1] * sinf_on_pi(h);
        return OkLab::new(d[0], a, b);
    }
}

impl FromColor<Xyz<D65>> for OkLch {
    fn from_color(color: Xyz<D65>) -> Self {
        let lab: OkLab = color.into_color();
        return lab.into_color();
    }
}

impl FromColor<OkLch> for Xyz<D65> {
    fn from_color(color: OkLch) -> Self {
        let lab: OkLab = color.into_color();
        return lab.into_color();
    }
}
