use colorkit::ColorSpace;
use colorkit::ColorTransmute;

use super::macros::impl_color_array;

macro_rules! base_funcs {
    ($name:ident, $len:expr) => {
        impl $name {
            /// Create a new color from RGB values.
            pub const fn new(r: f32, b: f32, g: f32) -> Self {
                return Self([r, b, g]);
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
