use super::macros::*;

/// Representation of an OkLch color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OkLch([f32; 3]);

//impl_color_new!([f32; 3], OkLch);
impl_self_index!(OkLch);
impl_from_tup3!(OkLch);
impl_typ_as_self!(OkLch, [f32; 3]);
impl_self_as_typ!([f32], OkLch);
impl_self_as_typ!([f32; 3], OkLch);
impl_from_inner!([f32; 3], OkLch);
