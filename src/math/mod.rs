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
pub use universal::roundf;
pub use universal::ulp_int_diff_f32;

/// Computes the square root
pub fn sqrtf(x: f32) -> f32 {
    arch::arch_fn!(name: sqrtf, args: x);
    #[allow(unused)]
    return universal::sqrtf(x);
}

/// Rounds to the nearest integer to the provided value.
///
/// In the event the value is exactly in the middle it
/// will round to the nearest even integer.
pub fn roundevenf(x: f32) -> f32 {
    arch::arch_fn!(name: roundevenf, args: x);
    #[allow(unused)]
    return universal::roundevenf(x);
}

/// Get the integer part of the float. Truncates the fraction always to zero.
pub fn truncf(x: f32) -> f32 {
    arch::arch_fn!(name: truncf, args: x);
    #[allow(unused)]
    return universal::truncf(x);
}
/// Rounds the integer greater than or equal to the provided value.
///
/// Similar to [`truncf`], but instead of torwards zero, it's
/// torwards positive infinity.
pub fn ceilf(x: f32) -> f32 {
    arch::arch_fn!(name: ceilf, args: x);
    #[allow(unused)]
    return universal::ceilf(x);
}
/// Rounds the integer less than or equal the provided value.
///
/// Similar to [`truncf`], but instead of torwards zero, it's
/// torwards negative infinity.
pub fn floorf(x: f32) -> f32 {
    arch::arch_fn!(name: floorf, args: x);
    #[allow(unused)]
    return universal::floorf(x);
}

// Const functions
#[rustfmt::skip]
pub use universal::ceilf as ceilf_const;
pub use universal::floorf as floorf_const;
pub use universal::roundevenf as roundevenf_const;
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
pub trait MathFuncs: Sized {
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
    /// Rounds the integer greater than or equal to the provided value.
    ///
    /// Similar to [`truncf`], but instead of torwards zero, it's
    /// torwards positive infinity.
    fn ceil(self) -> Self;
    /// Rounds the integer less than or equal the provided value.
    ///
    /// Similar to [`truncf`], but instead of torwards zero, it's
    /// torwards negative infinity.
    fn floor(self) -> Self;
    /// Rounds to the nearest integer to the provided value.
    ///
    /// In the event the value is exactly in the middle it
    /// will round away from zero.
    fn round(self) -> Self;
    /// Rounds to the nearest integer to the provided value.
    ///
    /// In the event the value is exactly in the middle it
    /// will round to the nearest even integer.
    fn roundeven(self) -> Self;
    /// Rounds to the nearest integer to the provided value.
    ///
    /// In the event the value is exactly in the middle it
    /// will round to the nearest even integer.
    ///
    /// Alias of [`MathFuncs::roundeven`]
    fn round_ties_even(self) -> Self {
        return Self::roundeven(self);
    }
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

    fn ceil(self) -> f32 {
        return ceilf(self);
    }

    fn floor(self) -> f32 {
        return floorf(self);
    }

    fn round(self) -> f32 {
        return roundf(self);
    }

    fn roundeven(self) -> Self {
        return roundevenf(self);
    }
}

/// Defines the a bound on a color space channel
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundF32 {
    Include(f32),
    Unbounded,
}

impl BoundF32 {
    #[inline]
    const fn in_bounds(min: BoundF32, max: BoundF32, value: f32) -> bool {
        !matches!(max, BoundF32::Include(m) if value > m)
            && !matches!(min, BoundF32::Include(m) if value < m)
    }
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

// Tests
#[cfg(test)]
mod tests;
