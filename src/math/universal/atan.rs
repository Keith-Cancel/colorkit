use core::f32::consts::FRAC_PI_2;
use core::f32::consts::FRAC_PI_4;
use core::f32::consts::PI;
use core::f64;

use colorkit::math::fma_inner;

fn atan_poly(x: f64) -> f64 {
    let x_2 = x * x;

    let p = fma_inner(x_2, 15159.0, 147455.0);
    let p = fma_inner(x_2, p, 345345.0);
    let p = fma_inner(x_2, p, 225225.0);

    let q = fma_inner(x_2, 1225.0, 44100.0);
    let q = fma_inner(x_2, q, 242550.0);
    let q = fma_inner(x_2, q, 420420.0);
    let q = fma_inner(x_2, q, 225225.0);

    return p / q;
}

pub fn atanf(x: f32) -> f32 {
    let x_pos = x.abs();
    // When x is small enough it's just x.
    // Also preserves the sign of zero.
    if x_pos < 0.00035211173 {
        return x;
    }
    // As x gets larger it eventually
    // just become PI/2
    if x_pos >= 62919776.0 {
        return FRAC_PI_2.copysign(x);
    }

    let mut x_1 = x as f64;
    let mut f = 0.0;

    if x_pos > 6.0 {
        let x_2 = x_1 * x_1;
        let p = fma_inner(x_2, 1155.0, 1190.0);
        let p = fma_inner(x_2, p, 231.0);

        let q = fma_inner(x_2, 1155.0, 1575.0);
        let q = fma_inner(x_2, q, 525.0);
        let q = fma_inner(x_2, q, 25.0);

        let r = x_1 * (p / q);

        return (f64::consts::FRAC_PI_2.copysign(x_1) - r) as f32;
    }

    if x_pos >= 1.05 {
        let c = (1.0 / 0.48f64).copysign(x_1);
        f += 1.1232763516377267f64.copysign(x_1); // ~= arctan(c);
        x_1 = (x_1 - c) / fma_inner(x_1, c, 1.0);
    } else if x_pos >= 0.30209234 {
        let c = (1.0 / 1.67f64).copysign(x_1);
        f += 0.5395384432298387f64.copysign(x_1); // ~= arctan(c);
        x_1 = (x_1 - c) / fma_inner(x_1, c, 1.0);
    }

    let r = fma_inner(x_1, atan_poly(x_1), f);
    return r as f32;
}

pub fn atan2f(y: f32, x: f32) -> f32 {
    if x.is_nan() || y.is_nan() {
        return x + y; // {any} + NaN = NaN
    }

    let x_i = x.to_bits();
    let y_i = y.to_bits();
    let x_a = x_i & 0x7fff_ffff; // abs(x)
    let y_a = y_i & 0x7fff_ffff; // abs(y)

    if x == 0.0 {
        let r = if x_i == 0x8000_0000 { PI } else { 0.0 };
        return r.copysign(y);
    }
    // x is +-infinity and y is +-infinity
    if x_a == 0x7f80_0000 && y_a == 0x7f80_0000 {
        let r = if x_i == 0xff80_0000 {
            3.0 * FRAC_PI_4
        } else {
            FRAC_PI_4
        };
        return r.copysign(y);
    }

    let mut t = atanf(y / x);
    if x < 0.0 {
        t += if y < 0.0 { -PI } else { PI };
    }
    return t;
}

#[cfg(test)]
mod test {
    use core::f32::consts::*;

    use super::*;
    #[test]
    fn atan2_f32() {
        let inf = f32::INFINITY;
        let ninf = f32::NEG_INFINITY;
        // Make sure all 4 zero cases match C's atanf
        assert_eq!(atan2f(0.0, 0.0).to_bits(), 0);
        assert_eq!(atan2f(0.0, -0.0), PI);
        assert_eq!(atan2f(-0.0, 0.0).to_bits(), 0x8000_0000);
        assert_eq!(atan2f(-0.0, -0.0), -PI);

        assert_eq!(atan2f(inf, f32::MAX), FRAC_PI_2);
        assert_eq!(atan2f(inf, -f32::MAX), FRAC_PI_2);
        assert_eq!(atan2f(ninf, f32::MAX), -FRAC_PI_2);
        assert_eq!(atan2f(ninf, -f32::MAX), -FRAC_PI_2);

        assert_eq!(atan2f(inf, inf), FRAC_PI_4);
        assert_eq!(atan2f(inf, ninf), 3.0 * FRAC_PI_4);
        assert_eq!(atan2f(ninf, inf), -FRAC_PI_4);
        assert_eq!(atan2f(ninf, ninf), 3.0 * -FRAC_PI_4);
    }
}

/*
pub const fn atanf2(x: f32) -> f32 {
    let recip = x > 1.0 || x < -1.0;
    let x = x as f64;
    let pi = FRAC_PI_2.copysign(x);
    let x = if recip { 1.0 / x } else { x };

    let x_2 = x * x;
    let x_4 = x_2 * x_2;
    let x_6 = x_4 * x_2;

    /*
    atan(x) ~= x * (P(x^2) / Q(x^2)) for [0.0..=1.0]
    p0 = 19.818_45704_21239;
    p1 = 22.376_09645_14904;
    p2 = 5.6710_79451_63760;
    p3 = 0.17630_40124_4227;

    q0 = 19.818_45705_95466;
    q1 = 28.982_24639_72206;
    q2 = 11.368_19042_96686;
    q3 = 1.0; */
    let p = 0.17630_40124_4227 * x_6
        + 5.6710_79451_63760 * x_4
        + 22.376_09645_14904 * x_2
        + 19.818_45704_21239;
    let q = x_6 + 11.368_19042_96686 * x_4 + 28.982_24639_72206 * x_2 + 19.818_45705_95466;
    let r = x * (p / q);
    let r = if recip { pi - r } else { r };
    return r as f32;
}*/
