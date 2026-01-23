use colorkit::utils::math::MathFuncs;
use rug::Float;

fn f64_to_f32_down(x: f64) -> f32 {
    let f = x as f32;
    return if (f as f64) > x { f.next_down() } else { f };
}

fn ulp_diff(ref_d: f64, x: f32) -> f64 {
    // Handle NaNs if the one is a NaN and the
    // other is not treat the difference as infinite.
    if ref_d.is_nan() || x.is_nan() {
        return if ref_d.is_nan() == x.is_nan() {
            0.0
        } else {
            f64::INFINITY
        };
    }
    let ref_f = ref_d as f32;

    // Integer part of the ulp.
    let ulp_i = ref_f.ulp_int_diff(x);

    // Simpler if these are all possitive.
    // Also ulp_int_diff will account for any
    // difference in sign we just need the
    // the fractional part.
    let ref_d = ref_d.abs();
    let ref_f = ref_f.abs();
    let x = x.abs();

    // Find the step size of the where the reference
    // stradles over the actual value.
    //
    // Rust uses Round to nearest, ties away from zero.
    // https://doc.rust-lang.org/reference/expressions/operator-expr.html#r-expr.as.numeric.float-narrowing
    //
    // Depending on the value it could be up or down
    // We need to ensure we round always in one direction
    // to get the straddle point. Otherwise we would need
    // way to know if we rounded up or down. So then we
    // could which know to call `next_up` or `next_down`.
    // It's just much simpler to force the direction down
    // or up.
    let ref_dwn = f64_to_f32_down(ref_d);
    let ulp_sz = (ref_dwn.next_up() - ref_dwn) as f64;
    // Shouldn't happen?
    // I guess maybe some kinda of hardware flush of a subnormal
    if ulp_sz == 0.0 {
        // Could also just pannic?
        return ulp_i as f64;
    }

    // The fractional part of the ulp
    let mut frac = (ref_d - (ref_f as f64)) / ulp_sz;
    // should the fraction add or subtract
    if x as f64 > ref_d {
        frac = -frac;
    }
    // Add the fractional part to Integer part of the ulp
    return ulp_i as f64 + frac;
}

#[cfg(test)]
mod test {
    use super::ulp_diff;

    #[test]
    fn uld_ref_diff() {
        let ep_d = f32::EPSILON as f64;
        let ep_f = f32::EPSILON;

        assert_eq!(ulp_diff(1.0 + (ep_d / 2.0), 1.0), 0.5);
        assert_eq!(ulp_diff(1.0 + (ep_d / 4.0), 1.0), 0.25);
        assert_eq!(ulp_diff(1.0 - (ep_d / 2.0), 1.0), 1.0);
        assert_eq!(ulp_diff(1.0 - (ep_d / 4.0), 1.0), 0.5);

        assert_eq!(ulp_diff(1.0 + (ep_d / 2.0), 1.0 + ep_f * 10.0), 9.5);
        assert_eq!(ulp_diff(1.0 + (ep_d / 4.0), 1.0 + ep_f * 10.0), 9.75);
        assert_eq!(ulp_diff(1.0 - (ep_d / 2.0), 1.0 - ep_f * 10.0), 19.0);
        assert_eq!(ulp_diff(1.0 - (ep_d / 4.0), 1.0 - ep_f * 10.0), 19.5);

        assert_eq!(ulp_diff(0.0 + (f32::from_bits(1) as f64) / 100.0, 0.0), 0.01);
    }
}
