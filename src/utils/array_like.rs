use std::array::from_fn;
use std::array::repeat;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::Index;
use std::ops::IndexMut;

/// Trait for types that behave like a fixed-size array (`[T; N]`).
///
/// Aggregates common array operations (slicing, indexing, construction) so code
/// can be generic over "array-like" containers. Implemented for the builtin
/// array type `[T; N]`.
///
/// # Example
/// ```rust
/// use colorkit::utils::ArrayLike;
/// let a = <[u32; 4] as ArrayLike<u32>>::from_fn(|i| i as u32);
/// assert_eq!(a.as_slice(), &[0, 1, 2, 3]);
/// ```
pub trait ArrayLike<T>:
    AsRef<[T]> + AsMut<[T]> + Borrow<[T]> + BorrowMut<[T]> + Index<usize, Output = T> + IndexMut<usize, Output = T>
{
    /// Length, but as a constant.
    const LEN: usize;
    /// Gets a reference to the array.
    ///
    /// If `N` is not the same length returns [`None`]`;
    fn as_array<const N: usize>(&self) -> Option<&[T; N]>;
    /// View as a slice reference.
    fn as_slice(&self) -> &[T];
    /// View as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [T];
    /// Returns a raw pointer to the underlying buffer.
    fn as_ptr(&self) -> *const T;
    /// Returns a raw mutable pointer to the underlying buffer.
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_slice().as_mut_ptr()
    }

    /// Construct by calling `f(i)` for each index (same semantics as [`std::array::from_fn`]).
    fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self;
    /// Get element or `None`.
    fn get(&self, i: usize) -> Option<&T>;
    /// Get mutable element or `None`.
    fn get_mut(&mut self, i: usize) -> Option<&mut T>;
    /// Number of elements.
    fn len(&self) -> usize;
    /// Create an array filled with `value` (requires `T: Clone`).
    fn repeat(value: T) -> Self
    where
        T: Clone;
}

impl<T, const N: usize> ArrayLike<T> for [T; N] {
    const LEN: usize = N;

    fn as_array<const N2: usize>(&self) -> Option<&[T; N2]> {
        if N2 != N {
            return None;
        }
        let ptr = self.as_ptr();
        // SAFETY: Length is the same.
        return Some(unsafe { &*(ptr.cast()) });
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        return self as *const [T] as *const T;
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        return self as *mut [T] as *mut T;
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        return self;
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        return self;
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        return self.as_slice().get(index);
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        return self.as_mut_slice().get_mut(index);
    }

    fn from_fn<F: FnMut(usize) -> T>(f: F) -> Self {
        return from_fn(f);
    }

    #[inline]
    fn len(&self) -> usize {
        return N;
    }

    #[inline]
    fn repeat(value: T) -> Self
    where
        T: Clone,
    {
        return repeat(value);
    }
}
