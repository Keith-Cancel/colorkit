#![no_std]
#![cfg_attr(feature = "type_const", allow(incomplete_features))]
#![cfg_attr(feature = "type_const", feature(min_generic_const_args, register_tool))]
#![cfg_attr(feature = "type_const", register_tool(type_const))] // Make rust analyzer not show this an error.

// Clippy Config
#![allow(clippy::needless_return)]
// While this lib uses f32 for most everything, I intentially want
// consts that could be used for f64 if I ever change the code
// to use like generics instead of just an f32. Also the compiler
// should round things correctly anyways.
#![allow(clippy::excessive_precision)]

pub extern crate self as colorkit;

#[rustfmt::skip]
pub mod colors;
pub mod convert;
pub mod layout;
pub mod math;
pub mod num_type;
pub mod ops;
pub mod scalar;
pub mod space;
pub mod wp;

// Few exports at the top level for convience.
pub use colors::Alpha;
pub use colors::LinSrgb;
pub use colors::OkLab;
pub use colors::Srgb;
pub use colors::Xyz;
pub use convert::FromColor;
pub use convert::IntoColor;
pub use space::ColorSpace;
