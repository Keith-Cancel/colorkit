use std::f32::consts;

use rand::RngCore;
use rug::Float;

use super::Ansi;
use super::PRIMES;
use super::PowersF32;
use super::RandomF32;
use crate::tests::MathFn;

pub struct Relative {
    values: Vec<f32>,
}

impl Relative {
    /// Precision to use to use for rug
    const PREC: u32 = 128;
    /// Relative error above which we consider the sample less than ideal
    pub const ERR_LIMIT: f64 = 1e-6;
    /// When the number of [`Relative::ERR_LIM`] exceeds this ratio warn.
    pub const ERR_RATIO: f64 = 1e-4;
    /// Any error above this value is bad.
    pub const ERR_BAD: f64 = 1e-5;

    pub fn new() -> Self {
        let mut v = Vec::<f32>::with_capacity(2100);
        v.push(0.0);
        v.push(consts::E);
        v.push(consts::PI);
        v.push(consts::FRAC_1_PI);
        v.push(consts::TAU);
        v.push(consts::SQRT_2);
        v.push(consts::FRAC_1_SQRT_2);
        v.push(consts::LN_2);
        let mut x = 0.0000000001f32;
        for _ in 0..80 {
            v.push(x);
            v.push(x.sqrt());
            x *= 1.5;
        }

        for i in 2..1000u32 {
            let mut f = i as f32;
            for _ in 0..6 {
                v.push(f);
                v.push(f.recip());
                f *= i as f32;
            }
        }

        return Relative {
            values: v
        };
    }

    pub fn run<F: MathFn>(&self) {
        let s = rand::rng().next_u64();
        let i = PowersF32::new()
            .chain(RandomF32::new(25000, s))
            .chain(PRIMES.iter().copied())
            .chain(self.values.iter().copied());
        self.run_case::<F, _>(i);
    }

    fn run_case<F: MathFn, I: Iterator<Item = f32>>(&self, iter: I) {
        let mut imp_to_std = Vec::<f64>::new();
        let mut imp_to_rug = Vec::<f64>::new();
        let mut std_to_rug = Vec::<f64>::new();

        for x in iter {
            if !x.is_finite() {
                continue;
            }
            if !F::ALLOW_ZERO && x == 0.0 {
                continue;
            }
            let x = if !F::ALLOW_NEG && x < 0.0 { x.abs() } else { x };

            let rug = F::rug_impl(Self::PREC, x);
            let std = F::std_f32_impl(x) as f64;
            let fun = F::test_f32_impl(x) as f64;
            let fun_r = Float::with_val(Self::PREC, fun);
            let std_r = Float::with_val(Self::PREC, std);

            if std != 0.0 {
                let err = (std - fun).abs() / std.abs();
                imp_to_std.push(err);
            }
            if !rug.is_zero() {
                let den = rug.clone().abs();
                let err = ((&rug - fun_r).abs() / &den).to_f64();
                imp_to_rug.push(err);

                let err = ((&rug - std_r).abs() / &den).to_f64();
                std_to_rug.push(err);
            }
        }

        imp_to_rug.sort_by(|a, b| a.partial_cmp(b).unwrap());
        imp_to_std.sort_by(|a, b| a.partial_cmp(b).unwrap());
        std_to_rug.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!(
            "\n=========== {}Relative Error{}: {}{}{} | Samples = {}{}{} ===========",
            Ansi::BOLD,
            Ansi::RESET,
            Ansi::CYAN,
            F::NAME,
            Ansi::RESET,
            Ansi::BOLD,
            imp_to_rug.len(),
            Ansi::RESET,
        );
        println!(
            "{:<11} {}{:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}{}",
            "Case",
            Ansi::BLUE,
            "Mean Err",
            "Min Err",
            "Max Err",
            "Std Dev",
            "p50",
            "p90",
            "p99",
            Ansi::RESET
        );

        Self::print_stats("Impl => Std", &imp_to_std);
        Self::print_stats("Impl => Ref", &imp_to_rug);
        Self::print_stats("Std  => Ref", &std_to_rug);

        let mut cnt = 0usize;
        for &x in imp_to_rug.as_slice() {
            if x > Self::ERR_LIMIT {
                cnt += 1;
            }
            if x >= Self::ERR_BAD {
                println!(
                    "{}Error:{} '{}' has a relative error exceeding {:.3e} > {:.3e}",
                    Ansi::RED,
                    Ansi::RESET,
                    F::NAME,
                    x,
                    Self::ERR_BAD
                );
                return;
            }
        }
        let ratio = (cnt as f64) / (imp_to_rug.len() as f64);
        if ratio > Self::ERR_RATIO {
            println!(
                "{}Warning:{} '{}' has {:.4}% > {} relative-error limit",
                Ansi::RED,
                Ansi::RESET,
                F::NAME,
                ratio * 100.0,
                Self::ERR_LIMIT
            );
        }
    }

    fn print_stats(name: &str, data: &[f64]) {
        if data.is_empty() {
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

        let mut sum = 0.0f64;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for &x in data {
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
            sum += x;
        }

        let mean = sum / (data.len() as f64);

        let mut var_sum = 0.0;
        for &x in data {
            let d = x - mean;
            var_sum += d * d;
        }
        let variance = var_sum / (data.len() as f64);
        let std_dev = variance.sqrt();

        let percent = |dat: &[f64], p: f64| -> f64 {
            let idx = ((dat.len() as f64) * p).floor() as usize;
            dat[idx.min(dat.len() - 1)]
        };
        let p50 = percent(data, 0.50);
        let p90 = percent(data, 0.90);
        let p99 = percent(data, 0.99);

        println!(
            "{}{:<11}{} {:>9.3e} {:>9.3e} {:>9.3e} {:>9.3e} {:>9.3e} {:>9.3e} {:>9.3e}",
            Ansi::BOLD,
            name,
            Ansi::RESET,
            mean,
            min,
            max,
            std_dev,
            p50,
            p90,
            p99,
        );
    }
}
