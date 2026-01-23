use colorkit::utils::math::quirt_f32;
use rug::Float;
use rug::ops::Pow;

use super::MathFn;

#[derive(Debug, Clone, Copy)]
pub struct Quirt;

impl MathFn for Quirt {
    const NAME: &'static str = "5th Root (Quintic Root)";
    const ALLOW_NEG: bool = true;
    const ALLOW_ZERO: bool = true;
    fn test_f32_impl(x: f32) -> f32 {
        return quirt_f32(x);
    }

    fn std_f32_impl(x: f32) -> f32 {
        let neg = x < 0.0;
        let x = if neg { -x } else { x };
        let x = x.powf(0.2);
        return if neg { -x } else { x };
    }

    fn rug_impl(prec: u32, x: f32) -> Float {
        let neg = x < 0.0;
        let p = Float::with_val(prec, 5u32).recip();
        let x = Float::with_val(prec, x).abs().pow(p);
        return if neg { -x } else { x };
    }
}
