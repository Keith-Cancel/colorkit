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
    /// Change the white point of the XYZ color without
    /// any chromatic adaptation.
    ///
    /// All numeric values are left unchanged.
    #[inline]
    pub const fn change_white_point<Wp: WhitePoint>(self) -> Color<Xyz<Wp>> {
        return Color(self.0);
    }
}

#[cfg(test)]
mod test {
    use colorkit::space::*;

    use super::*;
    #[test]
    fn new_xyz() {
        let mut c = Color::<Xyz<D65>>::new_xyz(0.2, 0.3, 0.4);
        assert_eq!(c.channels(), 3);
        assert_eq!(c.x(), 0.2);
        assert_eq!(c.y(), 0.3);
        assert_eq!(c.z(), 0.4);
        assert_eq!(c[0], 0.2);
        assert_eq!(c[1], 0.3);
        assert_eq!(c[2], 0.4);

        c.set_x(0.5);
        assert_eq!(c.x(), 0.5);
        assert_eq!(c.y(), 0.3);
        assert_eq!(c.z(), 0.4);
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.3);
        assert_eq!(c[2], 0.4);

        c.set_y(0.6);
        c.set_z(0.7);
        assert_eq!(c.x(), 0.5);
        assert_eq!(c.y(), 0.6);
        assert_eq!(c.z(), 0.7);
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.6);
        assert_eq!(c[2], 0.7);
    }
}
