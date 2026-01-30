use colorkit::math::BoundF32;
use colorkit::space2::ColorArray;
use colorkit::space2::ColorData;
use colorkit::space2::ColorSpace;
use colorkit::space2::ColorTransmute;
use colorkit::space2::XyzConvert;

use super::Xyz;

/// Wraps a color space with Alpha channel for transparency.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Alpha<S: ColorTransmute>(S, f32);

impl<S: ColorTransmute> Alpha<S> {
    /// Set the colors alpha channel value.
    pub const fn set_alpha(&mut self, alpha: f32) {
        self.1 = alpha;
    }
    /// View [`Alpha`] as a reference to the underylying colorspace.
    #[inline]
    pub const fn as_color(&self) -> &S {
        return &self.0;
    }
    /// View [`Alpha`] as a mutable reference to the underylying colorspace.
    #[inline]
    pub const fn as_mut_color(&mut self) -> &mut S {
        return &mut self.0;
    }
}

base_funcs!(Alpha);

impl<S: ColorTransmute> AsRef<S> for Alpha<S> {
    fn as_ref(&self) -> &S {
        return &self.0;
    }
}

impl<S: ColorTransmute> AsMut<S> for Alpha<S> {
    fn as_mut(&mut self) -> &mut S {
        return &mut self.0;
    }
}

macro_rules! base_funcs {
    ($name:ident) => {
        impl<S: ColorTransmute> $name<S> {
            const LEN: usize = S::CHANNELS + 1;
            /// Get the colors alpha channel value.
            pub const fn alpha(&self) -> f32 {
                return self.1;
            }
            /// View the alpha color as a slice reference.
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
            /// View the alpha color as a mutable slice
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

        impl<S: ColorTransmute> AsRef<[f32]> for $name<S> {
            #[inline]
            fn as_ref(&self) -> &[f32] {
                return self.as_slice();
            }
        }

        impl<S: ColorTransmute> AsMut<[f32]> for $name<S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorTransmute> core::borrow::Borrow<[f32]> for $name<S> {
            #[inline]
            fn borrow(&self) -> &[f32] {
                return self.as_slice();
            }
        }

        impl<S: ColorTransmute> core::borrow::BorrowMut<[f32]> for $name<S> {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorTransmute> Default for $name<S> {
            fn default() -> Self {
                return Self(S::DEFAULT, 1.0);
            }
        }

        impl<S: ColorTransmute> core::ops::Index<usize> for $name<S> {
            type Output = f32;
            #[inline]
            fn index(&self, index: usize) -> &f32 {
                return &self.as_slice()[index];
            }
        }

        impl<S: ColorTransmute> core::ops::IndexMut<usize> for $name<S> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut f32 {
                return &mut self.as_mut_slice()[index];
            }
        }

        impl<S: ColorTransmute> ColorData for $name<S> {
            type WhitePoint = S::WhitePoint;
            const DEFAULT: Self = Self(S::DEFAULT, 1.0);
            const LINEAR: bool = S::LINEAR;
            const CHANNEL_MAX: &'static [BoundF32] = { Self::MAX.split_at(Self::LEN).0 };
            const CHANNEL_MIN: &'static [BoundF32] = { Self::MIN.split_at(Self::LEN).0 };
        }

        impl<S: ColorTransmute> ColorArray for $name<S> {
            const CHANNELS: usize = Self::LEN;
            fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self {
                let mut f = f;
                return Self(S::from_fn(|i| f(i)), f(S::CHANNELS));
            }
            #[inline]
            fn as_slice(&self) -> &[f32] {
                return self.as_slice();
            }
            #[inline]
            fn as_mut_slice(&mut self) -> &mut [f32] {
                return self.as_mut_slice();
            }
        }

        impl<S: ColorTransmute> XyzConvert for $name<S> {
            fn into_xyz(self) -> Xyz<Self::WhitePoint> {
                return self.0.into_xyz();
            }
            fn from_xyz(color: Xyz<Self::WhitePoint>) -> Self {
                return Self(S::from_xyz(color), 1.0);
            }
        }

        impl<S: ColorTransmute> ColorSpace for $name<S> {}

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

    #[test]
    fn from_fn() {
        let arr = [0.125f32, 0.25, 0.375, 0.5];
        let c = <Alpha<Srgb>>::from_fn(|i| arr[i]);
        let r = &c;

        assert_eq!(c[0], 0.125);
        assert_eq!(c[1], 0.25);
        assert_eq!(c[2], 0.375);
        assert_eq!(c[3], 0.5);

        assert_eq!(r.as_color().red(), 0.125);
    }
}
