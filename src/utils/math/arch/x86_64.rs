use core::arch::asm;

#[inline(always)]
pub fn sqrtf(x: f32) -> f32 {
    let mut ret = x;
    unsafe {
        asm!(
            "sqrtss {r}, {r}",
            r = inout(xmm_reg) ret,
            options(nomem, nostack),
        )
    };
    return ret;
}
