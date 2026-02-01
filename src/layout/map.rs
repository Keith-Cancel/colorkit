use colorkit::num_type::*;

pub trait MapSealed {}

/// Describes the logical channel ordering used by a [`Layout`](super::Layout).
///
/// Implementors provide two compile-time permutations:
/// - `MAP`: logical -> storage index (logical channel `i` is stored at `MAP[i]`)
/// - `MAP_REVERSE`: storage -> logical index (storage slot `j` contains logical `MAP_REVERSE[j]`)
///
/// This lets `Layout`-based operations (e.g. `get`/`set`) work with different
/// physical channel orderings (for example `ARGB` vs `RGBA`) without changing
/// the logical channel API or swapping values around.
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

/// Width `4` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map4<const A: usize = 0, const B: usize = 1, const C: usize = 2, const D: usize = 3>;

impl<const A: usize, const B: usize, const C: usize, const D: usize> MapSealed for Map4<A, B, C, D> {}
impl<const A: usize, const B: usize, const C: usize, const D: usize> LayoutMap for Map4<A, B, C, D> {
    type Channels = N4;
    const MAP: &'static [usize] = &[A, B, C, D];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 4];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev
    };
}

/// Width `5` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map5<const A: usize = 0, const B: usize = 1, const C: usize = 2, const D: usize = 3, const E: usize = 4>;

impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize> MapSealed for Map5<A, B, C, D, E> {}
impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize> LayoutMap for Map5<A, B, C, D, E> {
    type Channels = N5;
    const MAP: &'static [usize] = &[A, B, C, D, E];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 5];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev
    };
}

/// Width `6` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map6<
    const A: usize = 0,
    const B: usize = 1,
    const C: usize = 2,
    const D: usize = 3,
    const E: usize = 4,
    const F: usize = 5,
>;

impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize, const F: usize> MapSealed
    for Map6<A, B, C, D, E, F>
{
}
impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize, const F: usize> LayoutMap
    for Map6<A, B, C, D, E, F>
{
    type Channels = N6;
    const MAP: &'static [usize] = &[A, B, C, D, E, F];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 6];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev[F] = 5;
        rev
    };
}

/// Width `7` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map7<
    const A: usize = 0,
    const B: usize = 1,
    const C: usize = 2,
    const D: usize = 3,
    const E: usize = 4,
    const F: usize = 5,
    const G: usize = 6,
>;

impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize, const F: usize, const G: usize>
    MapSealed for Map7<A, B, C, D, E, F, G>
{
}
impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize, const F: usize, const G: usize>
    LayoutMap for Map7<A, B, C, D, E, F, G>
{
    type Channels = N7;
    const MAP: &'static [usize] = &[A, B, C, D, E, F, G];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 7];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev[F] = 5;
        rev[G] = 6;
        rev
    };
}

/// Width `8` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map8<
    const A: usize = 0,
    const B: usize = 1,
    const C: usize = 2,
    const D: usize = 3,
    const E: usize = 4,
    const F: usize = 5,
    const G: usize = 6,
    const H: usize = 7,
>;

impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
> MapSealed for Map8<A, B, C, D, E, F, G, H>
{
}
impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
> LayoutMap for Map8<A, B, C, D, E, F, G, H>
{
    type Channels = N8;
    const MAP: &'static [usize] = &[A, B, C, D, E, F, G, H];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 8];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev[F] = 5;
        rev[G] = 6;
        rev[H] = 7;
        rev
    };
}

/// Width `9` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map9<
    const A: usize = 0,
    const B: usize = 1,
    const C: usize = 2,
    const D: usize = 3,
    const E: usize = 4,
    const F: usize = 5,
    const G: usize = 6,
    const H: usize = 7,
    const I: usize = 8,
>;

impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
    const I: usize,
> MapSealed for Map9<A, B, C, D, E, F, G, H, I>
{
}
impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
    const I: usize,
> LayoutMap for Map9<A, B, C, D, E, F, G, H, I>
{
    type Channels = N9;
    const MAP: &'static [usize] = &[A, B, C, D, E, F, G, H, I];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 9];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev[F] = 5;
        rev[G] = 6;
        rev[H] = 7;
        rev[I] = 8;
        rev
    };
}

/// Width `10` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map10<
    const A: usize = 0,
    const B: usize = 1,
    const C: usize = 2,
    const D: usize = 3,
    const E: usize = 4,
    const F: usize = 5,
    const G: usize = 6,
    const H: usize = 7,
    const I: usize = 8,
    const J: usize = 9,
>;

impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
    const I: usize,
    const J: usize,
> MapSealed for Map10<A, B, C, D, E, F, G, H, I, J>
{
}
impl<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
    const F: usize,
    const G: usize,
    const H: usize,
    const I: usize,
    const J: usize,
> LayoutMap for Map10<A, B, C, D, E, F, G, H, I, J>
{
    type Channels = N10;
    const MAP: &'static [usize] = &[A, B, C, D, E, F, G, H, I, J];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 10];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev[E] = 4;
        rev[F] = 5;
        rev[G] = 6;
        rev[H] = 7;
        rev[I] = 8;
        rev[J] = 9;
        rev
    };
}
