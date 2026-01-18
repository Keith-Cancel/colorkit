use super::*;

pub trait MapSealed {}

/// Specifies the order the color channels in a color.
/// Basically remaps the indexs used for `set()` and `get()`
pub trait ChannelMap: Copy + MapSealed {
    type Channels: ChannelCount;
    const MAP: &'static [usize];
    const MAP_REVERSE: &'static [usize];
}

/// Width `1` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map1;

impl MapSealed for Map1 {}
impl ChannelMap for Map1 {
    type Channels = Channel1;
    const MAP: &'static [usize] = &[0];
    const MAP_REVERSE: &'static [usize] = &[0];
}

/// Width `2` Marker type specifying the mapping of color channels
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Map2<const A: usize = 0, const B: usize = 1>;

impl<const A: usize, const B: usize> MapSealed for Map2<A, B> {}
impl<const A: usize, const B: usize> ChannelMap for Map2<A, B> {
    type Channels = Channel2;
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
impl<const A: usize, const B: usize, const C: usize> ChannelMap for Map3<A, B, C> {
    type Channels = Channel3;
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
impl<const A: usize, const B: usize, const C: usize, const D: usize> ChannelMap for Map4<A, B, C, D> {
    type Channels = Channel4;
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
impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize> ChannelMap
    for Map5<A, B, C, D, E>
{
    type Channels = Channel5;
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
impl<const A: usize, const B: usize, const C: usize, const D: usize, const E: usize, const F: usize> ChannelMap
    for Map6<A, B, C, D, E, F>
{
    type Channels = Channel6;
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
    ChannelMap for Map7<A, B, C, D, E, F, G>
{
    type Channels = Channel7;
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
> ChannelMap for Map8<A, B, C, D, E, F, G, H>
{
    type Channels = Channel8;
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
> ChannelMap for Map9<A, B, C, D, E, F, G, H, I>
{
    type Channels = Channel9;
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
