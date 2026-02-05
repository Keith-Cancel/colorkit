macro_rules! impl_color_array {
    (name: $name:ident, channels: $len:expr, extra_args: { $($args:ident),* }, generics: { $($generics:tt)* }, gen_use: { $($gen_use:tt)*} ) => {
        impl $($generics)* AsRef<[f32]> for $name $($gen_use)* {
            #[inline]
            fn as_ref(&self) -> &[f32] {
                return &self.0;
            }
        }

        impl $($generics)* AsMut<[f32]> for $name $($gen_use)* {
            #[inline]
            fn as_mut(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl $($generics)* core::borrow::Borrow<[f32]> for $name $($gen_use)* {
            #[inline]
            fn borrow(&self) -> &[f32] {
                return &self.0;
            }
        }

        impl $($generics)* core::borrow::BorrowMut<[f32]> for $name $($gen_use)* {
            #[inline]
            fn borrow_mut(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl $($generics)* core::ops::Index<usize> for $name $($gen_use)* {
            type Output = f32;
            #[inline]
            fn index(&self, index: usize) -> &f32 {
                return &self.0[index];
            }
        }

        impl $($generics)* core::ops::IndexMut<usize> for $name $($gen_use)* {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut f32 {
                return &mut self.0[index];
            }
        }

        impl $($generics)* colorkit::space::ColorArray for $name $($gen_use)* {
            fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self {
                return Self(core::array::from_fn(f), $($args),*);
            }
            #[inline]
            fn as_slice(&self) -> &[f32] {
                return &self.0;
            }
            #[inline]
            fn as_mut_slice(&mut self) -> &mut [f32] {
                return &mut self.0;
            }
        }

        impl $($generics)* core::convert::From<[f32; $len]> for $name $($gen_use)* {
            fn from(values: [f32; $len]) -> Self {
                return Self(values, $($args),*);
            }
        }

        impl $($generics)* core::convert::From<$name $($gen_use)*> for [f32; $len] {
            fn from(values: $name $($gen_use)*) -> Self {
                return values.0;
            }
        }

        impl $($generics)* $name $($gen_use)* {
            /// Create a new instance of the color from an array
            pub const fn from_array(values: [f32; $len]) -> Self {
                return Self(values, $($args),*);
            }
            /// Convert an instance of the color to an array.
            pub const fn into_array(self) -> [f32; $len] {
                return self.0;
            }
        }
    };
}
pub(crate) use impl_color_array;

/// Implenment AsRef<$typ> for $slf and AsMut<$typ> for $slf
macro_rules! impl_self_as_typ {
    ($typ:ty, $slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* >) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> AsRef<$typ> for $slf<$($var),*> {
            #[inline]
            fn as_ref(&self) -> &$typ {
                return &self.0;
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*> AsMut<$typ> for $slf<$($var),*> {
            #[inline]
            fn as_mut(&mut self) -> &mut $typ {
                return &mut self.0;
            }
        }
    };
    ($typ:ty, $slf:ident) => {
        impl_as_ref_mut!($typ, $slf<>);
    };
}
pub(crate) use impl_self_as_typ;
