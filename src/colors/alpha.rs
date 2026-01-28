use colorkit::math::BoundF32;
use colorkit::space2::ColorData;
use colorkit::space2::ColorTransmute;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorTransmute>(S, f32);

base_funcs!(Alpha);

macro_rules! base_funcs {
    ($name:ident) => {
        impl<S: ColorTransmute> $name<S> {
            const LEN: usize = S::CHANNELS + 1;

            /// View alpha color as a slice reference.
            #[inline]
            pub const fn as_slice(&self) -> &[f32] {
                // Safety:
                // The ColorSpace S is marked as transmuteable
                // We also mark the alpha wrapper as repr(C)
                // If min_const_generic_args is stabilized this
                // can be replaced with safe constructs.
                let p = self as *const _ as *const f32;
                return unsafe { core::slice::from_raw_parts(p, Self::LEN) };
            }
            /// View alpha color as a mutable slice
            #[inline]
            pub const fn as_mut_slice(&mut self) -> &mut [f32] {
                let p = self as *mut _ as *mut f32;
                // Safety:
                // The ColorSpace S is marked as transmuteable
                // We also mark the alpha wrapper as repr(C)
                // If min_const_generic_args is stabilized this
                // can be replaced with safe constructs.
                return unsafe { core::slice::from_raw_parts_mut(p, Self::LEN) };
            }
        }

        impl<S: ColorTransmute> Default for $name<S> {
            fn default() -> Self {
                return Self(S::DEFAULT, 1.0);
            }
        }

        impl<S: ColorTransmute> AsRef<[f32]> for $name<S> {
            fn as_ref(&self) -> &[f32] {
                return self.as_slice();
            }
        }

        impl<S: ColorTransmute> AsMut<[f32]> for $name<S> {
            fn as_mut(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorTransmute> ColorData for $name<S> {
            type WhitePoint = S::WhitePoint;
            const DEFAULT: Self = Self(S::DEFAULT, 1.0);
            const LINEAR: bool = S::LINEAR;
            const CHANNEL_MAX: &'static [BoundF32] = { Self::MAX.split_at(Self::LEN).0 };
            const CHANNEL_MIN: &'static [BoundF32] = { Self::MIN.split_at(Self::LEN).0 };
        }

        impl<S: ColorTransmute> $name<S> {
            const MAX: &'static [BoundF32] = &const {
                // Just make this larger than likely needed can't use
                // S or Self in the len of an array =(
                let mut max = [BoundF32::Unbounded; 16];
                let mut i = 0;
                while i < S::CHANNEL_MAX.len() {
                    max[i] = S::CHANNEL_MAX[i];
                    i += 1;
                }
                max[i] = BoundF32::Include(1.0);
                max
            };
            const MIN: &'static [BoundF32] = &const {
                let mut arr = [BoundF32::Unbounded; 16];
                let mut i = 0;
                while i < S::CHANNEL_MIN.len() {
                    arr[i] = S::CHANNEL_MIN[i];
                    i += 1;
                }
                arr[i] = BoundF32::Include(0.0);
                arr
            };
        }
    };
}
pub(crate) use base_funcs;

#[cfg(test)]
mod test {
    use colorkit::colors::OkLab;
    use colorkit::colors::Srgb;
    use colorkit::colors::Xyz;
    use colorkit::space2::ColorData;
    use colorkit::wp::D65;

    use super::*;

    #[test]
    fn min_max() {
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Srgb>>::CHANNEL_MIN.len(), 4);

        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[0], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[1], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[2], BoundF32::Unbounded);
        assert_eq!(<Alpha<Xyz<D65>>>::CHANNEL_MAX[3], BoundF32::Include(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX.len(), 4);
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[0], BoundF32::Include(1.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[1], BoundF32::Include(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[2], BoundF32::Include(0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MAX[3], BoundF32::Include(1.0));

        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[0], BoundF32::Include(0.0));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[1], BoundF32::Include(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[2], BoundF32::Include(-0.5));
        assert_eq!(<Alpha<OkLab>>::CHANNEL_MIN[3], BoundF32::Include(0.0));
    }
}
