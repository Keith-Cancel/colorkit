//! Colorspaces like [`Srgb`], [`OkLab`], [`Xyz`] ect...
mod alpha;
mod oklab;
mod oklch;
mod rgb;
mod xyz;

#[rustfmt::skip]
pub(crate) mod macros;

pub use alpha::Alpha;
pub use alpha::AlphaPre;
pub use alpha::AlphaWrap;
pub use oklab::OkLab;
pub use rgb::LinSrgb;
pub use rgb::Srgb;
pub use xyz::Xyz;
