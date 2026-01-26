#![no_std]
#![cfg_attr(feature = "type_const", allow(incomplete_features))]
#![cfg_attr(feature = "type_const", feature(min_generic_const_args, register_tool))]
#![cfg_attr(feature = "type_const", register_tool(type_const))] // Make rust analyzer not show this an error.
pub extern crate self as colorkit;

mod space2;

#[rustfmt::skip]
pub use space2::ColorSpace;
pub use space2::ColorArray;
pub use space2::ColorTransmute;
pub use space2::RgbLike;
pub use space2::white_point as wp;
pub mod colors;
//pub mod channels;
//pub mod layout;
//pub mod scalar;
pub mod color;
pub mod math;
pub mod space;
pub mod utils;
