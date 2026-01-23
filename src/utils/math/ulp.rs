/// Calculate the ULP difference between two f32
pub const fn ulp_int_diff_f32(a: f32, b: f32) -> u32 {
    if a.signum() != b.signum() {
        return ulp_int_diff_f32(0.0, a.abs()) + ulp_int_diff_f32(0.0, b.abs()) + 1;
    }
    let a = a.to_bits();
    let b = b.to_bits();
    return if a > b { a - b } else { b - a };
}
