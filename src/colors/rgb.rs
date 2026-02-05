use colorkit::convert::ColorTransmute;
use colorkit::convert::FromColor;
use colorkit::convert::XyzMatrices;
use colorkit::layout::Layout;
use colorkit::layout::LayoutMap;
use colorkit::math::BoundF32;
use colorkit::math::cbrtf;
use colorkit::math::quirtf;
use colorkit::math::sqrtf;
use colorkit::num_type::N3;
use colorkit::num_type::Number;
use colorkit::scalar::NormF32;
use colorkit::scalar::Rounding;
use colorkit::space::*;
use colorkit::wp::D65;

use super::Xyz;
use super::macros::*;

macro_rules! base_funcs {
    ($name:ident, $len:expr) => {
        impl $name {
            /// Create a new color from RGB values.
            #[inline]
            pub const fn new(r: f32, g: f32, b: f32) -> Self {
                return Self([r, g, b]);
            }
            /// Create a new color from rgb octets.
            #[inline]
            pub const fn new_u8(r: u8, g: u8, b: u8) -> Self {
                return Self([
                    r as f32 / 255.0,
                    g as f32 / 255.0,
                    b as f32 / 255.0,
                ]);
            }

            /// Get the red channel's value.
            #[inline]
            pub const fn red(&self) -> f32 {
                return self.0[0];
            }
            /// Get the blue channel's value.
            #[inline]
            pub const fn blue(&self) -> f32 {
                return self.0[1];
            }
            /// Get the green channel's value.
            #[inline]
            pub const fn green(&self) -> f32 {
                return self.0[2];
            }
            /// Set the red channel's value.
            #[inline]
            pub const fn set_red(&mut self, value: f32) {
                self.0[0] = value;
            }
            /// Set the blue channel's value.
            #[inline]
            pub const fn set_blue(&mut self, value: f32) {
                self.0[1] = value;
            }

            /// Set the green channel's value.
            #[inline]
            pub const fn set_green(&mut self, value: f32) {
                self.0[2] = value;
            }

            /// Clamp channel values between 0.0 and 1.0
            pub const fn clamp(self) -> Self {
                return Self([
                    f32::clamp(self.0[0], 0.0, 1.0),
                    f32::clamp(self.0[1], 0.0, 1.0),
                    f32::clamp(self.0[2], 0.0, 1.0),
                ]);
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                return Self([0.0, 0.0, 0.0]);
            }
        }

        impl ColorData for $name {
            type WhitePoint = D65;
            type Channels = N3;
            const DEFAULT: Self = Self([0.0, 0.0, 0.0]);
            const LINEAR: bool = true;
            const CHANNEL_MAX: &'static [BoundF32] = &[BoundF32::Include(1.0); 3];
            const CHANNEL_MIN: &'static [BoundF32] = &[BoundF32::Include(0.0); 3];
        }

        impl ColorLayout for $name {
            fn from_layout<L: Layout>(layout: L) -> Self {
                debug_assert!(<L::Channels as Number>::N >= 3);
                let r = layout.get_norm(0).get();
                let g = layout.get_norm(1).get();
                let b = layout.get_norm(2).get();
                return Self([r, g, b]);
            }

            fn from_layout_map<L: Layout, M: LayoutMap<Channels = L::Channels>>(layout: L) -> Self {
                debug_assert!(<L::Channels as Number>::N >= 3);
                let r = layout.get_norm(M::map(0)).get();
                let g = layout.get_norm(M::map(1)).get();
                let b = layout.get_norm(M::map(2)).get();
                return Self([r, g, b]);
            }

            fn into_layout<L: Layout>(self, round: Rounding) -> L {
                debug_assert!(<L::Channels as Number>::N == 3);
                return L::from_fn_norm(|i| NormF32::new(self.0[i]), round);
            }

            fn into_layout_map<L: Layout, M: LayoutMap>(self, round: Rounding) -> L {
                debug_assert!(<L::Channels as Number>::N == 3);
                return L::from_fn_norm(|i| NormF32::new(self.0[M::unmap(i)]), round);
            }

            fn into_layout_dither<L: Layout, D: crate::scalar::Dither>(self, round: Rounding, dither: &mut D) -> L {
                debug_assert!(<L::Channels as Number>::N == 3);
                return L::from_fn_norm_dither(|i| NormF32::new(self.0[i]), round, dither);
            }

            fn into_layout_dither_map<L: Layout, D: crate::scalar::Dither, M: LayoutMap>(
                self,
                round: Rounding,
                dither: &mut D,
            ) -> L {
                debug_assert!(<L::Channels as Number>::N == 3);
                return L::from_fn_norm_dither(|i| NormF32::new(self.0[M::unmap(i)]), round, dither);
            }
        }

        impl ColorMaybeAlpha for $name {
            type NoAlpha = Self;
            const ALPHA_KIND: AlphaKind = AlphaKind::None;
            const ALPHA_INDEX: Option<usize> = None;
            #[inline]
            fn opacity(&self) -> f32 {
                return 1.0;
            }
            #[inline]
            fn strip_alpha(self) -> Self::NoAlpha {
                return self;
            }
            #[inline]
            fn try_alpha_mut(&mut self) -> Option<&mut f32> {
                return None;
            }
            #[inline]
            fn try_alpha_ref(&self) -> Option<&f32> {
                return None;
            }
        }

        impl ColorSpace for $name {
            fn get_norm(&self, index: usize) -> NormF32 {
                return NormF32::new(self.0[index]);
            }
        }

        impl ColorSlice for $name {}
        unsafe impl ColorTransmute for $name {}
        impl RgbLike for $name {}
    };
}

/// Represention of an Srgb color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Srgb([f32; 3]);

base_funcs!(Srgb, 3);

impl_self_index!(Srgb);
impl_from_tup3!(Srgb);
impl_typ_as_self!(Srgb, [f32; 3]);
impl_self_as_typ!([f32], Srgb);
impl_self_as_typ!([f32; 3], Srgb);
impl_self_from_typ!([f32; 3], Srgb);

impl_color_array! {
    name: Srgb,
    channels: 3,
    extra_args: {},
    generics: {},
    gen_use: {}
}

impl Srgb {
    /// Convert Srgb into Linear Srgb
    pub const fn into_linear(self) -> LinSrgb {
        return LinSrgb([
            linear(self.0[0]),
            linear(self.0[1]),
            linear(self.0[2]),
        ]);
    }
}

impl FromColor<Xyz<D65>> for Srgb {
    fn from_color(color: Xyz<D65>) -> Self {
        return LinSrgb::from_color(color).into_nonlinear();
    }
}

impl FromColor<Srgb> for Xyz<D65> {
    fn from_color(color: Srgb) -> Self {
        return <Xyz<D65>>::from_color(color.into_linear());
    }
}

/// Represention of a Linear Srgb color using [`f32`] values.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinSrgb([f32; 3]);

base_funcs!(LinSrgb, 3);

impl_self_index!(LinSrgb);
impl_from_tup3!(LinSrgb);
impl_typ_as_self!(LinSrgb, [f32; 3]);
impl_self_as_typ!([f32], LinSrgb);
impl_self_as_typ!([f32; 3], LinSrgb);
impl_self_from_typ!([f32; 3], LinSrgb);

impl_color_array! {
    name: LinSrgb,
    channels: 3,
    extra_args: {},
    generics: {},
    gen_use: {}
}

impl LinSrgb {
    /// Convert Linear Srgb into Srgb
    pub fn into_nonlinear(self) -> Srgb {
        return Srgb([
            nonlinear(self.0[0]),
            nonlinear(self.0[1]),
            nonlinear(self.0[2]),
        ]);
    }
}

impl XyzMatrices for LinSrgb {
    #[rustfmt::skip]
    const INTO_XYZ: [f32; 9] = [
        0.4124574455823671, 0.3575758652455160, 0.1804372478263999,
        0.2126733703784081, 0.7151517304910320, 0.0721748991305599,
        0.0193339427616735, 0.1191919550818387, 0.9503028385523726,
    ];
    #[rustfmt::skip]
    const FROM_XYZ: [f32; 9] = [
         3.2404462546477404, -1.5371347618200820, -0.4985301930227293,
        -0.9692666062446794,  1.8760119597883695,  0.0415560422144301,
         0.0556435035643528, -0.2040261797359601,  1.0572265677227023
    ];
}

fn nonlinear(l: f32) -> f32 {
    // 0.0031308 old
    let s = if l <= 0.00313066844250063 {
        l * 12.92
    } else {
        let sq = sqrtf(l);
        let cb = cbrtf(l);
        let c = sqrtf(sq) * sqrtf(cb);

        1.055 * c - 0.055
    };
    return s;
}

// https://entropymine.com/imageworsener/srgbformula/
const fn linear(s: f32) -> f32 {
    // 0.04045 old
    let l = if s <= 0.0404482362771082 {
        s / 12.92
    } else {
        let x = (s + 0.055) / 1.055;
        // Equals x.powf(2.4)
        let x2 = x * x;
        x2 * quirtf(x2)
    };
    return l;
}

impl FromColor<Srgb> for LinSrgb {
    fn from_color(value: Srgb) -> Self {
        return value.into_linear();
    }
}

impl FromColor<LinSrgb> for Srgb {
    fn from_color(value: LinSrgb) -> Self {
        return value.into_nonlinear();
    }
}

#[cfg(test)]
mod test {
    use colorkit::math::MathFuncs;
    use colorkit::wp::WhitePoint;

    use super::*;
    #[test]
    fn linear_convert() {
        let c = Srgb::new(0.34117647058, 0.89019607843, 0.53725490196);
        let c = c.into_linear();
        assert!(c[0] >= 0.0953074);
        assert!(c[0] <= 0.0953075);

        // These values have an exact representation so should not be lost.
        let c = Srgb::new(0.5, 0.75, 0.125);
        let c = c.into_linear();
        assert!(c[0] >= 0.214 && c[0] <= 0.2141);
        let c = c.into_nonlinear();
        assert_eq!(c[0], 0.5);
        assert_eq!(c[1], 0.75);
        assert_eq!(c[2], 0.125);
    }

    #[test]
    fn xyz_convert() {
        let c = Srgb::new(1.0, 1.0, 1.0);
        let x = c.into_xyz();
        assert!(x[0].almost_eq(D65::X, 2e-7));
        assert!(x[1].almost_eq(D65::Y, 2e-7));
        assert!(x[2].almost_eq(D65::Z, 2e-7));

        let c = Srgb::from_xyz(D65::color());
        assert!(c[0].almost_eq(1.0, 2e-7));
        assert!(c[1].almost_eq(1.0, 2e-7));
        assert!(c[2].almost_eq(1.0, 2e-7));

        let c = Srgb::new(0.0, 0.0, 0.0);
        let x = <Xyz<D65>>::from_color(c);
        assert_eq!(x[0], 0.0);
        assert_eq!(x[1], 0.0);
        assert_eq!(x[2], 0.0);
    }
}
