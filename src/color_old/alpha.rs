use colorkit::layout::Layout;
use colorkit::scalar::IntoScalar;
use colorkit::scalar::Scalar;

use super::AlphaNone;
use super::ChannelMap;
use super::Color;
use super::ColorFormat;
use super::format::IsAlpha;

pub trait ColorAlpha: ColorFormat<Alpha: IsAlpha> + Color {
    fn alpha(&self) -> Self::Scalar {
        return self.as_layout().get(Self::Alpha::CHAN_IDX);
    }

    #[inline]
    fn alpha_index(&self) -> usize {
        return Self::Alpha::CHAN_IDX;
    }

    fn set_alpha(&mut self, value: Self::Scalar) {
        self.as_layout_mut().set(Self::Alpha::CHAN_IDX, value);
    }

    /// Remove the alpha channel from this color.
    ///
    /// Depending on the output colors layout the alpha data may still exist physically,
    /// but logically is no longer treated as alpha depending on the output layout.
    fn remove_alpha<C: Color + ColorFormat<Alpha = AlphaNone, Space = Self::Space>>(self) -> C {
        return C::from_fn(|phys_idx| {
            if let Some(&log_idx) = <C::Map as ChannelMap>::MAP_REVERSE.get(phys_idx) {
                return self.get(log_idx).into_scalar();
            }

            if phys_idx < <Self::Layout as Layout>::CHANNELS {
                return self.as_layout().get(phys_idx).into_scalar();
            }

            return <C::Scalar as Scalar>::DEFAULT;
        });
    }
}

#[cfg(test)]
mod test {
    use colorkit::layout::Planar4;
    use colorkit::scalar::NormF32;

    use super::super::*;
    use super::*;

    #[test]
    fn remove_alpha() {
        let lay = Planar4::from_array([51, 102, 153, 204]);
        let a = Srgba8::from_layout(lay);

        let b: Srgb8 = a.remove_alpha();
        assert_eq!(a.get(0), b.get(0));
        assert_eq!(a.get(1), b.get(1));
        assert_eq!(a.get(2), b.get(2));

        // Don't actually delete alpha, but just logically.
        let b: ColorType<Srgb, Planar4<NormF32>, AlphaNone, Map3> = a.remove_alpha();
        assert_eq!(0.2, b.get(0));
        assert_eq!(0.4, b.get(1));
        assert_eq!(0.6, b.get(2));
        assert_eq!(0.8, b.layout.get(3));
        assert!(b.try_alpha().is_none(), "Should have failed to get alpha!");

        let b: ColorType<Srgb, Planar4<NormF32>, AlphaNone, Map3<2, 0, 1>> = a.remove_alpha();
        assert_eq!(0.2, b.get(0));
        assert_eq!(0.4, b.get(1));
        assert_eq!(0.6, b.get(2));
        assert_eq!(0.8, b.layout.get(3));

        assert_eq!(0.2, b.layout.get(2));
        assert_eq!(0.4, b.layout.get(0));
        assert_eq!(0.6, b.layout.get(1));

        // Alpha channel should get overwritten
        let lay = Planar4::from_array([51, 204, 102, 153]);
        let a = <ColorType<Srgb, Planar4<u8>, AlphaSome<1>, Map3>>::from_layout(lay);
        assert_eq!(a.get(0), 51);
        assert_eq!(a.get(1), 102);
        assert_eq!(a.get(2), 153);
        assert_eq!(a.alpha(), 204);

        let b: ColorType<Srgb, Planar4<NormF32>, AlphaNone, Map3> = a.remove_alpha();

        assert_eq!(0.2, b.get(0));
        assert_eq!(0.4, b.get(1));
        assert_eq!(0.6, b.get(2));

        assert_eq!(0.2, b.layout.get(0));
        assert_eq!(0.4, b.layout.get(1));
        assert_eq!(0.6, b.layout.get(2));
        assert_eq!(0.6, b.layout.get(3));
    }
}
