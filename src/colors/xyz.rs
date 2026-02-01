use core::marker::PhantomData;

use colorkit::convert::ColorTransmute;
use colorkit::layout::Layout;
use colorkit::math::BoundF32;
use colorkit::space::ColorData;
use colorkit::space::ColorSpace;
use colorkit::wp::WhitePoint;

use super::macros::impl_color_array;
use crate::num_type::Number;

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

    pub(crate) fn from_layout_inner<L: Layout>(lay: L) -> Xyz<W> {
        debug_assert!(<L::Channels as Number>::N >= 3);
        // At least for XYZ the channels are not exactly bounded,
        // but I guess we can assume that each channel was normalized
        // using the white point. So unormalize the channels.
        let x = lay.get_norm(0) * W::X;
        let y = lay.get_norm(1) * W::Y;
        let z = lay.get_norm(2) * W::Z;
        return Self([x, y, z], PhantomData);
    }
}

impl_color_array! {
    name: Xyz,
    channels: 3,
    extra_args: { PhantomData },
    generics: { <W: WhitePoint> },
    gen_use: { <W> }
}

impl<W: WhitePoint> Default for Xyz<W> {
    #[inline]
    fn default() -> Self {
        return Self([0.0, 0.0, 0.0], PhantomData);
    }
}

impl<W: WhitePoint> ColorData for Xyz<W> {
    type WhitePoint = W;
    const DEFAULT: Self = Self([0.0, 0.0, 0.0], PhantomData);
    const CHANNELS: usize = 3;
    const LINEAR: bool = true;
    const CHANNEL_MAX: &'static [BoundF32] = &[BoundF32::Unbounded; 3];
    const CHANNEL_MIN: &'static [BoundF32] = &[BoundF32::Include(0.0); 3];
}

impl<W: WhitePoint> ColorSpace for Xyz<W> {}
unsafe impl<W: WhitePoint> ColorTransmute for Xyz<W> {}
