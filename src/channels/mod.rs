//! Traits for dealing with channel width and similar.
mod map;

pub use map::ChannelMap;
pub use map::Map1;
pub use map::Map2;
pub use map::Map3;
pub use map::Map4;
pub use map::Map5;
pub use map::Map6;
pub use map::Map7;
pub use map::Map8;
pub use map::Map9;

mod private {
    pub trait ChannelsSealed {}
    pub trait GenericSealed {}
}

use private::ChannelsSealed;
use private::GenericSealed;

/// This more of a marker trait to around this feature not being stable
/// #![feature(associated_const_equality)].
///
/// <https://github.com/rust-lang/rust/issues/92827>
pub trait ChannelCount: ChannelsSealed {
    /// Number of Channels
    const COUNT: usize;
    /// Next Size up
    type Next: ChannelCount;
    /// Next Size Down
    type Prev: ChannelCount;
}

/// Marker Type to get defined [`ChannelCount`] types for a generic N.
///
/// Just machinery to get around the lack of
/// #![feature(associated_const_equality)].
/// This does limit such generic up to a predifined N.
pub struct ChannelGeneric<const N: usize>;

impl<const N: usize> GenericSealed for ChannelGeneric<N> {}

/// Marker trait just for [`ChannelGeneric`]
pub trait GenericCount: GenericSealed {
    type CountType: ChannelCount;
}

macro_rules! impl_chan_counts {
    ($($name:ident : $cnt:expr),+ $(,)?) => {
        $(
            // Marker Type denoting a channel width of `$cnt`
            #[doc = concat!("A marker type denoting a channel width of `", stringify!($cnt), "`")]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $name;

            impl ChannelsSealed for $name {}

            impl ChannelCount for $name {
                const COUNT: usize = $cnt;
                type Next = <ChannelGeneric<const { $cnt + 1 }> as GenericCount>::CountType;
                type Prev = <ChannelGeneric<const { $cnt - 1 }> as GenericCount>::CountType;
            }

            impl GenericCount for ChannelGeneric<$cnt> {
                type CountType = $name;
            }
        )+
    };
}

impl_chan_counts! {
    Channel1: 1,
    Channel2: 2,
    Channel3: 3,
    Channel4: 4,
    Channel5: 5,
    Channel6: 6,
    Channel7: 7,
    Channel8: 8,
    Channel9: 9,
}

// Marker Type denoting No Channels
#[doc = concat!("A marker type denoting a channel width of `", stringify!($cnt), "`")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelNone;
impl ChannelsSealed for ChannelNone {}
impl ChannelCount for ChannelNone {
    const COUNT: usize = 0;
    type Next = Channel1;
    type Prev = ChannelNone;
}

impl GenericCount for ChannelGeneric<0> {
    type CountType = ChannelNone;
}
// We stop at 9 so aftwards just return None
impl GenericCount for ChannelGeneric<10> {
    type CountType = ChannelNone;
}
