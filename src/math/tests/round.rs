use core::f32::consts::PI;

use super::universal;

fn bit_eq(a: f32, b: f32) -> bool {
    return a.to_bits() == b.to_bits();
}

#[test]
fn trunc_frac() {
    frac(universal::truncf, 0.0, -0.0);
    frac(super::truncf, 0.0, -0.0);
}
#[test]
fn floor_frac() {
    frac(universal::floorf, 0.0, -1.0);
    frac(super::floorf, 0.0, -1.0);
}
fn frac<F: Fn(f32) -> f32>(func: F, pos: f32, neg: f32) {
    let fracs = [0.1, 0.9, 0.75, 0.5, 0.25, 2e-32, 2e-45];
    assert!(bit_eq(func(0.0), 0.0));
    assert!(bit_eq(func(-0.0), -0.0));
    for v in fracs {
        assert!(bit_eq(func(v), pos));
        assert!(bit_eq(func(-v), neg));
    }
}

#[test]
fn trunc_mixed() {
    mixed(universal::truncf);
    mixed(super::truncf);
}
#[rustfmt::skip]
fn mixed<F: Fn(f32) -> f32>(func: F) {
    assert!(bit_eq(func( 1.1),   1.0));
    assert!(bit_eq(func(-1.1),  -1.0));
    assert!(bit_eq(func( 1.25),  1.0));
    assert!(bit_eq(func(-1.25), -1.0));
    assert!(bit_eq(func( 1.9),   1.0));
    assert!(bit_eq(func(-1.9),  -1.0));
    assert!(bit_eq(func(PI),     3.0));
    assert!(bit_eq(func(8.125),  8.0));

    assert!(bit_eq(func( 4194303.25), 4194303.0));
    assert!(bit_eq(func(-4194304.5), -4194304.0));
    assert!(bit_eq(func( 8388607.5), 8388607.0));
}

#[test]
fn trunc_large() {
    large(universal::truncf);
    large(super::truncf);
}
#[test]
fn floor_large() {
    large(universal::floorf);
    large(super::floorf);
}
fn large<F: Fn(f32) -> f32>(func: F) {
    assert!(bit_eq(func(8388608.0), 8388608.0));
    assert!(bit_eq(func(-8388608.0), -8388608.0));
    assert!(bit_eq(func(16777216.0), 16777216.0));
    assert!(bit_eq(func(-16777216.0), -16777216.0));
    assert!(bit_eq(func(25165824.0), 25165824.0));
    assert!(bit_eq(func(-25165824.0), -25165824.0));
    assert!(bit_eq(func(8589934592.0), 8589934592.0));
    assert!(bit_eq(func(f32::INFINITY), f32::INFINITY));
    assert!(bit_eq(func(f32::NEG_INFINITY), f32::NEG_INFINITY));
}
