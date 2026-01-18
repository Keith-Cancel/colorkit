/// Defines a Type as a ColorSpace, and specifies how many channels it needs.
pub trait ColorSpace: Copy {
    /// Number channels in the color space
    const CHANNELS: usize;
}

/// Color spaces that are RGB like.
pub trait ColorSpaceRgb: ColorSpace {}

macro_rules! define_color_space {
    ($($name:ident: $len:expr),+ $(,)? ) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
            impl ColorSpace for $name {
                const CHANNELS: usize = $len;
            }

        )*
    };
}

define_color_space! {
    Srgb:    3,
    LinSrgb: 3,
    OkLab:   3,
}

impl ColorSpaceRgb for Srgb {}
impl ColorSpaceRgb for LinSrgb {}
