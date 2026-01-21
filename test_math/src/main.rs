mod math_fn;

use colorkit::utils::math;
pub use math_fn::MathFn;

fn main() {
    println!("{}", math::quirt(32.0));
}
