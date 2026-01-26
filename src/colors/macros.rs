macro_rules! impl_color_array {
    ($name:ident, $len:expr) => {
        impl AsRef<[f32]> for $name {
            fn as_ref(&self) -> &[f32] {
                return &self.0;
            }
        }

        impl AsMut<[f32]> for $name {
            fn as_mut(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl core::borrow::Borrow<[f32]> for $name {
            fn borrow(&self) -> &[f32] {
                return &self.0;
            }
        }

        impl core::borrow::BorrowMut<[f32]> for $name {
            fn borrow_mut(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = f32;
            fn index(&self, index: usize) -> &f32 {
                return &self.0[index];
            }
        }

        impl core::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut f32 {
                return &mut self.0[index];
            }
        }

        impl colorkit::space2::ColorArray for $name {
            const CHANNELS: usize = $len;

            fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self {
                return Self(core::array::from_fn(f));
            }

            fn as_slice(&self) -> &[f32] {
                return &self.0;
            }

            fn as_mut_slice(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl core::convert::From<[f32; $len]> for $name {
            fn from(values: [f32; $len]) -> Self {
                return Self(values);
            }
        }

        impl core::convert::From<$name> for [f32; $len] {
            fn from(values: $name) -> Self {
                return values.0;
            }
        }

        impl $name {
            /// Create a new instance of the color from an array
            pub const fn from_array(values: [f32; $len]) -> Self {
                return Self(values);
            }
            /// Convert an instance of the color to an array.
            pub const fn into_array(self) -> [f32; $len] {
                return self.0;
            }
        }
    };
}
pub(crate) use impl_color_array;
