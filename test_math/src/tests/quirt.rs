use colorkit::utils::math::quirt;
use rug::Float;
use rug::ops::Pow;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Quirt;

impl MathFn for Quirt {
    const NAME: &'static str = "5th Root (Quintic Root)";

    fn test_f32_impl(x: f32) -> f32 {
        return quirt(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        return x.powf(0.2);
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        let p = Float::with_val(prec, 5u32).recip();
        let x = Float::with_val(prec, x);
        return x.pow(p);
    }
}
