use std::f32::consts;

use colorkit::utils::math::MathFuncs;

use super::Ansi;
use crate::tests::MathFn;

pub struct Ulp {
    values: Vec<f32>,
}

impl Ulp {
    /// Precision to use to use for rug
    const PREC: u32 = 128;

    pub fn new() -> Self {
        let mut v = Vec::<f32>::with_capacity(128);
        v.push(0.0);
        v.push(consts::E);
        v.push(consts::PI);
        v.push(consts::FRAC_1_PI);
        v.push(consts::TAU);
        v.push(consts::SQRT_2);
        v.push(consts::FRAC_1_SQRT_2);
        v.push(consts::LN_2);
        let mut x = 0.0000000001f32;
        for _ in 0..120 {
            v.push(x);
            v.push(x.sqrt());
            x *= 1.5;
        }
        return Ulp {
            values: v
        };
    }

    pub fn run<F: MathFn>(&self) {
        self.run_case::<F, _>(self.values.iter().copied());
    }

    fn run_case<F: MathFn, I: Iterator<Item = f32>>(&self, iter: I) {
        let mut std_sum = 0.0f64;
        let mut imp_sum = 0.0f64;

        let mut std_max = f64::NEG_INFINITY;
        let mut imp_max = f64::NEG_INFINITY;

        let mut std_mval = f32::NAN;
        let mut imp_mval = f32::NAN;

        let mut cnt = 0u64;
        for x in iter {
            if !x.is_finite() {
                continue;
            }
            if !F::ALLOW_ZERO && x == 0.0 {
                continue;
            }
            let x = if !F::ALLOW_NEG && x < 0.0 { x.abs() } else { x };

            let rug = F::rug_impl(Self::PREC, x).to_f64();
            let std = F::std_f32_impl(x);
            let fun = F::test_f32_impl(x);

            let imp_ulp = ulp_diff(rug, fun);
            let std_ulp = ulp_diff(rug, std);

            std_sum += std_ulp;
            if std_ulp > std_max {
                std_max = std_ulp;
                std_mval = x;
            }

            imp_sum += imp_ulp;
            if imp_ulp > imp_max {
                imp_max = imp_ulp;
                imp_mval = x;
            }

            cnt += 1;
        }

        println!(
            "\n===== {}ULP Difference{}: {}{}{} | Samples = {}{}{} =====",
            Ansi::BOLD,
            Ansi::RESET,
            Ansi::CYAN,
            F::NAME,
            Ansi::RESET,
            Ansi::BOLD,
            cnt,
            Ansi::RESET,
        );
        println!(
            "{:<11} {}{:>8} {:>8}  {:>10} {}",
            "Case",
            Ansi::BLUE,
            "Mean ULP",
            "Max ULP",
            "Max Value",
            Ansi::RESET
        );
        Self::print_stats("Impl ULP", imp_sum, cnt, imp_max, imp_mval);
        Self::print_stats("Std ULP", std_sum, cnt, std_max, std_mval);
    }

    fn print_stats(name: &str, sum: f64, count: u64, ulp_max: f64, max_val: f32) {
        if count < 1 {
            println!(
                "{}{:<12}{} {}(no samples){}",
                Ansi::BOLD,
                name,
                Ansi::RESET,
                Ansi::YELLOW,
                Ansi::RESET
            );
            return;
        }

        let mean = sum / (count as f64);

        println!(
            "{}{:<11}{} {:>8.3} {:>8.3}  {:#08x} ({:.5e})",
            Ansi::BOLD,
            name,
            Ansi::RESET,
            mean,
            ulp_max,
            max_val.to_bits(),
            max_val
        );
    }
}

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
