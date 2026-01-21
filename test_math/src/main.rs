mod math_fn;
mod tests;

use colorkit::utils::math;
pub use math_fn::MathFn;

fn main() {
    println!("{}", math::quirt(32.0));
}
