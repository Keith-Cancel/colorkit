/// Computes the quintic root or 5th root.
pub const fn quirt(x: f32) {
    let bits = x.to_bits();
    // TODO
    // for rough first guess
    // * divide f32 exponent by 5
    // * apply some linear/cheap approx to m^(1/5) of the mantissa.
    // Apply some number of rounds of Newton's or Halley's method
}
