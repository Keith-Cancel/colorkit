use core::arch::asm;

#[inline(always)]
pub fn sqrtf(x: f32) -> f32 {
    let mut ret = x;
    unsafe {
        asm!(
            "fsqrt {r:s}, {r:s}",
            r = inout(vreg) ret,
            // https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.value-type-constraints
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return ret;
}
