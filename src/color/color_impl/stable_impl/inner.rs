use std::ops::Index;
use std::ops::IndexMut;
use std::slice;

use colorkit::space::ColorSpace;
use colorkit::utils::*;

use super::Color;
use super::macros::color_inner;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct ColorInner<N: Number>(pub(crate) N::Arr<f32>);

// Manually handle each number so I can have const methods.
with_nums!(color_inner!);

impl<N: Number> ColorInner<N> {
    #[inline]
    pub const fn try_from_slice(slice: &[f32]) -> Option<Self> {
        if slice.len() != N::N {
            return None;
        }
        let ptr = slice.as_ptr() as *const N::Arr<f32>;
        // Safety:
        // Number is a sealed trait so Arr associated type is always an array
        // We check that the length slice is equal to the array before reading.
        // Also alignment is the same for both the slice and array.
        return Some(Self(unsafe { ptr.read() }));
    }

    #[inline]
    pub const fn get(&self, index: usize) -> Option<&f32> {
        if index >= N::N {
            return None;
        }
        let slc = self.as_slice();
        return Some(&slc[index]);
    }

    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut f32> {
        if index >= N::N {
            return None;
        }
        let slc = self.as_mut_slice();
        return Some(&mut slc[index]);
    }

    #[inline]
    pub const fn as_slice(&self) -> &[f32] {
        let ptr = (&self.0) as *const _ as *const f32;
        // Safety:
        // Arr<f32> is an array so this safe.
        return unsafe { slice::from_raw_parts(ptr, N::N) };
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [f32] {
        let ptr = (&mut self.0) as *mut _ as *mut f32;
        // Safety:
        // Arr<f32> is an array so this safe.
        return unsafe { slice::from_raw_parts_mut(ptr, N::N) };
    }

    /// Transmute this ColorInner into a `Color<S>`
    /// Safety: should only be called if the Color<S> is actually
    /// a wrapper around Self
    pub const unsafe fn transmute<S: ColorSpace>(self) -> Color<S> {
        #[repr(C)]
        union Transmute<Sp: ColorSpace, No: Number> {
            color: Color<Sp>,
            inner: ColorInner<No>,
        }
        // Can't use std::mem::transmute with a generic
        // so get around it with a union.
        let tmp = Transmute::<S, N> {
            inner: self
        };
        // Safety:
        // Caller should ensure Color<S> is just a wrapper around Self.
        return unsafe { tmp.color };
    }
}

impl<N: Number> Index<usize> for ColorInner<N> {
    type Output = f32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<N: Number> IndexMut<usize> for ColorInner<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}
