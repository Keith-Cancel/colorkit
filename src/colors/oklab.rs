use colorkit::convert::ColorTransmute;
use colorkit::convert::FromColor;
use colorkit::convert::XyzConvert;
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
    #[rustfmt::skip]
    pub const M1: [f32; 9] = [
        0.8189330101, 0.3618667424, -0.1288597137,
        0.0329845436, 0.9293118715,  0.0361456387,
        0.0482003018, 0.2643662691,  0.6338517070
    ];
    /// The matrix M1's inverse (M2^-1)
    ///
    /// Used to convert OkLab to XYZ
    #[rustfmt::skip]
    pub const M1_INV: [f32; 9] = [
         1.227013851104, -0.557799980652,  0.281256148966,
        -0.040580178423,  1.112256869617, -0.071676678666,
        -0.076381284506, -0.421481978418,  1.586163220441,
    ];
    /// Matrix used as part the conversion From XYZ
    ///
    /// <https://bottosson.github.io/posts/oklab/>
    #[rustfmt::skip]
    pub const M2: [f32; 9] = [
        0.2104542553,  0.7936177850, -0.0040720468,
        1.9779984951, -2.4285922050, 0.4505937099,
        0.0259040371,  0.7827717662, -0.8086757660,
    ];
    /// The matrix M2's inverse (M2^-1)
    ///
    /// Used to convert OkLab to XYZ
    #[rustfmt::skip]
    pub const M2_INV: [f32; 9] = [
        0.999999998451,  0.396337792174,  0.215803758061,
        1.000000008882, -0.105561342324, -0.063854174772,
        1.000000054672, -0.089484182095, -1.291485537864,
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

impl FromColor<Xyz<D65>> for OkLab {
    fn from_color(color: Xyz<D65>) -> Self {
        let mut lms = matrix_3x3_vec3_mul(&Self::M1, color.as_slice());
        for v in &mut lms {
            *v = cbrtf(*v);
        }
        return Self(matrix_3x3_vec3_mul(&Self::M2, &lms));
    }
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

impl XyzConvert for OkLab {
    fn from_xyz(color: super::Xyz<Self::WhitePoint>) -> Self {
        let mut lms = matrix_3x3_vec3_mul(&Self::M1, color.as_slice());
        for v in &mut lms {
            *v = cbrtf(*v);
        }
        return Self(matrix_3x3_vec3_mul(&Self::M2, &lms));
    }
    fn into_xyz(self) -> Xyz<Self::WhitePoint> {
        let mut lms = matrix_3x3_vec3_mul(&Self::M2_INV, &self.0);
        for v in &mut lms {
            let ch = *v;
            *v = ch * ch * ch;
        }
        return Xyz::from_array(matrix_3x3_vec3_mul(&Self::M1_INV, &lms));
    }
}
