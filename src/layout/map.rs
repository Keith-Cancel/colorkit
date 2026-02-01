use colorkit::num_type::*;

pub trait MapSealed {}

/// Describes the channel ordering used by a [`Layout`](super::Layout).
///
/// Implementors provide two compile-time permutations:
/// - `MAP`: logical -> storage index (logical channel `i` is stored at `MAP[i]`)
/// - `MAP_REVERSE`: storage -> logical index (storage slot `j` contains logical `MAP_REVERSE[j]`)
///
/// This lets `Layout`-based operations (e.g. `get`/`set`) work with different
/// physical channel orderings (for example `ARGB` vs `RGBA`) without changing
/// the logical channel API.
pub trait LayoutMap: Copy + MapSealed {
    type Channels: Number;
    const MAP: &'static [usize];
    const MAP_REVERSE: &'static [usize];
}

/// Width `1` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map1;

impl MapSealed for Map1 {}
impl LayoutMap for Map1 {
    type Channels = N1;
    const MAP: &'static [usize] = &[0];
    const MAP_REVERSE: &'static [usize] = &[0];
}

/// Width `2` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map2<const A: usize = 0, const B: usize = 1>;

impl<const A: usize, const B: usize> MapSealed for Map2<A, B> {}
impl<const A: usize, const B: usize> LayoutMap for Map2<A, B> {
    type Channels = N2;
    const MAP: &'static [usize] = &[A, B];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 2];
        rev[A] = 0;
        rev[B] = 1;
        rev
    };
}

/// Width `3` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map3<const A: usize = 0, const B: usize = 1, const C: usize = 2>;

impl<const A: usize, const B: usize, const C: usize> MapSealed for Map3<A, B, C> {}
impl<const A: usize, const B: usize, const C: usize> LayoutMap for Map3<A, B, C> {
    type Channels = N3;
    const MAP: &'static [usize] = &[A, B, C];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 3];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev
    };
}
