use colorkit::math::quirtf;
use colorkit::space::LinSrgb;
use colorkit::space::RgbLike;
use colorkit::space::Srgb;

use super::Color;

impl<S: RgbLike> Color<S> {
    /// Create a new color from R, G, B values.
    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        return Self::crate_new([r, g, b]);
    }
    /// Get `Red` value.
    pub const fn red(&self) -> f32 {
        return self.0.as_slice()[0];
    }
    /// Get `Green` value.
    pub const fn green(&self) -> f32 {
        return self.0.as_slice()[1];
    }
    /// Get `Blue` value.
    pub const fn blue(&self) -> f32 {
        return self.0.as_slice()[2];
    }
    /// Set `Red` value.
    pub const fn set_red(&mut self, value: f32) {
        self.0.as_mut_slice()[0] = value;
    }
    /// Set `Green` value.
    pub const fn set_green(&mut self, value: f32) {
        self.0.as_mut_slice()[1] = value;
    }
    /// Set `Blue` value.
    pub const fn set_blue(&mut self, value: f32) {
        self.0.as_mut_slice()[2] = value;
    }
}

impl Color<Srgb> {
    pub const fn into_linear(self) -> Color<LinSrgb> {
        let [r, g, b] = self.clamp().crate_inner();
        return Color::crate_new([linear(r), linear(g), linear(b)]);
    }
}

// https://entropymine.com/imageworsener/srgbformula/
const fn linear(s: f32) -> f32 {
    // 0.04045 old
    let l = if s <= 0.0404482362771082 {
        s / 12.92
    } else {
        let x = (s + 0.055) / 1.055;
        // Equals x.powf(2.4)
        let x2 = x * x;
        x2 * quirtf(x2)
    };
    return l;
}

#[cfg(test)]
mod test {
    use colorkit::space::*;

    use super::*;
    #[test]
    fn new_rgb() {
        let mut c = Color::<Srgb>::new_rgb(0.2, 0.3, 0.4);
        assert_eq!(c.channels(), 3);
        assert_eq!(c.red(), 0.2);
        assert_eq!(c.green(), 0.3);
        assert_eq!(c.blue(), 0.4);
        assert_eq!(c[0], 0.2);
        assert_eq!(c[1], 0.3);
        assert_eq!(c[2], 0.4);

        c.set_red(0.5);
        assert_eq!(c.red(), 0.5);
        assert_eq!(c.green(), 0.3);
        assert_eq!(c.blue(), 0.4);
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.3);
        assert_eq!(c[2], 0.4);

        c.set_blue(0.7);
        c.set_green(0.6);
        assert_eq!(c.red(), 0.5);
        assert_eq!(c.green(), 0.6);
        assert_eq!(c.blue(), 0.7);
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.6);
        assert_eq!(c[2], 0.7);
    }

    #[test]
    fn linear() {
        let c = Color::<Srgb>::new_rgb(0.34117647058, 0.89019607843, 0.53725490196);
        let c = c.into_linear();
        assert!(c[0] >= 0.0953074);
        assert!(c[0] <= 0.0953075);
    }
}
