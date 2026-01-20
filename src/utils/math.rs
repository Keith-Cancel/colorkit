/// Computes the quintic root or 5th root.
pub const fn quirt(x: f32) {
    let bits = x.to_bits();
    // TODO
    // for rough first guess
    // * divide f32 exponent by 5
    // * apply some linear/cheap approx to m^(1/5) of the mantissa.
    // * This is because a f32 is basiclly m * 2^k and (m * 2^k)^(1/5)
    //   is m^(1/5) * 2^(k/5)
    // Apply some number of rounds of Newton's or Halley's method
}
