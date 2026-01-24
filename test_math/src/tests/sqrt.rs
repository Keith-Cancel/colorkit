use colorkit::math::sqrtf_const;
use rug::Float;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Sqrt;

impl MathFn for Sqrt {
    const NAME: &'static str = "Square Root";
    const ALLOW_NEG: bool = false;
    const ALLOW_ZERO: bool = true;
    fn test_f32_impl(x: f32) -> f32 {
        // Test the universal implentation
        // otherwise we might be testing hardware implentations.
        return sqrtf_const(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.sqrt();
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        return Float::with_val(prec, x).sqrt();
    }
}
