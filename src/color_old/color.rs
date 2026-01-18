use colorkit::layout::Layout;

use super::AlphaChannel;
use super::AlphaNone;
use super::ColorFormat;
use crate::color::ColorSpace;
use crate::scalar::IntoScalar;

pub trait Color: ColorFormat {
    /// Get the value of a color channel.
    #[inline]
    fn get(&self, index: usize) -> Self::Scalar {
        let i = Self::layout_index(index);
        return self.as_layout().get(i);
    }
    /// Set the value of a color channel.
    #[inline]
    fn set(&mut self, index: usize, value: Self::Scalar) {
        let i = Self::layout_index(index);
        self.as_layout_mut().set(i, value);
    }

    /// Try to read the alpha channel.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha(&self) -> Option<Self::Scalar> {
        let i = <Self::Alpha as AlphaChannel>::CHANNEL?;
        return Some(self.as_layout().get(i));
    }

    /// Try to get the index of the alpha channel in the layout.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    #[inline]
    fn try_alpha_index(&self) -> Option<usize> {
        return <Self::Alpha as AlphaChannel>::CHANNEL;
    }

    /// Try to set the alpha channel.
    ///
    /// Returns `Err(AlphaNone)` if this color has no alpha channel.
    fn try_set_alpha(&mut self, value: Self::Scalar) -> Result<(), AlphaNone> {
        let Some(i) = <Self::Alpha as AlphaChannel>::CHANNEL else {
            return Err(AlphaNone);
        };
        self.as_layout_mut().set(i, value);
        return Ok(());
    }
    /// Converts a [`Color`] to a color of the to new format in the same color space.
    /// The can change the ['Layout'](colorkit::layout::Layout), [`Scalar`], and or
    /// [`ChannelMap`](`colorkit::color::ChannelMap`)
    ///
    /// Essentially, only the [`ColorSpace`] and [`AlphaChannel`] need be the same.
    fn convert_format<C: Color<Space = Self::Space, Alpha = Self::Alpha>>(self) -> C {
        let mut ret = C::default();
        let mut i = 0usize;
        while i < <Self::Space as ColorSpace>::CHANNELS {
            ret.set(i, self.get(i).into_scalar());
            i += 1;
        }
        if let Some(a) = self.try_alpha() {
            let _ = ret.try_set_alpha(a.into_scalar());
        }
        return ret;
    }
}
