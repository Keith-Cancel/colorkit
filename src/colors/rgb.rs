use colorkit::ColorSpace;
use colorkit::ColorTransmute;

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
    };
}

/// Represention of an Srgb color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Srgb([f32; 3]);

base_funcs!(Srgb, 3);
impl_color_array!(Srgb, 3);

/// Represention of a Linear Srgb color using [`f32`] values.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct LinSrgb([f32; 3]);

base_funcs!(LinSrgb, 3);
impl_color_array!(LinSrgb, 3);
