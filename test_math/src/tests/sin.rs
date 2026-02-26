use std::f32::consts::PI;

use colorkit::math::sinf_on_pi;
use rug::Float;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Sin;

impl MathFn for Sin {
    const NAME: &'static str = "sin_on_pi(x)";
    const ALLOW_NEG: bool = true;
    const ALLOW_ZERO: bool = true;
    const MAX: f32 = PI;
    const MIN: f32 = -PI;

    fn test_f32_impl(x: f32) -> f32 {
        return sinf_on_pi(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.sin();
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        return Float::with_val(prec, x).sin();
    }
}
