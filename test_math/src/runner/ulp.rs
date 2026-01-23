use colorkit::utils::math::MathFuncs;

fn ulp_diff(ref_: f64, b: f32) -> f64 {
    if ref_.is_nan() || b.is_nan() {
        return if ref_.is_nan() == b.is_nan() {
            0.0
        } else {
            f64::INFINITY
        };
    }
    let int_ulp = (ref_ as f32).ulp_int_diff(b);

    todo!();
}
