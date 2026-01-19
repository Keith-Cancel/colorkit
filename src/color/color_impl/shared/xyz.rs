use colorkit::space::WhitePoint;
use colorkit::space::Xyz;

use super::Color;

impl<W: WhitePoint> Color<Xyz<W>> {
    /// Create a new color from X, Y, Z values.
    pub const fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        return Self::crate_new([x, y, z]);
    }
    /// Get `X` value.
    pub const fn x(&self) -> f32 {
        return self.0.as_slice()[0];
    }
    /// Get `Y` value.
    pub const fn y(&self) -> f32 {
        return self.0.as_slice()[1];
    }
    /// Get `Z` value.
    pub const fn z(&self) -> f32 {
        return self.0.as_slice()[2];
    }
    /// Set `X` value.
    pub const fn set_x(&mut self, value: f32) {
        self.0.as_mut_slice()[0] = value;
    }
    /// Set `Y` value.
    pub const fn set_y(&mut self, value: f32) {
        self.0.as_mut_slice()[1] = value;
    }
    /// Set `Z` value.
    pub const fn set_z(&mut self, value: f32) {
        self.0.as_mut_slice()[2] = value;
    }
}
