use std::f32::consts;
use std::hint::black_box;
use std::time::Instant;

use crate::tests::MathFn;

pub struct Perf {
    values: Vec<f32>,
}

impl Perf {
    pub fn new() -> Self {
        let l1 = 70;
        let l2 = 10000;
        let cap = 4 + l1 * 2 + l2 * 5;
        // Generate a bunch floats to use for testing.
        let mut v = Vec::<f32>::with_capacity(cap);
        v.push(consts::E);
        v.push(consts::PI);
        v.push(consts::SQRT_2);
        v.push(541.0); // 100th prime

        let mut x = 0.0000000001f32;
        for _ in 0..l1 {
            v.push(x);
            v.push(x.sqrt());
            x *= 1.5;
        }

        let mut x = 0.125f32;
        for i in 0..l2 {
            v.push(x.to_radians().sin());
            v.push(x.sqrt());
            let cb = x.cbrt();
            let cb = if (i & 1) == 0 { cb } else { -cb };
            v.push(cb);

            let p = x.powi(5);
            v.push(p);
            v.push(p.recip());
            x += 0.125;
        }
        assert_eq!(cap, v.len());
        for x in v.as_slice() {
            assert!(x.is_normal());
        }

        return Perf {
            values: v
        };
    }

    pub fn run<F: MathFn>(&self) {
        let arr = self.values.as_slice();

        Self::touch(arr);
        let now = Instant::now();
        for x in arr {
            black_box(*x);
        }
        println!("Baseline: {:.3?}", now.elapsed());

        Self::touch(arr);
        let now = Instant::now();
        for x in arr {
            black_box(F::std_f32_impl(*x));
        }
        println!("Reference: {:.3?}", now.elapsed());

        Self::touch(arr);
        let now = Instant::now();
        for x in arr {
            black_box(F::test_f32_impl(*x));
        }
        println!("Candidate: {:.3?}", now.elapsed());
    }

    #[inline(never)]
    fn run_case<F: MathFn, const RUNS: usize>(&self, name: &str, func: fn(f32) -> f32, arr: &[f32], n: f64) {
        let mut times_ns: [u64; RUNS] = [0; RUNS];

        for i in 0..RUNS {
            Self::touch(arr);

            let start = Instant::now();
            for &x in arr {
                black_box(func(x));
            }
            let elapsed = start.elapsed();
            times_ns[i] = elapsed.as_nanos() as u64;
        }
    }

    #[inline(never)]
    fn touch(arr: &[f32]) {
        for x in arr {
            black_box(*x);
        }
    }
}
