//! Colorspaces like [`Srgb`], [`OkLab`], [`Xyz`] ect...
mod alpha2;
mod oklab;
mod rgb;
mod xyz;

#[rustfmt::skip]
pub(crate) mod macros;

pub use alpha2::Alpha;
pub use alpha2::AlphaPre;
pub use alpha2::AlphaWrap;
pub use oklab::OkLab;
pub use rgb::LinSrgb;
pub use rgb::Srgb;
pub use xyz::Xyz;
