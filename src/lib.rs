#![no_std]
#![cfg_attr(feature = "type_const", allow(incomplete_features))]
#![cfg_attr(feature = "type_const", feature(min_generic_const_args, register_tool))]
#![cfg_attr(feature = "type_const", register_tool(type_const))] // Make rust analyzer not show this an error.
pub extern crate self as colorkit;

#[rustfmt::skip]
pub mod colors;
pub mod convert;
pub mod math;
pub mod scalar;
pub mod space;
pub mod wp;
//pub mod channels;
//pub mod layout;
//pub mod utils;
