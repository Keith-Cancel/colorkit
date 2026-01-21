const F32_BIAS: i32 = 127;
const F32_MSK_EXP: u32 = 0x7f800000;
const F32_MSK_MAN: u32 = 0x007fffff;

#[inline]
const fn exponent(bits: u32) -> i8 {
    let e = ((bits >> 23) & 0xff) as i32 - F32_BIAS;
    return e as i8;
}

/// Mask the floats exponent and divide it by 5.
#[inline]
const fn exponent_div_5(bits: u32) -> u32 {
    let e = bits & F32_MSK_EXP;
    // ((e - 127) / 5) + 127 = (e / 5) + (508 / 5)
    let d = e / 5;
    // Now add the fractional bit.
    // 508 / 5 ~= 0x65.999999 in a fixed point u32 with 24 bit fraction
    // shift right 1 and it's then 0x32cccccc
    let d = if e <= (127u32 << 23) {
        d + 0x32cccccc + 0x666666 + 2
    } else {
        d + 0x32cccccc + 1
    };
    return d >> 23;
}

/// Computes the quintic root or 5th root.
pub fn quirt(x: f32) -> f32 {
    // table 2^(x/5) for f32
    const TWO_OVER_5: [f32; 5] = [
        1.0,
        1.1486983549970350, // 2^(1/5)
        1.3195079107728943, // 2^(2/5)
        1.5157165665103981, // 2^(3/5)
        1.7411011265922483, // 2^(4/5)
    ];

    let bits = x.to_bits();
    let neg = bits & 0x80000000;
    let abs = bits & 0x7fffffff;
    let exp = exponent(bits);

    // Either inifnity or NaN
    if abs >= F32_MSK_EXP {
        return x;
    }

    // For rough first guess
    // * divide f32 exponent by 5
    // * apply some linear/cheap approx to m^(1/5) of the mantissa.
    // * This is because a f32 is basiclly m * 2^k and (m * 2^k)^(1/5)
    //   is m^(1/5) * 2^(k/5)
    let q = exp.div_euclid(5);
    let r = exp.rem_euclid(5);
    let frac = TWO_OVER_5[r as usize]; // Fractional exponent after dividing by 5

    let q = (q as i32 + F32_BIAS) as u32;
    let v = f32::from_bits(neg | (q << 23)) * frac;

    let a = x as f64;
    let mut v = v as f64;
    println!("\nV: {:.32}", v);
    for _ in 0..3 {
        let n = 4.0 * v + (a / (v * v * v * v));
        v = n / 5.0;
    }

    return v as f32;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_exponent() {
        let mut v: f32 = 1.0;
        assert_eq!(exponent(v.to_bits()), 0);

        for i in 1..=128i32 {
            v *= 2.0;
            assert_eq!(exponent(v.to_bits()), i as i8);
        }

        v = 1.0;
        for i in 1..=127 {
            v *= 0.5;
            assert_eq!(exponent(v.to_bits()), (-i));
        }
    }

    #[test]
    fn exp_div_5() {
        let mut v: f32 = 1.53125;
        for _ in 0..128 {
            let bits = v.to_bits();
            let e_i = (exponent(bits) / 5) as i32;
            let e_r1 = (e_i + 127) as u32;
            let e_r2 = exponent_div_5(bits);
            assert_eq!(e_r1, e_r2);
            v *= 2.0;
        }

        v = 1.53125;
        for _ in 0..127 {
            let bits = v.to_bits();
            let e_i = (exponent(bits) / 5) as i32;
            let e_r1 = (e_i + 127) as u32;
            let e_r2 = exponent_div_5(bits);
            assert_eq!(e_r1, e_r2);
            v *= 0.5;
        }
    }
    extern crate std;
    #[test]
    fn hmm() {
        println!("{:.32}", quirt(0.125));
        println!("{:.32}", f32::powf(0.125, 0.2));
        println!("{:.32}", quirt(0.25));
        println!("{:.32}", f32::powf(0.25, 0.2));
        println!("{:.32}", quirt(100.0));
        println!("{:.32}", f32::powf(100.0, 0.2));
        println!("{:.32}", quirt(513479.0));
        println!("{:.32}", f32::powf(513479.0, 0.2));
        println!("{:.32}", quirt(77098997.0));
        println!("{:.32}", f32::powf(77098997.0, 0.2));

        //13.87089633941650390625000000000000
        //13.870896696932448919916278887221704989674558691001695883592078542...
        //13.87089729309082031250000000000000

        //37.79285812377929687500000000000000
        //37.792858865422987497487438270521833999746461437347417976071421339...
        //37.79286193847656250000000000000000
    }
}
