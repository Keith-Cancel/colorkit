use core::arch::asm;

#[inline(always)]
pub fn sqrtf(x: f32) -> f32 {
    let mut ret = x;
    unsafe {
        asm!(
            "fsqrt s0, s0",
            //"fsqrt {r}, {r}",
            //r = inout(??) ret,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            //options(nomem, nostack, pure),
        )
    };
    return ret;
}
