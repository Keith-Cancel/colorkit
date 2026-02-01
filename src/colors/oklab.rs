use colorkit::convert::ColorTransmute;
use colorkit::convert::FromColor;
use colorkit::convert::IntoColor;
use colorkit::layout::Layout;
use colorkit::layout::LayoutMap;
use colorkit::math::BoundF32;
use colorkit::math::cbrtf;
use colorkit::math::matrix_3x3_vec3_mul;
use colorkit::num_type::Number;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::space::ColorData;
use colorkit::space::ColorLayout;
use colorkit::space::ColorSpace;
use colorkit::wp::D65;

use super::LinSrgb;
use super::Srgb;
use super::Xyz;
use super::macros::impl_color_array;

/// Represention of an OkLab color using [`f32`] values.
///
/// Oklab's `a` and `b` channels are unbounded in theory, but
/// some operations require bounds. So any operations that
/// require bounds use `-0.5` and `0.5` as the bounds.
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
    /// Linear sRGB to LMS
    #[rustfmt::skip]
    pub const L_SRGB_LMS: [f32; 9] = [
        0.4122709801829559, 0.5362975720566723, 0.0514314477603717,
        0.2119393935361401, 0.6806887252266035, 0.1073718812372563,
        0.0883294833137031, 0.2817528442280716, 0.6299176724582253,
    ];
    /// LMS to linear sRGB, the inverse of [`OkLab::L_SRGB_LMS`]
    #[rustfmt::skip]
    pub const LMS_L_SRGB: [f32; 9] = [
         4.0762520493926093, -3.3071512880696345,  0.2308992386770251,
        -1.2685206721376014,  2.6097988538914582, -0.3412781817538568,
        -0.0041972628456039, -0.7035828725078884,  1.7077801353534923,
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

    fn rgb_into_lms(rgb: [f32; 3]) -> [f32; 3] {
        return matrix_3x3_vec3_mul(&Self::L_SRGB_LMS, &rgb);
    }

    fn xyz_into_lms(xyz: [f32; 3]) -> [f32; 3] {
        return matrix_3x3_vec3_mul(&Self::M1, &xyz);
    }

    fn lms_into_xyz(lms: [f32; 3]) -> [f32; 3] {
        return matrix_3x3_vec3_mul(&Self::M1_INV, &lms);
    }

    fn lms_into_rgb(lms: [f32; 3]) -> [f32; 3] {
        return matrix_3x3_vec3_mul(&Self::LMS_L_SRGB, &lms);
    }

    fn lms_into_lab(mut lms: [f32; 3]) -> [f32; 3] {
        for v in &mut lms {
            *v = cbrtf(*v);
        }
        return matrix_3x3_vec3_mul(&OkLab::M2, &lms);
    }

    fn lab_into_lms(lab: [f32; 3]) -> [f32; 3] {
        let mut lms = matrix_3x3_vec3_mul(&OkLab::M2_INV, &lab);
        for v in &mut lms {
            let ch = *v;
            *v = ch * ch * ch;
        }
        return lms;
    }
}

impl_color_array! {
    name: OkLab,
    channels: 3,
    extra_args: {},
    generics: {},
    gen_use: {}
}

impl ColorSpace for OkLab {
    /// Return the channel at `index` normalized into `[0.0, 1.0]`.
    ///
    /// Oklab `a` and `b` channels are theoretically unbounded, but for
    /// normalization a practical range of `[-0.5, 0.5]` is assumed.
    fn get_norm(&self, index: usize) -> NormF32 {
        let v = self.0[index];
        let v = if index > 0 { v + 0.5 } else { v };
        return NormF32::new(v);
    }
}

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
    // Oklab a, and b channels in theory are unbounded
    // but at least from understanding the practical
    // range is only -0.5 to 0.5.
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

impl ColorLayout for OkLab {
    /// Create an instance of [`OkLab`] from a [`Layout`]
    ///
    /// Oklab's channels `a` and `b` are in theory unbounded this
    /// implementation assumes that range -0.5 to 0.5 inclusive.
    ///
    /// If we have some kind of quantization of Oklab a bound of
    /// some kind would have been used. For example CSS uses
    /// -0.4 to 0.4. If that range was reduced to like a [`u8`]
    /// `-0.4` would be `0`, and `0.4` would `255`.
    ///
    /// Because of this ambiguity you might need to rescale the
    /// [`Layout`] or the resulting [`OkLab`] if the range is
    /// different.
    ///
    /// This calls `get_norm()` on the layout and shifts value by `+0.5`
    fn from_layout<L: Layout>(layout: L) -> Self {
        debug_assert!(<L::Channels as Number>::N >= 3);
        let l = layout.get_norm(0).get();
        let a = layout.get_norm(1).get();
        let b = layout.get_norm(2).get();
        return Self([l, a, b]);
    }

    fn from_layout_map<L: Layout, M: LayoutMap<Channels = L::Channels>>(layout: L) -> Self {
        debug_assert!(<L::Channels as Number>::N >= 3);
        let l = layout.get_norm(M::map(0)).get();
        let a = layout.get_norm(M::map(1)).get();
        let b = layout.get_norm(M::map(2)).get();
        return Self([l, a, b]);
    }

    fn into_layout<L: Layout>(self, round: Rounding) -> L {
        debug_assert!(<L::Channels as Number>::N == 3);
        let a = [
            NormF32::new(self.l()),
            NormF32::new(self.a() + 0.5),
            NormF32::new(self.b() + 0.5),
        ];
        return L::from_fn_norm(|i| a[i], round);
    }
}

impl FromColor<Xyz<D65>> for OkLab {
    fn from_color(color: Xyz<D65>) -> Self {
        let lms = Self::xyz_into_lms(color.into_array());
        return Self(Self::lms_into_lab(lms));
    }
}

impl FromColor<OkLab> for Xyz<D65> {
    fn from_color(color: OkLab) -> Self {
        let lms = OkLab::lab_into_lms(color.0);
        return <Xyz<D65>>::from_array(OkLab::lms_into_xyz(lms));
    }
}

impl FromColor<LinSrgb> for OkLab {
    fn from_color(color: LinSrgb) -> Self {
        let lms = Self::rgb_into_lms(color.into_array());
        return Self(Self::lms_into_lab(lms));
    }
}

impl FromColor<OkLab> for LinSrgb {
    fn from_color(color: OkLab) -> Self {
        let lms = OkLab::lab_into_lms(color.0);
        return LinSrgb::from_array(OkLab::lms_into_rgb(lms));
    }
}

impl FromColor<Srgb> for OkLab {
    fn from_color(color: Srgb) -> Self {
        return color.into_linear().into_color();
    }
}

impl FromColor<OkLab> for Srgb {
    fn from_color(color: OkLab) -> Self {
        let c = LinSrgb::from_color(color);
        return c.into_nonlinear();
    }
}

#[cfg(test)]
mod test {
    use colorkit::wp::WhitePoint;

    use super::*;
    use crate::convert::IntoColor;
    use crate::math::MathFuncs;

    #[test]
    fn wp_to_lms() {
        let wp = D65::color().into_array();
        let lms = OkLab::xyz_into_lms(wp);
        assert_eq!(lms, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn white_and_black() {
        let lab = OkLab::new(1.0, 0.0, 0.0);
        let xyz = lab.into_xyz().into_array();
        let wp = D65::color().into_array();
        assert!(wp[0].almost_eq(xyz[0], 2e-7));
        assert!(wp[1].almost_eq(xyz[1], 2e-7));
        assert!(wp[2].almost_eq(xyz[2], 2e-7));

        let lab = OkLab::new(0.0, 0.0, 0.0);
        let xyz = lab.into_xyz().into_array();
        assert_eq!(xyz, [0.0, 0.0, 0.0]);
    }
    // Examples from:
    // https://bottosson.github.io/posts/oklab/
    #[test]
    fn oklab_examples() {
        let c: OkLab = <Xyz<D65>>::new(1.0, 0.0, 0.0).into_color();
        // He only gives 3 digits of precision, in his blog post
        // but still good check to make sure we are in the right
        // ball park.
        assert!(c[0].almost_eq(0.450, 1e-3));
        assert!(c[1].almost_eq(1.236, 1e-3));
        assert!(c[2].almost_eq(-0.019, 1e-3));

        let c: OkLab = <Xyz<D65>>::new(0.0, 1.0, 0.0).into_color();
        assert!(c[0].almost_eq(0.922, 1e-3));
        assert!(c[1].almost_eq(-0.671, 1e-3));
        assert!(c[2].almost_eq(0.263, 1e-3));

        let c: OkLab = <Xyz<D65>>::new(0.0, 0.0, 1.0).into_color();
        assert!(c[0].almost_eq(0.153, 1e-3));
        assert!(c[1].almost_eq(-1.415, 1e-3));
        assert!(c[2].almost_eq(-0.449, 1e-3));
    }

    // Just rough sanity check.
    #[test]
    fn rgb() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let lab = OkLab::from_color(red);

        assert!(lab[0].almost_eq(0.628, 1e-3));
        assert!(lab[1].almost_eq(0.225, 1e-3));
        assert!(lab[2].almost_eq(0.126, 1e-3));

        let grn = Srgb::new(0.0, 1.0, 0.0);
        let lab = OkLab::from_color(grn);
        assert!(lab[0].almost_eq(0.866, 1e-3));
        assert!(lab[1].almost_eq(-0.234, 1e-3));
        assert!(lab[2].almost_eq(0.179, 1e-3));

        let blu = Srgb::new(0.0, 0.0, 1.0);
        let lab = OkLab::from_color(blu);
        assert!(lab[0].almost_eq(0.452, 1e-3));
        assert!(lab[1].almost_eq(-0.032, 1e-3));
        assert!(lab[2].almost_eq(-0.312, 1e-3));

        let mag = Srgb::new(1.0, 0.0, 1.0);
        let lab = OkLab::from_color(mag);
        assert!(lab[0].almost_eq(0.702, 1e-3));
        assert!(lab[1].almost_eq(0.275, 1e-3));
        assert!(lab[2].almost_eq(-0.169, 1e-3));
    }
}
