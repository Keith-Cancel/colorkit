use core::f32::consts::PI as PI_32;
use core::f64::consts::PI as PI_64;

use colorkit::math::fma_inner;

/// approximates tan(x) via P/Q
fn tan_rational_poly(x_1: f64) -> (f64, f64) {
    let x_2 = x_1 * x_1;
    // 10395x - 1260x^3 + 21x^5
    let p = fma_inner(x_2, 21.0, -1260.0);
    let p = fma_inner(x_2, p, 10395.0);
    let p = x_1 * p;

    // 10395 - 4725x^2 + 210x^4 - x^6
    let q = 210.0 - x_2;
    let q = fma_inner(x_2, q, -4725.0);
    let q = fma_inner(x_2, q, 10395.0);
    return (p, q);
}

/// Evaluate `cos(x)` for x in [-pi, pi] (radians)
pub fn cosf_on_pi(x: f32) -> f32 {
    debug_assert!(x >= -PI_32 && x <= PI_32);
    const C1: f64 = 0.4636476090008061; // tan(C1) ~= 0.5
    const C2: f64 = 0.9272952180016122;
    let x = x.abs();
    let flip = x > 1.72;
    let x = x as f64;
    let x = if flip { PI_64 - x } else { x };

    // In this case x_1 = x - C1
    // use the identity: tan(a + b) = (tan(a) + tan(b))/(1 - tan(a)*tan(b))
    // basically add the C1 back to x_1, but this lets me double the range.
    let (p, q) = if x >= C2 {
        let (p, q) = tan_rational_poly(fma_inner(x, 0.5, -C1));
        (fma_inner(q, 0.5, p), fma_inner(p, -0.5, q))
    } else {
        tan_rational_poly(x * 0.5)
    };

    let p = p * p;
    let q = fma_inner(q, q, p);
    let r = fma_inner(p / q, -2.0, 1.0);

    let r = if flip { -r } else { r };
    return r as f32;
}

/// Evaluate `sin(x)` for x in [-pi, pi] (radians)
pub fn sinf_on_pi(x: f32) -> f32 {
    todo!();
}
