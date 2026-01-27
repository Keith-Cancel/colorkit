use colorkit::ColorData;
use colorkit::space2::ChannelBound;
use colorkit::wp::D65;

use super::macros::impl_color_array;

/// Represention of an OkLab color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct OkLab([f32; 3]);

impl OkLab {
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

impl Default for OkLab {
    #[inline]
    fn default() -> Self {
        Self([0.0, 0.0, 0.0])
    }
}

impl ColorData for OkLab {
    type WhitePoint = D65;
    const DEFAULT: Self = Self([0.0, 0.0, 0.0]);
    const LINEAR: bool = true;
    const CHANNEL_MAX: &'static [ChannelBound] = &[
        ChannelBound::Included(1.0),
        ChannelBound::Included(0.5),
        ChannelBound::Included(0.5),
    ];
    const CHANNEL_MIN: &'static [ChannelBound] = &[
        ChannelBound::Included(0.0),
        ChannelBound::Included(-0.5),
        ChannelBound::Included(-0.5),
    ];
}
