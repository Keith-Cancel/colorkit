use std::f32::consts::PI;

use colorkit::math::cosf_on_pi;
use rug::Float;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Cos;

impl MathFn for Cos {
    const NAME: &'static str = "cos(x)";
    const ALLOW_NEG: bool = true;
    const ALLOW_ZERO: bool = true;
    const MAX: f32 = PI;
    const MIN: f32 = -PI;

    fn test_f32_impl(x: f32) -> f32 {
        return cosf_on_pi(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.cos();
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        return Float::with_val(prec, x).cos();
    }
}
