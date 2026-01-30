use colorkit::convert::ColorTransmute;
use colorkit::convert::FromColor;
use colorkit::math::BoundF32;
use colorkit::math::cbrtf;
use colorkit::math::matrix_3x3_vec3_mul;
use colorkit::space::ColorArray;
use colorkit::space::ColorData;
use colorkit::space::ColorSpace;
use colorkit::wp::D65;

use super::Xyz;
use super::macros::impl_color_array;

/// Represention of an OkLab color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OkLab([f32; 3]);

impl OkLab {
    /// Matrix used as part the conversion From XYZ
    ///
    /// <https://bottosson.github.io/posts/oklab/>
    /// Also See: `./tools/matrices.jl`
    #[rustfmt::skip]
    pub const M1: [f32; 9] = [
        0.8189884519759543,  0.3618912424500841, -0.1288684337956881,
        0.0329839110434283,  0.9292941054748394,  0.0361449481021053,
        0.0481841465068086,  0.2642776899213323,  0.6336393175236821,
    ];
    /// The matrix M1's inverse (M2^-1)
    ///
    /// Used to convert OkLab to XYZ
    #[rustfmt::skip]
    pub const M1_INV: [f32; 9] = [
         1.2269307883646734, -0.5578106462677585,  0.2813504165573680,
        -0.0405774290091394,  1.1122781329556793, -0.0717007039465399,
        -0.0763761059650258, -0.4214900468431466,  1.5866948892040570,
    ];
    /// Matrix used as part the conversion From XYZ
    ///
    /// <https://bottosson.github.io/posts/oklab/>
    /// Also See: `./tools/matrices.jl`
    #[rustfmt::skip]
    pub const M2: [f32; 9] = [
        0.2104542683093140,  0.7936177747023052, -0.0040720430116193,
        1.9779985324311686, -2.4285922420485796,  0.4505937096174110,
        0.0259040424655478,  0.7827717124575297, -0.8086757549230775,
    ];
    /// The matrix M2's inverse (M2^-1)
    ///
    /// Used to convert OkLab to XYZ
    #[rustfmt::skip]
    pub const M2_INV: [f32; 9] = [
        1.0,  0.3963377773761749,  0.2158037573099136,
        1.0, -0.1055613458156586, -0.0638541728258133,
        1.0, -0.0894841775298119, -1.2914855480194092,
    ];

    /// Create a new color from `Lab` values.
    pub const fn new(l: f32, a: f32, b: f32) -> Self {
        return Self([l, a, b]);
    }
    /// Get the Color's the `L` channel value.
    #[inline]
    pub const fn l(&self) -> f32 {
        return self.0[0];
    }
    /// Get the Color's the `a` channel value.
    #[inline]
    pub const fn a(&self) -> f32 {
        return self.0[1];
    }
    /// Get the Color's the `b` channel value.
    #[inline]
    pub const fn b(&self) -> f32 {
        return self.0[2];
    }
    /// Set the Color's the `L` channel's value.
    #[inline]
    pub const fn set_l(&mut self, value: f32) {
        self.0[0] = value;
    }
    /// Set the Color's the `a` channel's value.
    #[inline]
    pub const fn set_a(&mut self, value: f32) {
        self.0[0] = value;
    }
    /// Set the Color's the `b` channel's value.
    #[inline]
    pub const fn set_b(&mut self, value: f32) {
        self.0[2] = value;
    }
}

impl_color_array! {
    name: OkLab,
    channels: 3,
    extra_args: {},
    generics: {},
    gen_use: {}
}

impl ColorSpace for OkLab {}
unsafe impl ColorTransmute for OkLab {}

impl Default for OkLab {
    #[inline]
    fn default() -> Self {
        Self([0.0, 0.0, 0.0])
    }
}

impl ColorData for OkLab {
    type WhitePoint = D65;
    const DEFAULT: Self = Self([0.0, 0.0, 0.0]);
    const CHANNELS: usize = 3;
    const LINEAR: bool = true;
    const CHANNEL_MAX: &'static [BoundF32] = &[
        BoundF32::Include(1.0),
        BoundF32::Include(0.5),
        BoundF32::Include(0.5),
    ];
    const CHANNEL_MIN: &'static [BoundF32] = &[
        BoundF32::Include(0.0),
        BoundF32::Include(-0.5),
        BoundF32::Include(-0.5),
    ];
}

impl FromColor<Xyz<D65>> for OkLab {
    fn from_color(color: Xyz<D65>) -> Self {
        let mut lms = matrix_3x3_vec3_mul(&Self::M1, color.as_slice());
        for v in &mut lms {
            *v = cbrtf(*v);
        }
        return Self(matrix_3x3_vec3_mul(&Self::M2, &lms));
    }
}

impl FromColor<OkLab> for Xyz<D65> {
    fn from_color(color: OkLab) -> Self {
        let mut lms = matrix_3x3_vec3_mul(&OkLab::M2_INV, &color.0);
        for v in &mut lms {
            let ch = *v;
            *v = ch * ch * ch;
        }
        return Xyz::from_array(matrix_3x3_vec3_mul(&OkLab::M1_INV, &lms));
    }
}
