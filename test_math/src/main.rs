pub mod runner;
pub mod tests;

use colorkit::utils::math;

fn main() {
    let p = runner::Perf::new();
    p.run::<tests::Quirt>();
    println!("{}", math::quirt(32.0));
}
