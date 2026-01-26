use colorkit::math::cbrtf;
use rug::Float;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Cbrt;

impl MathFn for Cbrt {
    const NAME: &'static str = "3rd Root (Cubic Root)";
    const ALLOW_NEG: bool = true;
    const ALLOW_ZERO: bool = true;
    fn test_f32_impl(x: f32) -> f32 {
        return cbrtf(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.cbrt();
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        return Float::with_val(prec, x).cbrt();
    }
}
