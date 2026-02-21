use core::f64::consts::FRAC_PI_2;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
const fn mul_add(x: f64, a: f64, b: f64) -> f64 {
    return x.mul_add(a, b);
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
const fn mul_add(x: f64, a: f64, b: f64) -> f64 {
    return x * a + c;
}

const fn atan_poly(x: f64) -> f64 {
    let x_2 = x * x;

    let p = mul_add(x_2, 15159.0, 147455.0);
    let p = mul_add(x_2, p, 345345.0);
    let p = mul_add(x_2, p, 225225.0);

    let q = mul_add(x_2, 1225.0, 44100.0);
    let q = mul_add(x_2, q, 242550.0);
    let q = mul_add(x_2, q, 420420.0);
    let q = mul_add(x_2, q, 225225.0);

    return p / q;
}

pub const fn atanf(x: f32) -> f32 {
    let mut x_1 = x as f64;
    let mut f = 0.0;
    let x = x.abs();

    if x > 6.0 {
        let x_2 = x_1 * x_1;
        let p = mul_add(x_2, 1155.0, 1190.0);
        let p = mul_add(x_2, p, 231.0);

        let q = mul_add(x_2, 1155.0, 1575.0);
        let q = mul_add(x_2, q, 525.0);
        let q = mul_add(x_2, q, 25.0);

        let r = x_1 * (p / q);

        return (FRAC_PI_2.copysign(x_1) - r) as f32;
    }

    if x >= 1.05 {
        let c = (1.0 / 0.48f64).copysign(x_1);
        f += 1.1232763516377267f64.copysign(x_1); // ~= arctan(c);
        x_1 = (x_1 - c) / mul_add(x_1, c, 1.0);
    } else if x >= 0.30209234 {
        let c = (1.0 / 1.67f64).copysign(x_1);
        f += 0.5395384432298387f64.copysign(x_1); // ~= arctan(c);
        x_1 = (x_1 - c) / mul_add(x_1, c, 1.0);
    }

    let r = mul_add(x_1, atan_poly(x_1), f);
    return r as f32;
}

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
}
