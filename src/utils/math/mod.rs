mod arch;
mod quirt;
mod ulp;

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

pub use quirt::quirtf;
pub use ulp::ulp_int_diff_f32;

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
    /// Computes the quintic root or 5th root.
    fn quirt(self) -> Self;
    /// ULP int difference between two values.
    fn ulp_int_diff(self, other: Self) -> u32;
}

impl MathFuncs for f32 {
    fn quirt(self) -> f32 {
        return quirtf(self);
    }

    fn ulp_int_diff(self, other: f32) -> u32 {
        return ulp_int_diff_f32(self, other);
    }
}
