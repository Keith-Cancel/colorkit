use colorkit::color::AlphaNone;
use colorkit::color::AlphaSome;
use colorkit::color::ChannelMap;
use colorkit::color::ColorAlpha;
use colorkit::color::ColorOkLab;
use colorkit::color::OkLab;
use colorkit::layout::Layout;
use colorkit::scalar::NormF32;
use colorkit::scalar::NotNormalized;
use colorkit::scalar::Scalar;

use super::ColorType;

impl<L: Layout, O: ChannelMap> ColorType<OkLab, L, AlphaNone, O> {
    /// Create a new OkLAB color.
    pub fn new(l: L::Scalar, a: L::Scalar, b: L::Scalar) -> Self {
        let mut c = Self::default();
        c.set_l(l);
        c.set_a(a);
        c.set_b(b);
        return c;
    }

    pub fn new_f32(l: f32, a: f32, b: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new(NormF32::new(l)?, NormF32::new(a)?, NormF32::new(b)?));
    }
}

impl<L: Layout, O: ChannelMap, const N: usize> ColorType<OkLab, L, AlphaSome<N>, O> {
    /// Create a new OkLAB Alpha color.
    pub fn new(l: L::Scalar, a: L::Scalar, b: L::Scalar, alpha: L::Scalar) -> Self {
        let mut c = Self::default();
        c.set_l(l);
        c.set_a(a);
        c.set_b(b);
        c.set_alpha(alpha);
        return c;
    }

    pub fn new_f32(l: f32, a: f32, b: f32, alpha: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new(
            NormF32::new(l)?,
            NormF32::new(a)?,
            NormF32::new(b)?,
            NormF32::new(alpha)?,
        ));
    }

    /// Create a new color OkLAB Alpha color from only LAB values.
    pub fn new_lab(l: L::Scalar, a: L::Scalar, b: L::Scalar) -> Self {
        return Self::new(l, a, b, L::Scalar::SCALAR_MAX);
    }

    pub fn new_lab_f32(l: f32, a: f32, b: f32) -> Result<Self, NotNormalized>
    where
        L: Layout<Scalar = NormF32>,
    {
        return Ok(Self::new_lab(NormF32::new(l)?, NormF32::new(a)?, NormF32::new(b)?));
    }
}
