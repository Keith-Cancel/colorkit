use colorkit::math::atanf;
use rug::Float;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Atan;

impl MathFn for Atan {
    const NAME: &'static str = "arctan(x)";
    const ALLOW_NEG: bool = true;
    const ALLOW_ZERO: bool = true;

    fn test_f32_impl(x: f32) -> f32 {
        return atanf(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.atan();
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        return Float::with_val(prec, x).atan();
    }
}
