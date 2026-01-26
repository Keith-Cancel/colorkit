pub mod runner;
pub mod tests;

use colorkit::math::MathFuncs;
use tests::*;

fn main() {
    let p = runner::Perf::new();
    let r = runner::Relative::new();
    let u = runner::Ulp::new();

    p.run::<Sqrt>();
    r.run::<Sqrt>();
    u.run::<Sqrt>();

    p.run::<Cbrt>();
    r.run::<Cbrt>();
    u.run::<Cbrt>();

    p.run::<Quirt>();
    r.run::<Quirt>();
    u.run::<Quirt>();

    println!("{}", 32.0.quirt());
    println!("{}", 243.0.quirt());
}
