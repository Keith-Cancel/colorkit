use colorkit::ColorSpace;
use colorkit::ColorTransmute;
use colorkit::math::cbrtf;
use colorkit::math::quirtf;
use colorkit::math::sqrtf;
use colorkit::space2::ChannelBound;
use colorkit::wp::D65;

use super::macros::impl_color_array;

macro_rules! base_funcs {
    ($name:ident, $len:expr) => {
        impl $name {
            /// Create a new color from RGB values.
            #[inline]
            pub const fn new(r: f32, g: f32, b: f32) -> Self {
                return Self([r, g, b]);
            }
            /// Get the red channel's value.
            #[inline]
            pub const fn red(&self) -> f32 {
                return self.0[0];
            }
            /// Get the blue channel's value.
            #[inline]
            pub const fn blue(&self) -> f32 {
                return self.0[1];
            }
            /// Get the green channel's value.
            #[inline]
            pub const fn green(&self) -> f32 {
                return self.0[2];
            }
            /// Set the red channel's value.
            #[inline]
            pub const fn set_red(&mut self, value: f32) {
                self.0[0] = value;
            }
            /// Set the blue channel's value.
            #[inline]
            pub const fn set_blue(&mut self, value: f32) {
                self.0[1] = value;
            }

            /// Set the green channel's value.
            #[inline]
            pub const fn set_green(&mut self, value: f32) {
                self.0[2] = value;
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                return Self([0.0, 0.0, 0.0]);
            }
        }

        impl ColorSpace for $name {
            const DEFAULT: Self = Self([0.0, 0.0, 0.0]);
            type WhitePoint = D65;
            const LINEAR: bool = true;
            const CHANNEL_MAX: &'static [ChannelBound] = &[ChannelBound::Included(1.0); 3];
            const CHANNEL_MIN: &'static [ChannelBound] = &[ChannelBound::Included(0.0); 3];
        }
    };
}

/// Represention of an Srgb color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Srgb([f32; 3]);

impl Srgb {
    /// Convert Srgb into Linear Srgb
    pub const fn into_linear(self) -> LinSrgb {
        return LinSrgb([
            linear(self.0[0]),
            linear(self.0[1]),
            linear(self.0[2]),
        ]);
    }
}

base_funcs!(Srgb, 3);
impl_color_array!(Srgb, 3);

/// Represention of a Linear Srgb color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct LinSrgb([f32; 3]);

impl LinSrgb {
    /// Convert Linear Srgb into Srgb
    pub fn into_nonlinear(self) -> Srgb {
        return Srgb([
            nonlinear(self.0[0]),
            nonlinear(self.0[1]),
            nonlinear(self.0[2]),
        ]);
    }
}

base_funcs!(LinSrgb, 3);
impl_color_array!(LinSrgb, 3);

fn nonlinear(l: f32) -> f32 {
    // 0.0031308 old
    let s = if l <= 0.00313066844250063 {
        l * 12.92
    } else {
        let sq = sqrtf(l);
        let cb = cbrtf(l);
        let c = sqrtf(sq) * sqrtf(cb);

        1.055 * c - 0.055
    };
    return s;
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

impl From<Srgb> for LinSrgb {
    fn from(value: Srgb) -> Self {
        return value.into_linear();
    }
}

impl From<LinSrgb> for Srgb {
    fn from(value: LinSrgb) -> Self {
        return value.into_nonlinear();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn linear() {
        let c = Srgb::new(0.34117647058, 0.89019607843, 0.53725490196);
        let c = c.into_linear();
        assert!(c[0] >= 0.0953074);
        assert!(c[0] <= 0.0953075);

        // These values have an exact representation so should not be lost.
        let c = Srgb::new(0.5, 0.75, 0.125);
        let c = c.into_linear();
        assert!(c[0] >= 0.214 && c[0] <= 0.2141);
        let c = c.into_nonlinear();
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.75);
        assert_eq!(c[2], 0.125);
    }
}
