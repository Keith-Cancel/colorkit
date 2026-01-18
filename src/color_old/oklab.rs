use super::Color;
use super::ColorFormat;
use super::OkLab;

pub trait ColorOkLab: ColorFormat<Space = OkLab> + Color {
    #[inline]
    fn l(&self) -> Self::Scalar {
        return self.get(0);
    }
    #[inline]
    fn a(&self) -> Self::Scalar {
        return self.get(1);
    }
    #[inline]
    fn b(&self) -> Self::Scalar {
        return self.get(2);
    }
    #[inline]
    fn set_l(&mut self, value: Self::Scalar) {
        return self.set(0, value);
    }
    #[inline]
    fn set_a(&mut self, value: Self::Scalar) {
        return self.set(1, value);
    }
    #[inline]
    fn set_b(&mut self, value: Self::Scalar) {
        return self.set(2, value);
    }
}
