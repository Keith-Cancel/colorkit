use core::f64;
use std::f32::consts;

use rug::Float;

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
        //let i = PowersF32::new().chain(PRIMES.iter().copied());
        //self.run_case::<F, _>(F::std_f32_impl, i);

        let i = PowersF32::new().chain(PRIMES.iter().copied());
        self.run_case::<F, _>(F::test_f32_impl, i);
    }

    fn run_case<F: MathFn, I: Iterator<Item = f32>>(&self, func: fn(f32) -> f32, iter: I) {
        let mut std_res = Vec::<f64>::new();
        let mut rug_res = Vec::<f64>::new();
        let mut worst_std = (f64::NEG_INFINITY, f32::NAN);
        let mut worst_rug = (f64::NEG_INFINITY, f32::NAN);

        for x in iter {
            let rug = F::rug_impl(Self::PREC, x);
            let std = F::std_f32_impl(x) as f64;
            let fun = func(x) as f64;
            let fun_r = Float::with_val(Self::PREC, fun);

            if std != 0.0 {
                let err = (std - fun).abs() / std.abs();
                if err > worst_std.0 {
                    worst_std = (err, x);
                }
                std_res.push(err);
            }
            if rug != 0.0 {
                let err = ((&rug - fun_r).abs() / fun.abs()).to_f64();
                if err > worst_rug.0 {
                    worst_rug = (err, x);
                }
                rug_res.push(err);
            }
        }
        println!("worst rug: {} => {}", worst_rug.0, worst_rug.1);
        println!("worst std: {} => {}", worst_std.0, worst_std.1);
    }

    fn print_stats(name: &str, data: &[f64]) {
        
    }
}
