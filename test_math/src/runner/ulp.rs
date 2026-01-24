use std::f32::consts;

use colorkit::math::MathFuncs;
use rand::RngCore;

use super::Ansi;
use super::PRIMES;
use super::PowersF32;
use super::RandomF32;
use crate::tests::MathFn;

pub struct Ulp {
    values: Vec<f32>,
}

struct UlpStats {
    sum:     f64,
    max:     f64,
    mval:    f32,
    cnt:     u64,
    // buckets: <=.25, <=0.5, <=1, <=2, <=4, > 4
    buckets: [u64; 6],
}

impl UlpStats {
    pub fn new() -> Self {
        return Self {
            sum:     0.0,
            max:     f64::NEG_INFINITY,
            mval:    f32::NAN,
            cnt:     0,
            buckets: [0; 6],
        };
    }

    pub fn count(&self) -> u64 {
        return self.cnt;
    }

    pub fn store(&mut self, ulp: f64, val: f32) {
        self.sum += ulp;
        self.cnt += 1;
        if ulp > self.max {
            self.max = ulp;
            self.mval = val;
        }
        match ulp {
            0.0..=0.25 => self.buckets[0] += 1,
            0.25..=0.5 => self.buckets[1] += 1,
            0.5..=1.0 => self.buckets[2] += 1,
            1.0..=2.0 => self.buckets[3] += 1,
            2.0..=4.0 => self.buckets[4] += 1,
            _ => self.buckets[5] += 1,
        }
    }

    pub fn print_stats(&self, name: &str) {
        if self.cnt < 1 {
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

        let mean = self.sum / (self.cnt as f64);

        println!(
            "{}{:<11}{} {:>8.3} {:>8.3}  0x{:08x} ({:.5e})",
            Ansi::BOLD,
            name,
            Ansi::RESET,
            mean,
            self.max,
            self.mval.to_bits(),
            self.mval
        );
    }

    pub fn print_buckets(&self, name: &str) {
        println!(
            "{}{} ULP Sums:{} {:5} <= .25, {:5} <= .5, {:5} <= 1, {:5} <= 2, {:5} <= 4, {:5} > 4",
            Ansi::DIM,
            name,
            Ansi::RESET,
            self.buckets[0],
            self.buckets[1],
            self.buckets[2],
            self.buckets[3],
            self.buckets[4],
            self.buckets[5]
        );
    }
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
        let s = rand::rng().next_u64();
        let i = PowersF32::new()
            .chain(RandomF32::new(150000, s))
            .chain(self.values.iter().copied())
            .chain(PRIMES.iter().copied());
        self.run_case::<F, _>(i);
    }

    fn run_case<F: MathFn, I: Iterator<Item = f32>>(&self, iter: I) {
        let mut std_st = UlpStats::new();
        let mut imp_st = UlpStats::new();

        for x in iter {
            if !x.is_finite() {
                continue;
            }
            if !F::ALLOW_ZERO && x == 0.0 {
                continue;
            }
            let x = if !F::ALLOW_NEG && x < 0.0 { x.abs() } else { x };

            let rug = F::rug_impl(Self::PREC, x).to_f64();

            // It's not resentable as a f32 so does not
            // make sense to compute a ULP.
            if (rug as f32).is_infinite() {
                continue;
            }

            let std = F::std_f32_impl(x);
            let fun = F::test_f32_impl(x);

            let imp_ulp = ulp_diff(rug, fun);
            let std_ulp = ulp_diff(rug, std);

            std_st.store(std_ulp, x);
            imp_st.store(imp_ulp, x);
        }

        println!(
            "\n===== {}ULP Difference{}: {}{}{} | Samples = {}{}{} =====",
            Ansi::BOLD,
            Ansi::RESET,
            Ansi::CYAN,
            F::NAME,
            Ansi::RESET,
            Ansi::BOLD,
            imp_st.count(),
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
        imp_st.print_stats("Impl ULP");
        std_st.print_stats("Std  ULP");
        imp_st.print_buckets("Impl");
        std_st.print_buckets("Std ");
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
