use core::arch::asm;

#[inline(always)]
pub fn sqrtf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "sqrtss {r}, {r}",
            r = inout(xmm_reg) x,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return x;
}

#[inline(always)]
pub fn truncf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "roundss {r}, {r}, 0xb",
            r = inout(xmm_reg) x,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return x;
}

#[inline(always)]
pub fn floorf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "roundss {r}, {r}, 0x9",
            r = inout(xmm_reg) x,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return x;
}

#[inline(always)]
pub fn ceilf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "roundss {r}, {r}, 0xa",
            r = inout(xmm_reg) x,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return x;
}
