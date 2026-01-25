pub(crate) mod arch;
mod universal;

// TODO: arch and const fns
// Ideally all the math functions would be `const fn`, but that means
// I can't use hardware features =(
//
// If this gets stabilized:
// https://github.com/rust-lang/rust/issues/124625
// update my impls to take advantage of it, although not
// likely anytime time soon since the rfc is not even done
// at the time of writing this sighs....
// I guess in the mean time just add const version that
// that calls the universal implementation.

pub use universal::quirtf;
pub use universal::ulp_int_diff_f32;

/// Computes the square root
pub fn sqrtf(x: f32) -> f32 {
    arch::arch_fn!(name: sqrtf, args: x);
    #[allow(unused)]
    return universal::sqrtf(x);
}

// Const functions
#[rustfmt::skip]
/// Computes the square root with a `const fn`
pub use universal::sqrtf as sqrtf_const;

/// Common math functions
///
/// This is to make up for the fact #[no_std] does
/// have all math functions for f32, so I have reimplemented
/// some for a no standard enviroment.
///
/// I have also have some tests that evaluate the accuracy and
/// performance of these functions in the root of this repo
/// called `test_math`
pub trait MathFuncs {
    /// Computes the square root
    fn sqrt(self) -> Self;
    /// Computes the quintic root or 5th root.
    fn quirt(self) -> Self;
    /// ULP int difference between two values.
    fn ulp_int_diff(self, other: Self) -> u32;
}

impl MathFuncs for f32 {
    fn sqrt(self) -> f32 {
        return sqrtf(self);
    }

    fn quirt(self) -> f32 {
        return universal::quirtf(self);
    }

    fn ulp_int_diff(self, other: f32) -> u32 {
        return universal::ulp_int_diff_f32(self, other);
    }
}
