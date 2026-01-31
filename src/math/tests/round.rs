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
fn ceil_frac() {
    frac(universal::ceilf, 1.0, -0.0);
    frac(super::ceilf, 1.0, -0.0);
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
    mixed(universal::truncf, 0.0, 0.0);
    mixed(super::truncf, 0.0, 0.0);
}
#[test]
fn ceil_mixed() {
    mixed(universal::ceilf, 1.0, 0.0);
    mixed(super::ceilf, 1.0, 0.0);
}
#[test]
fn floor_mixed() {
    mixed(universal::floorf, 0.0, -1.0);
    mixed(super::floorf, 0.0, -1.0);
}
fn mixed<F: Fn(f32) -> f32>(func: F, pos: f32, neg: f32) {
    let val = [
        1.1, 1.25, 1.9, PI, 8.125, 4194303.25, 4194304.5, 8388607.5,
    ];
    let exp = [
        1.0, 1.00, 1.0, 3.0, 8.000, 4194303.00, 4194304.0, 8388607.0,
    ];
    for (&v, e) in val.iter().zip(exp) {
        assert!(bit_eq(func(v), e + pos));
        assert!(bit_eq(func(-v), (-e) + neg));
    }
}

#[test]
fn trunc_large() {
    large(universal::truncf);
    large(super::truncf);
}
#[test]
fn ceil_large() {
    large(universal::ceilf);
    large(super::ceilf);
}
#[test]
fn round_large() {
    large(universal::roundf);
    large(super::roundf);
}
#[test]
fn round_even_large() {
    large(universal::roundevenf);
    large(super::roundevenf);
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

#[test]
fn round_frac() {
    round_frac_(universal::roundf);
    round_frac_(super::roundf);
}
fn round_frac_<F: Fn(f32) -> f32>(func: F) {
    const HALF_PLUS: f32 = f32::from_bits(0x3f000000 + 1);
    const HALF_MINUS: f32 = f32::from_bits(0x3f000000 - 1);
    const ONE_MINUS: f32 = f32::from_bits(0x3f800000 - 1);
    let val = [
        0.0,
        0.125,
        0.25,
        1.0 / 3.0,
        0.4,
        HALF_MINUS,
        0.5,
        HALF_PLUS,
        0.53125,
        2.0 / 3.0,
        0.6,
        0.75,
        ONE_MINUS,
    ];
    let exp = [
        0.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    for (i, (&v, e)) in val.iter().zip(exp).enumerate() {
        assert!(bit_eq(func(v), e), "Possitive failed at: {}", i);
        assert!(bit_eq(func(-v), -e), "Negative failed at: {}", i);
    }
}

#[test]
fn round_even_frac() {
    round_even_frac_(universal::roundevenf);
    round_even_frac_(super::roundevenf);
}
fn round_even_frac_<F: Fn(f32) -> f32>(func: F) {
    const HALF_PLUS: f32 = f32::from_bits(0x3f000000 + 1);
    const HALF_MINUS: f32 = f32::from_bits(0x3f000000 - 1);
    const ONE_MINUS: f32 = f32::from_bits(0x3f800000 - 1);
    let val = [
        0.0,
        0.125,
        0.25,
        1.0 / 3.0,
        0.4,
        HALF_MINUS,
        0.5,
        HALF_PLUS,
        0.53125,
        2.0 / 3.0,
        0.6,
        0.75,
        ONE_MINUS,
    ];
    let exp = [
        0.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ];
    for (i, (&v, e)) in val.iter().zip(exp).enumerate() {
        assert!(bit_eq(func(v), e), "Possitive failed at: {}", i);
        assert!(bit_eq(func(-v), -e), "Negative failed at: {}", i);
    }
}

#[test]
fn round_even_mixed() {
    round_even_mixed_(universal::roundevenf);
    round_even_mixed_(super::roundevenf);
}
fn round_even_mixed_<F: Fn(f32) -> f32>(func: F) {
    let val = [
        1.0, 1.1, 1.25, 1.5, 1.9, PI, 8.125, 8.5, 4194303.25, 4194303.5, 4194304.5, 8388607.5,
    ];
    let exp = [
        1.0, 1.0, 1.00, 2.0, 2.0, 3.0, 8.000, 8.0, 4194303.00, 4194304.0, 4194304.0, 8388608.0,
    ];
    for (&v, e) in val.iter().zip(exp) {
        assert!(bit_eq(func(v), e));
        assert!(bit_eq(func(-v), -e));
    }
}
