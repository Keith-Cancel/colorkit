use colorkit::math::fma_inner;

/// approximates tan(x) via P/Q for x in range 0.0..0.86345325
#[inline(always)]
fn tan_rational_poly(x_1: f64) -> (f64, f64) {
    const C0: f64 = 0.5033867350352298; // ~= tan(0.466353325)
    let shift = x_1 >= 0.466353325;

    // Take advantage of a tan identity to double the range.
    let x_1 = if shift { x_1 - 0.466353325 } else { x_1 };
    let x_2 = x_1 * x_1;

    // 10395x - 1260x^3 + 21x^5
    let p = fma_inner(x_2, 21.0, -1260.0);
    let p = fma_inner(x_2, p, 10395.0);
    let p = x_1 * p;

    // 10395 - 4725x^2 + 210x^4 - x^6
    let q = 210.0 - x_2;
    let q = fma_inner(x_2, q, -4725.0);
    let q = fma_inner(x_2, q, 10395.0);

    // In this case x_1 = x - C0
    // use the identity: tan(a + b) = (tan(a) + tan(b))/(1 - tan(a)*tan(b))
    // basically add the C0 back to x_1
    if shift {
        return (fma_inner(q, C0, p), fma_inner(p, -C0, q));
    };
    return (p, q);
}
