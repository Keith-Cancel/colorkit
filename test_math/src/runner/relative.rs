use core::f64;
use std::f32::consts;

use rug::Float;

use super::Ansi;
use super::PRIMES;
use super::PowersF32;
use crate::tests::MathFn;

pub struct Relative {
    values: Vec<f32>,
}

impl Relative {
    const PREC: u32 = 128;

    pub fn new() -> Self {
        let mut v = Vec::<f32>::with_capacity(100);
        v.push(consts::E);
        v.push(consts::PI);
        v.push(consts::FRAC_1_PI);
        v.push(consts::TAU);
        v.push(consts::SQRT_2);
        v.push(consts::FRAC_1_SQRT_2);
        v.push(consts::LN_2);
        return Relative {
            values: v
        };
    }

    pub fn run<F: MathFn>(&self) {
        let i = PowersF32::new().chain(PRIMES.iter().copied());
        self.run_case::<F, _>(i);
    }

    fn run_case<F: MathFn, I: Iterator<Item = f32>>(&self, iter: I) {
        let mut imp_to_std = Vec::<f64>::new();
        let mut imp_to_rug = Vec::<f64>::new();
        let mut std_to_rug = Vec::<f64>::new();

        for x in iter {
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

        Self::print_stats("Impl => Std", &imp_to_std);
        Self::print_stats("Impl => Ref", &imp_to_rug);
        Self::print_stats("Std  => Ref", &std_to_rug);
    }

    fn print_stats(name: &str, data: &[f64]) {
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

        println!(
            "{}{:<15}{} {:>12.4e} {:>12.4e} {:>12.4e} {:>12.4e}",
            Ansi::BOLD,
            name,
            Ansi::RESET,
            mean,
            min,
            max,
            std_dev,
        );
    }
}
