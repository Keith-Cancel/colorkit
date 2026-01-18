#[rustfmt::skip]
mod no_fmt {
    use colorkit::color::*;
    use colorkit::layout::Planar3;
    use colorkit::layout::Planar4;
    use colorkit::layout::Packed565;
    use colorkit::scalar::NormF32;

    pub type Srgb565     = ColorType<Srgb,    Packed565>;
    pub type Srgb8       = ColorType<Srgb,    Planar3<u8>>;
    pub type Srgb16      = ColorType<Srgb,    Planar3<u16>>;
    pub type SrgbF32     = ColorType<Srgb,    Planar3<NormF32>>;
    pub type Srgba8      = ColorType<Srgb,    Planar4<u8>,      AlphaSome>;
    pub type Srgba16     = ColorType<Srgb,    Planar4<u16>,     AlphaSome>;
    pub type SrgbaF32    = ColorType<Srgb,    Planar4<NormF32>, AlphaSome>;
    pub type LinSrgb8    = ColorType<LinSrgb, Planar3<u8>>;
    pub type LinSrgb16   = ColorType<LinSrgb, Planar3<u16>>;
    pub type LinSrgbF32  = ColorType<LinSrgb, Planar3<NormF32>>;
    pub type LinSrgba8   = ColorType<LinSrgb, Planar4<u8>,      AlphaSome>;
    pub type LinSrgba16  = ColorType<LinSrgb, Planar4<u16>,     AlphaSome>;
    pub type LinSrgbaF32 = ColorType<LinSrgb, Planar4<NormF32>, AlphaSome>;
    pub type OkLab8      = ColorType<OkLab,   Planar3<u8>>;
    pub type OkLab16     = ColorType<OkLab,   Planar3<u16>>;
    pub type OkLabF32    = ColorType<OkLab,   Planar3<NormF32>>;
    pub type OkLaba8     = ColorType<OkLab,   Planar4<u8>,      AlphaSome>;
    pub type OkLaba16    = ColorType<OkLab,   Planar4<u16>,     AlphaSome>;
    pub type OkLabaF32   = ColorType<OkLab,   Planar4<NormF32>, AlphaSome>;
}

pub use no_fmt::*;
