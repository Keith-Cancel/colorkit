use core::marker::PhantomData;

use colorkit::wp::WhitePoint;

/// Represention of an CIE XYZ color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Xyz<Wp: WhitePoint>([f32; 3], PhantomData<Wp>);

impl<Wp: WhitePoint> Xyz<Wp> {
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
}
