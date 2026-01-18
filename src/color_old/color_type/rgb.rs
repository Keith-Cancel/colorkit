use colorkit::color::AlphaNone;
use colorkit::color::AlphaSome;
use colorkit::color::ChannelMap;
use colorkit::color::ColorAlpha;
use colorkit::color::ColorRGB;
use colorkit::color::ColorSpaceRgb;
use colorkit::layout::Layout;
use colorkit::scalar::NormF32;
use colorkit::scalar::NotNormalized;
use colorkit::scalar::Scalar;

use super::ColorType;

impl<C: ColorSpaceRgb, L: Layout, O: ChannelMap> ColorType<C, L, AlphaNone, O> {
    /// Create a new RGB color.
    pub fn new(r: L::Scalar, g: L::Scalar, b: L::Scalar) -> Self {
        let mut c = Self::default();
        c.set_red(r);
        c.set_green(g);
        c.set_blue(b);
        return c;
    }

    pub fn new_f32(r: f32, g: f32, b: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new(NormF32::new(r)?, NormF32::new(g)?, NormF32::new(b)?));
    }
}

impl<C: ColorSpaceRgb, L: Layout, O: ChannelMap, const N: usize> ColorType<C, L, AlphaSome<N>, O> {
    /// Create a new RGBA color.
    pub fn new(r: L::Scalar, g: L::Scalar, b: L::Scalar, a: L::Scalar) -> Self {
        let mut c = Self::default();
        c.set_red(r);
        c.set_green(g);
        c.set_blue(b);
        c.set_alpha(a);
        return c;
    }

    pub fn new_f32(r: f32, g: f32, b: f32, a: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new(
            NormF32::new(r)?,
            NormF32::new(g)?,
            NormF32::new(b)?,
            NormF32::new(a)?,
        ));
    }

    /// Create a new color RGBA color from only RGB values.
    pub fn new_rgb(r: L::Scalar, g: L::Scalar, b: L::Scalar) -> Self {
        return Self::new(r, g, b, L::Scalar::SCALAR_MAX);
    }

    pub fn new_rgb_f32(r: f32, g: f32, b: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new_rgb(NormF32::new(r)?, NormF32::new(g)?, NormF32::new(b)?));
    }
}
