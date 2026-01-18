use std::marker::PhantomData;

use colorkit::color::*;
use colorkit::layout::Layout;
use colorkit::scalar::Scalar;

use super::ColorType;

impl<S: ColorSpace, L: Layout, A: AlphaChannel, M: ChannelMap> Default for ColorType<S, L, A, M> {
    #[inline]
    fn default() -> Self {
        let _ = Self::ASSERT_COLOR;
        let mut ret = Self {
            layout: L::DEFAULT,
            _sp:    PhantomData,
        };
        let _ = ret.try_set_alpha(<L::Scalar as Scalar>::DEFAULT);
        return ret;
    }
}

impl<S: ColorSpace, L: Layout, A: AlphaChannel, M: ChannelMap> ColorFormat for ColorType<S, L, A, M> {
    type Scalar = L::Scalar;
    type Space = S;
    type Alpha = A;
    type Layout = L;
    type Map = M;

    #[inline]
    fn from_fn<F: FnMut(usize) -> Self::Scalar>(fun: F) -> Self {
        let _ = Self::ASSERT_COLOR;
        return Self {
            layout: L::from_fn(fun),
            _sp:    PhantomData,
        };
    }

    #[inline]
    fn from_layout(layout: Self::Layout) -> Self {
        let _ = Self::ASSERT_COLOR;
        return Self {
            layout,
            _sp: PhantomData,
        };
    }

    #[inline]
    fn as_layout(&self) -> &Self::Layout {
        return &self.layout;
    }

    #[inline]
    fn as_layout_mut(&mut self) -> &mut Self::Layout {
        return &mut self.layout;
    }
}

impl<S: ColorSpace, L: Layout, A: AlphaChannel, M: ChannelMap> Color for ColorType<S, L, A, M> {}
impl<S: ColorSpace, L: Layout, A: AlphaChannel, M: ChannelMap> ColorOps for ColorType<S, L, A, M> {}

impl<S: ColorSpace, L: Layout, M: ChannelMap, const N: usize> ColorAlpha for ColorType<S, L, AlphaSome<N>, M> {}

impl<L: Layout, A: AlphaChannel, M: ChannelMap> ColorOkLab for ColorType<OkLab, L, A, M> {}
impl<S: ColorSpaceRgb, L: Layout, A: AlphaChannel, M: ChannelMap> ColorRGB for ColorType<S, L, A, M> {}
