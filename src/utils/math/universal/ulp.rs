/// Calculate the ULP difference between two f32
pub const fn ulp_int_diff_f32(a: f32, b: f32) -> u32 {
    if a.signum() != b.signum() {
        return ulp_int_diff_f32(0.0, a.abs()) + ulp_int_diff_f32(0.0, b.abs()) + 1;
    }
    let a = a.to_bits();
    let b = b.to_bits();
    return if a > b { a - b } else { b - a };
}

#[cfg(test)]
mod test {
    use super::ulp_int_diff_f32 as ulp;

    #[test]
    fn ulp_check() {
        // rust's f32::MIN_POSITIVE is not the true minium
        // the true mininum is a subnormal value.
        let min = f32::from_bits(1);

        assert_eq!(ulp(1.0, 1.0f32.next_up()), 1);
        assert_eq!(ulp(1.0, 1.0f32.next_down()), 1);
        assert_eq!(ulp(1.0, 1.0 + f32::EPSILON), 1);
        assert_eq!(ulp(1.0, 1.0 - f32::EPSILON), 2);
        assert_eq!(ulp(1.0, 1.0 - f32::EPSILON), 2);
        assert_eq!(ulp(min, 0.0), 1);
        assert_eq!(ulp(-min, min), 3);
        assert_eq!(ulp(-1.0, -1.0 - f32::EPSILON), 1);
        assert_eq!(ulp(-1.0, -1.0 + f32::EPSILON), 2);
        assert_eq!(ulp(0.0, 0.0), 0);
        assert_eq!(ulp(-0.0, 0.0), 1);
        assert_eq!(ulp(f32::MAX, f32::INFINITY), 1);
        assert_eq!(ulp(-f32::MAX, f32::NEG_INFINITY), 1);
        assert_eq!(ulp(1.0, 0.5), 8388608);
        assert_eq!(ulp(1.0, 2.0), 8388608);
    }
}
