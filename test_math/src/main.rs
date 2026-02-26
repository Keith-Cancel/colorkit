pub mod runner;
pub mod tests;

use colorkit::math::MathFuncs;
use runner::*;
use tests::*;

fn run<F: MathFn>(p: &Perf, r: &Relative, u: &Ulp) {
    p.run::<F>();
    r.run::<F>();
    u.run::<F>();
}

fn main() {
    let p = Perf::new();
    let r = Relative::new();
    let u = Ulp::new();

    run::<Sqrt>(&p, &r, &u);
    run::<Cbrt>(&p, &r, &u);
    run::<Quirt>(&p, &r, &u);
    run::<Atan>(&p, &r, &u);
    run::<Cos>(&p, &r, &u);
    run::<Sin>(&p, &r, &u);

    println!("{}", 32.0.quirt());
    println!("{}", 243.0.quirt());
}
