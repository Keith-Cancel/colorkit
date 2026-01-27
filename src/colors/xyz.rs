use core::marker::PhantomData;

use colorkit::ColorData;
use colorkit::ColorSpace;
use colorkit::space2::ChannelBound;
use colorkit::space2::XyzConvert;
use colorkit::wp::WhitePoint;

use super::macros::impl_color_array;

/// Represention of an CIE XYZ color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
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
        self.0[0] = value;
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
    const DEFAULT: Self = Self([0.0, 0.0, 0.0], PhantomData);
    type WhitePoint = W;
    const LINEAR: bool = true;
    const CHANNEL_MAX: &'static [ChannelBound] = &[ChannelBound::Unbounded; 3];
    const CHANNEL_MIN: &'static [ChannelBound] = &[ChannelBound::Included(0.0); 3];
}

impl<W: WhitePoint> XyzConvert for Xyz<W> {
    #[inline]
    fn from_xyz(color: Xyz<W>) -> Self {
        return color;
    }
    fn into_xyz(self) -> Xyz<W> {
        return self;
    }
}

impl<W: WhitePoint> ColorSpace for Xyz<W> {}
