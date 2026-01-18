mod rgb;
mod oklab;
mod trait_impl;

use std::marker::PhantomData;

use colorkit::color::*;
use colorkit::layout::Layout;
use colorkit::layout::Planar3;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ColorType<S = Srgb, L = Planar3<u8>, A = AlphaNone, M = Map3> {
    pub layout: L,

    #[doc(hidden)]
    _sp: PhantomData<(A, M, S)>,
}

impl<S: ColorSpace, L: Layout, A: AlphaChannel, M: ChannelMap> ColorType<S, L, A, M> {
    /// Total number of channels.
    pub const CHANNEL_COUNT: usize = S::CHANNELS + (A::CHANNEL.is_some() as usize);

    pub(crate) const ASSERT_COLOR: () = Self::assert_color();

    /// Creates a new color with the given layout.
    pub const fn from_layout_const(layout: L) -> Self {
        let _ = Self::ASSERT_COLOR;
        return Self {
            layout,
            _sp: PhantomData,
        };
    }

    const fn assert_color() {
        assert!(
            Self::CHANNEL_COUNT <= L::CHANNELS,
            "Layout has too few channels for the given ColorSpace and Alpha channel"
        );

        if let Some(c) = A::CHANNEL {
            assert!(
                c < L::CHANNELS,
                "ColorType's Alpha channel index is outside the range of the Layout"
            );
        }

        let len = M::MAP.len();
        assert!(len == S::CHANNELS, "ColorSpace and Order channel size must be the same");

        let mut i = 0;
        while i < len {
            let a = M::MAP[i];
            let mut j = 0;
            assert!(
                a < S::CHANNELS,
                "ColorType's Order has a channel index outside the range of the Color Space"
            );
            while j < len {
                if i == j {
                    j += 1;
                    continue;
                }
                let b = M::MAP[j];
                assert!(a != b, "ColorType's Order has duplicate values.");

                j += 1;
            }
            i += 1;
        }
    }
}
