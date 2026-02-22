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

#[inline(always)]
pub fn roundevenf(mut x: f32) -> f32 {
    unsafe {
        asm!(
            "roundss {r}, {r}, 0x8",
            r = inout(xmm_reg) x,
            // https://doc.rust-lang.org/rust-by-example/unsafe/asm.html#options
            options(nomem, nostack, pure),
        )
    };
    return x;
}

#[inline(always)]
pub fn fma(mut x: f64, a: f64, b: f64) -> f64 {
    unsafe {
        asm!(
            "vfmadd132sd {x}, {b}, {a}",
            x = inout(xmm_reg) x,
            a = in(xmm_reg) a,
            b = in(xmm_reg) b,
            options(nomem, nostack, pure),
        );
    }
    return x;
}

#[cfg(all(test, not(miri)))]
mod test {
    #[allow(unused)]
    use super::*;
    #[test]
    #[cfg(target_feature = "fma")]
    fn fma_f64() {
        {
            let x = fma(12.0, 3.0, 1.0);
            assert_eq!(x, 37.0);
        }
    }
}
