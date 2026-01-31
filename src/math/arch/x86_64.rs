use core::arch::asm;

#[inline(always)]
pub fn sqrtf(x: f32) -> f32 {
    let mut ret = x;
    unsafe {
        asm!(
            "sqrtss {r}, {r}",
            r = inout(xmm_reg) ret,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return ret;
}

#[inline(always)]
pub fn truncf(x: f32) -> f32 {
    let mut ret = x;
    unsafe {
        asm!(
            "roundss {r}, {r}, 0x3",
            r = inout(xmm_reg) ret,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return ret;
}
