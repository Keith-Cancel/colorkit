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

#[inline(always)]
pub fn fma(x: f64, a: f64, b: f64) -> f64 {
    let mut ret = x;
    unsafe {
        asm!(
            "fmadd {x:d}, {x:d}, {a:d}, {b:d}",
            x = inout(vreg) ret,
            a = in(vreg) a,
            b = in(vreg) b,
            // https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.value-type-constraints
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return ret;
}
