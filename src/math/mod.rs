//! Floating point math functions. (e.g. [`sqrtf`], [`cbrtf`], [`quirtf`] ect..)
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

pub use universal::cbrtf;
pub use universal::quirtf;
pub use universal::ulp_int_diff_f32;

/// Computes the square root
pub fn sqrtf(x: f32) -> f32 {
    arch::arch_fn!(name: sqrtf, args: x);
    #[allow(unused)]
    return universal::sqrtf(x);
}

/// Get the integer part of the float. Truncates the fraction always to zero.
pub fn truncf(x: f32) -> f32 {
    arch::arch_fn!(name: truncf, args: x);
    #[allow(unused)]
    return universal::truncf(x);
}

// Const functions
#[rustfmt::skip]
/// Computes the square root with a `const fn`
///
pub use universal::sqrtf as sqrtf_const;
pub use universal::truncf as truncf_const;

/// Common math functions
///
/// This is to make up for the fact #\[no_std\] does
/// have all math functions for f32, so I have reimplemented
/// some for a no standard enviroment.
///
/// I have also have some tests that evaluate the accuracy and
/// performance of these functions in the root of this repo
/// called `test_math`
pub trait MathFuncs {
    /// Computes the square root
    fn sqrt(self) -> Self;
    /// Compute the cube root.
    fn cbrt(self) -> Self;
    /// Computes the quintic root or 5th root.
    fn quirt(self) -> Self;
    /// ULP int difference between two values.
    fn ulp_int_diff(self, other: Self) -> u32;
    /// Compare to floats are close enough with some tolerance/epsilon
    fn almost_eq(self, other: Self, tol: f32) -> bool;
    /// Get the integer part of the float. Truncates the fraction always to zero.
    fn trunc(self) -> Self;
}

impl MathFuncs for f32 {
    fn sqrt(self) -> f32 {
        return sqrtf(self);
    }

    fn cbrt(self) -> f32 {
        return cbrtf(self);
    }

    fn quirt(self) -> f32 {
        return universal::quirtf(self);
    }

    fn ulp_int_diff(self, other: f32) -> u32 {
        return universal::ulp_int_diff_f32(self, other);
    }

    fn almost_eq(self, other: f32, tol: f32) -> bool {
        return (self - other).abs() < tol;
    }

    fn trunc(self) -> f32 {
        return truncf(self);
    }
}

/// Defines the a bound on a color space channel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundF32 {
    Include(f32),
    Unbounded,
}

// TODO, maybe some inline asm or SSE intrinsics.
pub(crate) const fn matrix_3x3_vec3_mul(mat: &[f32; 9], vec: &[f32]) -> [f32; 3] {
    assert!(vec.len() == 3);
    let mut res = [0f32; 3];
    let mut i = 0usize;
    while i < 3 {
        res[0] += vec[i] * mat[i];
        res[1] += vec[i] * mat[i + 3];
        res[2] += vec[i] * mat[i + 6];
        i += 1;
    }
    return res;
}
