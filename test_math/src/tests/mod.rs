mod quirt;
mod sqrt;

pub use quirt::Quirt;
use rug::Float;
pub use sqrt::Sqrt;

/// Single Agrument Math functions
pub trait MathFn {
    /// Display name used in test outputs
    const NAME: &'static str;
    /// Whether or not the function allows negative values
    const ALLOW_NEG: bool;
    /// Whether or not the function allows zero.
    const ALLOW_ZERO: bool;
    /// Implementation to be tested.
    fn test_f32_impl(x: f32) -> f32;
    /// Stdlib for upperbound reference
    fn std_f32_impl(x: f32) -> f32;
    /// High-precision reference at `prec` bits
    fn rug_impl(prec: u32, x: f32) -> Float;
}
