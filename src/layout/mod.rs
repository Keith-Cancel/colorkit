//! Types that can be a color [`Layout`] for [`Color`](`colorkit::color::Color`)
//!
//! This module defines the [`Layout`] trait and conversion traits:
//!
//! - [`FromLayout`] / [`IntoLayout`] Converts between the [Scalar] types of layouts.
//!
//! - [`GrowLayout`] / [`TruncateLayout`] Converts channel width of Layouts, and [Scalar] types.

mod packed_565;
mod planar;

use colorkit::scalar::Dither;
use colorkit::scalar::NoDither;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::scalar::Scalar;
/// Packed 5-6-5 channel layout in a [u16].
pub use packed_565::Packed565;
/// Planar or array like channel layout
pub use planar::Planar;
/// Alias of [Planar<S, 3>]
pub use planar::Planar3;
/// Alias of [Planar<S, 4>]
pub use planar::Planar4;

/// Bridge between a layout and its underlying storage.
///
/// `Storage` must be convertible to and from the layout.
pub trait LayoutStorage: Sized + Into<Self::Storage> {
    /// Underlying storage type for this layout.
    type Storage: Into<Self>;

    /// View of the underlying storage.
    fn as_storage(&self) -> &Self::Storage;
    /// Mutable view of the underlying storage.
    fn as_storage_mut(&mut self) -> &mut Self::Storage;
}

/// A storage layout for a fixed number of channels.
///
/// Implementors represent how channel values are stored.
///
/// See [`Planar`] and [`Packed565`].
// TODO:
// If `min_const_generic_args`` and/or `associated_const_equality`
// are stabilized this trait can be made a little more ergonomic
// or have better constraints on the methods, for example instead
// maybe panicing on channel width difference in requantize I could
// use `associated_const_equality` to enforce a constraint.
pub trait Layout: Copy + Default + LayoutStorage {
    /// Default Value
    const DEFAULT: Self;
    /// Total number of channels
    const CHANNELS: usize;
    /// A type capable of holding the raw value of each channel.
    type ChannelType;

    /// Total number of channels for this layout.
    #[inline(always)]
    fn channels() -> usize {
        return Self::CHANNELS;
    }

    /// Create a layout by calling `fun` for each channel index.
    ///
    /// Similar to [std::array::from_fn]
    fn from_fn_raw<F: FnMut(usize) -> Self::ChannelType>(fun: F) -> Self;

    /// Create a layout by calling `fun` for each channel from the returned
    /// [`NormF32`] and rounding mode.
    fn from_fn_norm<F: FnMut(usize) -> NormF32>(fun: F, round: Rounding) -> Self {
        return Self::from_fn_norm_dither(fun, round, &mut NoDither);
    }

    /// Create a layout by calling `fun` for each channel from the returned
    /// [`NormF32`] and given rounding and dither.
    fn from_fn_norm_dither<F: FnMut(usize) -> NormF32, D: Dither>(fun: F, round: Rounding, dither: &mut D) -> Self;

    /// Return the value at channel `index` as a [`NormF32`]
    ///
    /// # Panics
    /// May Panic if `index` >= [`Layout::CHANNELS`]
    fn get_norm(&self, index: usize) -> NormF32;

    /// Sets the channel at `index` from a [`NormF32`] using the given rounding mode.
    ///
    /// # Panics
    /// May Panic if `index` >= [`Layout::CHANNELS`]
    fn set_norm(&mut self, value: NormF32, index: usize, round: Rounding) {
        return self.set_norm_dither(value, index, round, &mut NoDither);
    }

    /// Sets the channel at `index` from a [`NormF32`] using rounding and dithering.
    ///
    /// # Panics
    /// May Panic if `index` >= [`Layout::CHANNELS`]
    fn set_norm_dither<D: Dither>(&mut self, value: NormF32, index: usize, round: Rounding, dither: &mut D);

    /// Return the raw value at `index`.
    ///
    /// # Panics
    /// May Panic if `index` >= [`Layout::CHANNELS`]
    fn get_raw(&self, index: usize) -> Self::ChannelType;
    /// Set the raw value at `index`.
    ///
    /// # Panics
    /// May Panic if `index` >= [`Layout::CHANNELS`]
    fn set_raw(&mut self, index: usize, value: Self::ChannelType);

    /// Converts this layout into another layout with the same channel count.
    ///
    /// # Panics
    /// May panic if the channel counts do not match.
    fn requantize<L: Layout<CHANNELS = { Self::CHANNELS }>>(self, round: Rounding) -> L {
        debug_assert!(L::CHANNELS == Self::CHANNELS);
        return L::from_fn_norm(|i| self.get_norm(i), round);
    }

    /// Like [`Layout::requantize`], but applies dithering.
    ///
    /// # Panics
    /// May panic if the channel counts do not match.
    fn requantize_dither<L: Layout<CHANNELS = { Self::CHANNELS }>, D: Dither>(
        self,
        round: Rounding,
        dither: &mut D,
    ) -> L {
        debug_assert!(L::CHANNELS == Self::CHANNELS);
        return L::from_fn_norm_dither(|i| self.get_norm(i), round, dither);
    }
}

/// A [`LayoutScalar`] is layout whose [Layout::ChannelType] is a [`Scalar`] and uses its full value range.
///
/// All channels are assumed to have the same bit width and use the entire range
/// of the scalar type.
///
/// # Note
/// This trait is probably **not** suitable for packed formats like sRGB565, where channels
/// may have different bit widths or can only use part of the underlying ChannelType.
///
/// For example [`u8`] can hold each channel of sRGB565 and [`u8`] is scalar, but underlying channel widths
/// can't use all the bits of [`u8`] without conversion. Even [`BitUint`](colorkit::scalar::BitUint)
/// which is a scalar can't be used without conversion because the differing bit widths. Since we would
/// need to return a BitUint<5> or BitUint<6> on `get()`. So unless you want to convert each time and
/// don't mind the bias or error from that you probably should not implement this on such layouts.
pub trait LayoutScalar: Layout<ChannelType: Scalar> {
    /// Create a layout by calling `fun` for each channel index.
    ///
    /// Similar to [std::array::from_fn]
    fn from_fn<F: FnMut(usize) -> Self::ChannelType>(fun: F) -> Self {
        return Self::from_fn_raw(fun);
    }
    /// Return the raw value at `index`.
    ///
    /// # Panics
    /// May Panic if `index` >= the `COUNT` constant on [`Layout::Channels`]
    ///
    /// (see [`ChannelCount::COUNT`] for the definition).
    fn get(&self, index: usize) -> Self::ChannelType {
        return self.get_raw(index);
    }
    /// Set the scalar at `index`.
    ///
    /// # Panics
    /// May Panic if `index` >= the `COUNT` constant on [`Layout::Channels`]
    ///
    /// (see [`ChannelCount::COUNT`] for the definition).
    fn set(&mut self, index: usize, value: Self::ChannelType) {
        return self.set_raw(index, value);
    }
}

// Layout Conversion Traits
// ==================================================
/// Convert from another layout `L` into `Self`.
///
/// This is like [std::convert::From] but does not imply lossless conversion.
/// Implementations should not change channel count.
pub trait FromLayout<L: Layout>: Layout {
    fn from_layout(layout: L) -> Self;
}

impl<L1: Layout<CHANNELS = { L2::CHANNELS }>, L2: Layout> FromLayout<L1> for L2 {
    fn from_layout(layout: L1) -> Self {
        return layout.requantize(Rounding::Nearest);
    }
}

/// Convert this layout into another layout `L`
///
/// This is like [std::convert::Into] but does not imply lossless conversion.
/// Implementations should not change channel count.
pub trait IntoLayout<L: Layout>: Layout {
    fn into_layout(self) -> L;
}

impl<L1: Layout<CHANNELS = { L2::CHANNELS }>, L2: FromLayout<L1>> IntoLayout<L1> for L2 {
    fn into_layout(self) -> L1 {
        return L1::from_layout(self);
    }
}

/*
/// Convert a layout into another layout `L` with the same or more channels.
///
/// If `L` has more channels, missing channels are filled according to `L`'s
/// rules (for example using defaults). Scalar conversion may occur.
pub trait GrowLayout<L: Layout>: Layout {
    fn grow_layout(self) -> L;
}

/// Convert a layout into another layout `L` with the same or fewer channels.
///
/// If `L` has fewer channels, extra channels are discarded. Scalar conversion
/// may occur.
pub trait TruncateLayout<L: Layout>: Layout {
    fn truncate_layout(self) -> L;
}

// Some Conversion implentations
// ==================================================


impl<S: Scalar> FromLayout<Packed565> for Planar<S, 3> {
    fn from_layout(s: Packed565) -> Self {
        let a = <Planar<S, 3>>::from_array([
            s.get(0).into_scalar(),
            s.get(1).into_scalar(),
            s.get(2).into_scalar(),
        ]);
        return a;
    }
}

impl<S: Scalar> FromLayout<Planar<S, 3>> for Packed565 {
    fn from_layout(s: Planar<S, 3>) -> Self {
        let mut ret = Self::default();
        ret.set(0, s.get(0).into_scalar());
        ret.set(1, s.get(1).into_scalar());
        ret.set(2, s.get(2).into_scalar());
        return ret;
    }
}

impl<const N: usize, S: Scalar> GrowLayout<Planar<S, N>> for Packed565
where
    Planar<S, 3>: GrowLayout<Planar<S, N>>,
{
    fn grow_layout(self) -> Planar<S, N> {
        let a = <Planar<S, 3>>::from_array([
            self.get(0).into_scalar(),
            self.get(1).into_scalar(),
            self.get(2).into_scalar(),
        ]);
        return a.grow_layout();
    }
}

impl<const N: usize, S: Scalar> TruncateLayout<Planar<S, N>> for Packed565
where
    Planar<S, 3>: TruncateLayout<Planar<S, N>>,
{
    fn truncate_layout(self) -> Planar<S, N> {
        let a = <Planar<S, 3>>::from_array([
            self.get(0).into_scalar(),
            self.get(1).into_scalar(),
            self.get(2).into_scalar(),
        ]);
        return a.truncate_layout();
    }
}
*/
