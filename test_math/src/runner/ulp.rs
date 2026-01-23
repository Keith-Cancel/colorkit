use colorkit::utils::math::MathFuncs;

fn f64_to_f32_down(x: f64) -> f32 {
    let f = x as f32;
    return if (f as f64) > x { f.next_down() } else { f };
}

fn ulp_diff(ref_d: f64, x: f32) -> f64 {
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
    let ref_d = ref_d.abs();
    let ref_f = ref_f.abs();
    let x = x.abs();

    todo!();
}
