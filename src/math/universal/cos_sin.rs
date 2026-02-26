use core::f32::consts::PI as PI_32;
use core::f64::consts::PI as PI_64;

use colorkit::math::fma_inner;

/// approximates tan(x) via P/Q
#[inline(always)]
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

    // Use identity:
    // cos(x) = (1 - tan(x)^2)/(1 + tan(x)^2)
    // it's re-worked algrabraicly since tan(x) ~= p/q
    let p2 = p * p;
    let q2 = q * q;
    let r = (q2 - p2) / (q2 + p2);

    let r = if flip { -r } else { r }; // Flip sign if needed.
    return r as f32;
}

/// approximates tan(x) via P/Q
#[inline(always)]
fn tan_rational_poly2(x_1: f64) -> (f64, f64) {
    let x_2 = x_1 * x_1;
    // 135135x - 17325x^3 + 378x^5 - x^7
    let p = 378.0 - x_2;
    let p = fma_inner(x_2, p, -17325.0);
    let p = fma_inner(x_2, p, 135135.0);
    let p = x_1 * p;

    // 135135 - 62370x^2 + 3150x^4 - 28x^6
    let q = fma_inner(x_2, -28.0, 3150.0);
    let q = fma_inner(x_2, q, -62370.0);
    let q = fma_inner(x_2, q, 135135.0);

    return (p, q);
}

/// Evaluate `sin(x)` for x in [-pi, pi] (radians)
pub fn sinf_on_pi(x: f32) -> f32 {
    debug_assert!(x >= -PI_32 && x <= PI_32);
    let flip = x.abs() > 1.72;
    let x = x as f64;
    let x = if flip { x - PI_64.copysign(x) } else { x };

    let (p, q) = tan_rational_poly2(x * 0.5);
    // sin(x) = cos(x - pi/2)
    // So after feeding x/2 into tan we need to shift
    // over pi/4. Nicely tan(pi/4) = 1.0
    // So we can use the identity:
    // tan(a +- b) = (tan(a) +- tan(b))/(1.0 -+ tan(a)*tan(b))
    // making b = pi/4 we can can shift the poly over by pi/4
    // With a little algebra we get tan(x/2 - pi/4) ~= (p - q)/(q + p)
    // if then take the identity:
    // sin(x) = (1 - tan(x/2 - pi/4)^2)/(1 + tan(x/2 - pi/4)^2)
    // we can do a little more algerbra to get: (2*p*q)/(p^2 + q^2)
    let p1 = 2.0 * p * q;
    let q1 = fma_inner(q, q, p * p);
    let r = p1 / q1;
    let r = if flip { -r } else { r }; // Flip sign if needed.
    return r as f32;
}
