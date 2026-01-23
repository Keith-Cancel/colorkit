pub mod runner;
pub mod tests;

use colorkit::utils::math::MathFuncs;

fn main() {
    let p = runner::Perf::new();
    let r = runner::Relative::new();
    p.run::<tests::Quirt>();
    r.run::<tests::Quirt>();
    println!("{}", 32.0.quirt());
    println!("{}", 243.0.quirt());
}
