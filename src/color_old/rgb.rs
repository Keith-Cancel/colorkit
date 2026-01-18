use super::Color;
use super::ColorFormat;
use super::ColorSpaceRgb;

pub trait ColorRGB: ColorFormat<Space: ColorSpaceRgb> + Color {
    #[inline]
    fn red(&self) -> Self::Scalar {
        return self.get(0);
    }

    #[inline]
    fn green(&self) -> Self::Scalar {
        return self.get(1);
    }

    #[inline]
    fn blue(&self) -> Self::Scalar {
        return self.get(2);
    }

    #[inline]
    fn set_red(&mut self, value: Self::Scalar) {
        self.set(0, value);
    }

    #[inline]
    fn set_green(&mut self, value: Self::Scalar) {
        self.set(1, value);
    }

    #[inline]
    fn set_blue(&mut self, value: Self::Scalar) {
        self.set(2, value);
    }
}
