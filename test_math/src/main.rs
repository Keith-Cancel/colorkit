pub mod runner;
pub mod tests;

use colorkit::utils::math;

fn main() {
    let p = runner::Perf::new();
    let r = runner::Relative::new();
    p.run::<tests::Quirt>();
    r.run::<tests::Quirt>();
    println!("{}", math::quirt(32.0));
    println!("{}", math::quirt(243.0));
}
