use core::array;
use core::ops::Index;
use core::ops::IndexMut;

use colorkit::num_type::Num;
use colorkit::num_type::ToNumber;
use colorkit::scalar::Dither;
use colorkit::scalar::IntoScalar;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::scalar::Scalar;

use super::Layout;
use super::LayoutScalar;
use super::LayoutStorage;

/// A planar (contiguous) channel layout backed by an array.
///
/// This is a thin, `#[repr(transparent)]` wrapper around `[S; N]` that
/// provides the [`Layout`] / [`LayoutStorage`] behavior for N channels.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Planar<S: Scalar, const N: usize = 3>([S; N]);

/// Alias of [Planar<S, 3>]
pub type Planar3<S> = Planar<S, 3>;
/// Alias of [Planar<S, 4>]
pub type Planar4<S> = Planar<S, 4>;

impl<S: Scalar, const N: usize> Planar<S, N> {
    /// Create a `Planar` from a raw array without copying.
    #[inline]
    pub const fn from_array(array: [S; N]) -> Self {
        return Self(array);
    }

    /// Consume this `Planar` and return the inner array.
    #[inline]
    pub const fn into_array(self) -> [S; N] {
        return self.0;
    }

    /// Elementwise scalar conversion between planar layouts.
    ///
    /// Uses [IntoScalar] to convert each channel.
    #[inline]
    pub fn into_scalar<S1: Scalar>(self) -> Planar<S1, N> {
        // Looking at the implention and because it's in the std::lib
        // this has access to the `array_assume_init` even in stable.
        // It does what I basically was doing under the hood, but
        // but with all safe code.
        return Planar::<S1, N>(array::from_fn(|i| self.0[i].into_scalar()));
    }
}

impl<S: Scalar, const N: usize> Default for Planar<S, N> {
    /// Default value: every channel set to `S::DEFAULT`.
    fn default() -> Self {
        return Self([S::DEFAULT; N]);
    }
}

impl<S: Scalar, const N: usize> From<[S; N]> for Planar<S, N> {
    /// Construct `Planar` from an array (same as [`Planar::from_array`])
    #[inline]
    fn from(value: [S; N]) -> Self {
        return Self(value);
    }
}

impl<S: Scalar, const N: usize> From<Planar<S, N>> for [S; N] {
    /// Extract the inner array from `Planar` (same as [`Planar::into_array`])
    #[inline]
    fn from(value: Planar<S, N>) -> Self {
        return value.0;
    }
}

impl<S: Scalar, const N: usize> Index<usize> for Planar<S, N> {
    type Output = S;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<S: Scalar, const N: usize> IndexMut<usize> for Planar<S, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

/// Expose the inner array as the layout's `Storage`.
impl<S: Scalar, const N: usize> LayoutStorage for Planar<S, N> {
    type Storage = [S; N];
    #[inline]
    fn as_storage(&self) -> &Self::Storage {
        return &self.0;
    }

    #[inline]
    fn as_storage_mut(&mut self) -> &mut Self::Storage {
        return &mut self.0;
    }
}

impl<S: Scalar, const N: usize> Layout for Planar<S, N>
where
    Num<N>: ToNumber,
{
    const DEFAULT: Self = Self([S::DEFAULT; N]);
    type Channels = <Num<N> as ToNumber>::Number;
    type ChannelType = S;

    fn get_norm(&self, index: usize) -> NormF32 {
        return self[index].into_norm();
    }

    fn set_norm(&mut self, value: NormF32, index: usize, round: Rounding) {
        self[index] = S::from_norm(value, round);
    }

    fn set_norm_dither<D: Dither>(&mut self, value: NormF32, index: usize, round: Rounding, dither: &mut D) {
        self[index] = S::from_norm_dither(value, round, dither);
    }

    fn from_fn_norm<F: FnMut(usize) -> NormF32>(fun: F, round: Rounding) -> Self {
        let mut fun = fun;
        return Self(array::from_fn(|i| {
            let v = fun(i);
            S::from_norm(v, round)
        }));
    }

    fn from_fn_norm_dither<F: FnMut(usize) -> NormF32, D: Dither>(fun: F, round: Rounding, dither: &mut D) -> Self {
        let mut fun = fun;
        return Self(array::from_fn(|i| {
            let v = fun(i);
            S::from_norm_dither(v, round, dither)
        }));
    }

    #[inline]
    fn from_fn_raw<F: FnMut(usize) -> S>(fun: F) -> Self {
        return Self(array::from_fn(fun));
    }

    #[inline]
    fn get_raw(&self, index: usize) -> S {
        return self[index];
    }

    #[inline]
    fn set_raw(&mut self, index: usize, value: S) {
        self[index] = value;
    }
}

impl<S: Scalar, const N: usize> LayoutScalar for Planar<S, N> where Num<N>: ToNumber {}

#[cfg(test)]
mod test {
    use colorkit::layout::FromLayout;
    use colorkit::layout::IntoLayout;
    use colorkit::layout::LayoutScalar;
    use colorkit::scalar::NormF32;

    use super::*;

    #[test]
    fn set_get() {
        let mut p = <Planar3<u8>>::from_array([51, 102, 153]);

        assert_eq!(p.get(0), 51);
        assert_eq!(p.get(1), 102);
        assert_eq!(p.get(2), 153);

        assert_eq!(p.get_raw(0), 51);
        assert_eq!(p.get_raw(1), 102);
        assert_eq!(p.get_raw(2), 153);

        assert_eq!(p.get_norm(0), 0.2);
        assert_eq!(p.get_norm(1), 0.4);
        assert_eq!(p.get_norm(2), 0.6);

        p.set(0, 204);
        p.set_raw(1, 255);

        assert_eq!(p.get(0), 204);
        assert_eq!(p.get(1), 255);
        assert_eq!(p.get(2), 153);

        assert_eq!(p.get_raw(0), 204);
        assert_eq!(p.get_raw(1), 255);
        assert_eq!(p.get_raw(2), 153);

        assert_eq!(p.get_norm(0), 0.8);
        assert_eq!(p.get_norm(1), 1.0);
        assert_eq!(p.get_norm(2), 0.6);
    }

    #[test]
    fn test_from_layout() {
        let mut p0 = Planar::<u8>::default();
        p0.set(0, 255);
        p0.set(2, 51);

        p0 = p0.into_layout(); // Make sure self to self works
        assert_eq!(p0[0], 255);
        assert_eq!(p0[1], 0);
        assert_eq!(p0[2], 51);

        let mut p1 = Planar::<NormF32>::from_layout(p0);
        assert_eq!(p1[0], 1.0);
        assert_eq!(p1[1], 0.0);
        assert_eq!(p1[2], 0.2);

        p1.set(0, NormF32::new_clamped(0.4));
        p1.set(1, NormF32::new_clamped(0.498));
        p0 = p1.into_layout();
        assert_eq!(p0[0], 102);
        assert_eq!(p0[1], 127);
        assert_eq!(p0[2], 51);
    }
}
