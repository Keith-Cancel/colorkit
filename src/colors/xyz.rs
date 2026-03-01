use core::marker::PhantomData;

use colorkit::convert::*;
use colorkit::layout::Layout;
use colorkit::math::BoundF32;
use colorkit::num_type::N3;
use colorkit::num_type::Number;
use colorkit::scalar::Dither;
use colorkit::scalar::Rounding;
use colorkit::space::*;
use colorkit::wp::WhitePoint;

use super::macros::*;
use crate::scalar::NormF32;

/// Represention of an CIE XYZ color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Xyz<Wp: WhitePoint>([f32; 3], PhantomData<Wp>);

impl<W: WhitePoint> Xyz<W> {
    /// Create a new color from XYZ values.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        return Self([x, y, z], PhantomData);
    }
    /// Get the Color's the `X` channel value.
    #[inline]
    pub const fn x(&self) -> f32 {
        return self.0[0];
    }
    /// Get the Color's the `Y` channel value.
    #[inline]
    pub const fn y(&self) -> f32 {
        return self.0[1];
    }
    /// Get the Color's the `Z` channel value.
    #[inline]
    pub const fn z(&self) -> f32 {
        return self.0[2];
    }
    /// Set the Color's the `X` channel's value.
    #[inline]
    pub const fn set_x(&mut self, value: f32) {
        self.0[0] = value;
    }
    /// Set the Color's the `Y` channel's value.
    #[inline]
    pub const fn set_y(&mut self, value: f32) {
        self.0[1] = value;
    }
    /// Set the Color's the `Z` channel's value.
    #[inline]
    pub const fn set_z(&mut self, value: f32) {
        self.0[2] = value;
    }
    /// Change the white point of the XYZ color without
    /// any chromatic adaptation.
    ///
    /// All numeric values are left unchanged.
    #[inline]
    pub const fn change_white_point<Wp: WhitePoint>(self) -> Xyz<Wp> {
        return Xyz::<Wp>(self.0, PhantomData);
    }
}

impl_color_new!([f32; 3], Xyz<Wp: WhitePoint>, PhantomData);
impl_from_inner!([f32; 3], Xyz<Wp: WhitePoint>, PhantomData);
impl_self_index!(Xyz<Wp: WhitePoint>);
impl_from_tup3!(Xyz<Wp: WhitePoint>);
impl_typ_as_self!(Xyz<Wp: WhitePoint>, [f32; 3]);
impl_self_as_typ!([f32], Xyz<Wp: WhitePoint>);
impl_self_as_typ!([f32; 3], Xyz<Wp: WhitePoint>);

impl<W: WhitePoint> Default for Xyz<W> {
    #[inline]
    fn default() -> Self {
        return Self([0.0, 0.0, 0.0], PhantomData);
    }
}

impl<W: WhitePoint> ColorData for Xyz<W> {
    type WhitePoint = W;
    type Channels = N3;
    const LINEAR: bool = true;
    const CHANNEL_MAX: [BoundF32; 3] = [BoundF32::Unbounded; 3];
    const CHANNEL_MIN: [BoundF32; 3] = [BoundF32::Include(0.0); 3];
}

impl<W: WhitePoint> ColorLayout for Xyz<W> {
    /// Create an instance of CIE XYZ from a [`Layout`]
    ///
    /// XYZ channels are unbounded, so if we have some
    /// kinda of quantization of `XYZ` it safe to assume
    /// it was normalized relative to some white point.
    ///
    /// This calls `get_norm()` on the layout and scales
    /// each channel by the white point.
    fn from_layout<L: Layout, T: AsRef<L>>(layout: &T) -> Self {
        debug_assert!(<L::Channels as Number>::N >= 3);
        let lay = layout.as_ref();
        let x = lay.get_norm(0) * W::X;
        let y = lay.get_norm(1) * W::Y;
        let z = lay.get_norm(2) * W::Z;
        return Self([x, y, z], PhantomData);
    }

    /// Create a [`Layout`] from a CIE XYZ color.
    ///
    /// XYZ channels are unbounded, so to quantize the
    /// `XYZ` the values are normalized relative the
    /// white point, any values larger are clamped.
    fn into_layout<L: Layout>(self, round: Rounding) -> L {
        debug_assert!(<L::Channels as Number>::N == 3);
        let n = self.into_norm();
        return L::from_fn_norm(|i| n[i], round);
    }

    /// Create a [`Layout`] from a CIE XYZ color and [`Dither`]
    ///
    /// XYZ channels are unbounded, so to quantize the
    /// `XYZ` the values are normalized relative the
    /// white point, any values larger are clamped.
    fn into_layout_dither<L: Layout, D: Dither>(self, round: Rounding, dither: &mut D) -> L {
        debug_assert!(<L::Channels as Number>::N == 3);
        let n = self.into_norm();
        return L::from_fn_norm_dither(|i| n[i], round, dither);
    }
}

impl<W: WhitePoint> ColorBounds for Xyz<W> {
    fn clamp(self) -> Self {
        let [x, y, z] = self.0;
        // XYZ only has a lower a bound of zero.
        return Self::from_array([x.max(0.0), y.max(0.0), z.max(0.0)]);
    }
    fn clamp_channel(self, index: usize) -> Self {
        let mut a = self.0;
        a[index] = a[index].max(0.0);
        return Self::from_array(a);
    }
    fn is_clamped(&self) -> bool {
        for v in self.0 {
            if v < 0.0 {
                return false;
            }
        }
        return true;
    }
    #[inline]
    fn is_channel_clamped(&self, index: usize) -> bool {
        return self.0[index] >= 0.0;
    }
}

impl<W: WhitePoint> ColorNorm for Xyz<W> {
    /// Return the color normalized into `[0.0, 1.0]`.
    ///
    /// CIE XYZ channels are unbounded, so the `XYZ` values are
    /// normalized relative the white point, and then clamped to
    /// the normalized range.
    ///
    /// The value is computed by dividing the tristimulus channels by
    /// the reference white component (`X / W::X`, `Y / W::Y`, `Z / W::Z`)
    /// and then clamped.
    fn into_norm(self) -> [NormF32; 3] {
        let w = [W::X, W::Y, W::Z];
        return [
            NormF32::new(self[0] / w[0]),
            NormF32::new(self[1] / w[1]),
            NormF32::new(self[2] / w[2]),
        ];
    }
    /// Create an [`Xyz`] color from channels normalized into `[0.0, 1.0]`.
    ///
    /// CIE XYZ channels are unbounded, so the normalized `XYZ` values
    /// as assumed to have normalized relative to the white point.
    /// normalized relative the white point.
    ///
    /// The value is computed by mulitplying the tristimulus channels by
    /// the reference white component (`X * W::X`, `Y * W::Y`, `Z * W::Z`).
    #[inline]
    fn from_norm<T: AsRef<[NormF32]>>(values: T) -> Self {
        let w = [W::X, W::Y, W::Z];
        let v = values.as_ref();
        return Self::new(v[0] * w[0], v[1] * w[1], v[2] * w[2]);
    }
}

impl<W: WhitePoint> AlphaNone for Xyz<W> {}
impl<W: WhitePoint> ColorSpace for Xyz<W> {}
impl<W: WhitePoint> ColorSlice for Xyz<W> {}
unsafe impl<W: WhitePoint> ColorTransmute for Xyz<W> {}
