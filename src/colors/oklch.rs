use core::f32::consts::FRAC_1_SQRT_2;
use core::f32::consts::PI;
use core::f32::consts::TAU;

use colorkit::convert::*;
use colorkit::layout::*;
use colorkit::math::*;
use colorkit::num_type::N3;
use colorkit::num_type::Number;
use colorkit::scalar::*;
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

impl OkLch {
    const BOUNDS: [(f32, f32); 3] = [
        (0.0, 1.0),
        (0.0, FRAC_1_SQRT_2),
        (-PI, PI),
    ];
    /// Create a new color from `LCh` values.
    pub const fn new(l: f32, c: f32, h: f32) -> Self {
        return Self([l, c, h]);
    }
    /// Get the Color's the `L` channel value.
    #[inline]
    pub const fn l(&self) -> f32 {
        return self.0[0];
    }
    /// Get the Color's the `C` channel value.
    #[inline]
    pub const fn c(&self) -> f32 {
        return self.0[1];
    }
    /// Get the Color's the `h` channel angle.
    #[inline]
    pub const fn h(&self) -> f32 {
        return self.0[2];
    }
    /// Set the Color's the `L` channel's value.
    #[inline]
    pub const fn set_l(&mut self, value: f32) {
        self.0[0] = value;
    }
    /// Set the Color's the `C` channel's value.
    #[inline]
    pub const fn set_c(&mut self, value: f32) {
        self.0[1] = value;
    }
    /// Set the Color's the `h` channel angle.
    #[inline]
    pub const fn set_h(&mut self, value: f32) {
        let value = value.clamp(-PI, PI);
        self.0[2] = value;
    }
}

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
        BoundF32::Include(FRAC_1_SQRT_2),
        BoundF32::Include(PI),
    ];

    const CHANNEL_MIN: [BoundF32; 3] = [
        BoundF32::Include(0.0),
        BoundF32::Include(0.0),
        BoundF32::Include(-PI),
    ];
}

impl ColorBounds for OkLch {
    fn clamp(self) -> Self {
        let mut a = self.0;
        for (i, v) in a.iter_mut().enumerate() {
            let b = Self::BOUNDS[i];
            *v = v.clamp(b.0, b.1);
        }
        return Self::from_array(a);
    }
    fn clamp_channel(self, index: usize) -> Self {
        let mut a = self.0;
        let b = Self::BOUNDS[index];
        a[index] = a[index].clamp(b.0, b.1);
        return Self::from_array(a);
    }
    fn is_clamped(&self) -> bool {
        for i in 0..3 {
            let b = Self::BOUNDS[i];
            let v = self.0[i];
            if v < b.0 || v > b.1 {
                return false;
            }
        }
        return true;
    }
    #[inline]
    fn is_channel_clamped(&self, index: usize) -> bool {
        let c = self.0[index];
        let b = Self::BOUNDS[index];
        return c >= b.0 && c <= b.1;
    }
}

impl ColorNorm for OkLch {
    /// Get [`OkLch`] channels normalized into `[0.0, 1.0]`.
    ///
    /// Oklch `c` and `h` use the bounds of `[0.0, sqrt(.5)]`, and [-pi, pi]
    fn into_norm(self) -> [NormF32; 3] {
        let c = self[1] / FRAC_1_SQRT_2;
        let h = (self[2] + PI) / TAU;
        return [
            NormF32::new(self[0]),
            NormF32::new(c),
            NormF32::new(h),
        ];
    }
    /// Create an [`OkLch`] from channels normalized into `[0.0, 1.0]`.
    #[inline]
    fn from_norm<T: AsRef<[NormF32]>>(values: T) -> Self {
        let v = values.as_ref();
        let c = v[1] * FRAC_1_SQRT_2;
        let h = (v[2] * TAU) - PI;
        return Self([v[0].get(), c, h]);
    }
}

impl ColorLayout for OkLch {
    fn from_layout<L: Layout>(layout: L) -> Self {
        debug_assert!(<L::Channels as Number>::N >= 3);
        let l = layout.get_norm(0);
        let a = layout.get_norm(1);
        let b = layout.get_norm(2);
        return Self::from_norm([l, a, b]);
    }

    fn into_layout<L: Layout<Channels = Self::Channels>>(self, round: Rounding) -> L {
        let n = self.into_norm();
        return L::from_fn_norm(|i| n[i], round);
    }

    fn into_layout_dither<L: Layout<Channels = Self::Channels>, D: Dither>(
        self,
        round: Rounding,
        dither: &mut D,
    ) -> L {
        let n = self.into_norm();
        return L::from_fn_norm_dither(|i| n[i], round, dither);
    }
}

impl AlphaNone for OkLch {}
impl ColorSpace for OkLch {}
impl ColorSlice for OkLch {}
unsafe impl ColorTransmute for OkLch {}

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

impl FromColor<LinSrgb> for OkLch {
    fn from_color(color: LinSrgb) -> Self {
        let c: OkLab = color.into_color();
        return c.into_color();
    }
}

impl FromColor<OkLch> for LinSrgb {
    fn from_color(color: OkLch) -> Self {
        let c: OkLab = color.into_color();
        return c.into_color();
    }
}

impl FromColor<Srgb> for OkLch {
    fn from_color(color: Srgb) -> Self {
        let c: OkLab = color.into_color();
        return c.into_color();
    }
}

impl FromColor<OkLch> for Srgb {
    fn from_color(color: OkLch) -> Self {
        let c: OkLab = color.into_color();
        return c.into_color();
    }
}
