/// Implement ColorNew for $slf plus from_array and into_array on $slf directly.
macro_rules! impl_color_new {
    ($arr:ty, $slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* > $(,$ext:expr)?) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> colorkit::space::ColorNew for $slf<$($var),*> {
            #[inline]
            fn from_fn<F: FnMut(usize) -> f32>(f: F) -> Self {
                let a: $arr = core::array::from_fn(f);
                return Self(a $(, $ext)?);
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*>  $slf<$($var),*> {
            /// Create a new instance of the color from an array
            #[inline]
            pub const fn from_array(values: $arr) -> Self {
                return Self(values $(, $ext)?);
            }
            /// Convert an instance of the color to an array.
            #[inline]
            pub const fn into_array(self) -> $arr {
                return self.0;
            }
        }
    };
    ($arr:ty, $slf:ident $(,$ext:expr)?) => {
        impl_color_new!($arr, $slf<> $(,$ext)?);
    };
}
pub(crate) use impl_color_new;

/// Implement AsRef<$typ> for $slf and AsMut<$typ> for $slf
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
        impl_self_as_typ!($typ, $slf<>);
    };
}
pub(crate) use impl_self_as_typ;

/// Implement AsRef<$slf> for $typ and AsMut<$slf> for $typ
macro_rules! impl_typ_as_self {
    ($slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* >, $typ:ty) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> AsRef<$slf<$($var),*>> for $typ  {
            #[inline]
            fn as_ref(&self) -> &$slf<$($var),*> {
                let ptr = self as *const _ as *const $slf<$($var),*>;
                return unsafe { &*ptr };
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*> AsMut<$slf<$($var),*>> for $typ  {
            #[inline]
            fn as_mut(&mut self) -> &mut $slf<$($var),*> {
                let ptr = self as *mut _ as *mut $slf<$($var),*>;
                return unsafe { &mut *ptr };
            }
        }
    };
    ($slf:ident, $typ:ty) => {
        impl_typ_as_self!($slf<>, $typ);
    };
}
pub(crate) use impl_typ_as_self;

/// Implement From<$inner> for $slf and From<$slf> for $inner
macro_rules! impl_from_inner {
    ($inner:ty, $slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* > $(,$ext:expr)?) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> From<$inner> for $slf<$($var),*> {
            #[inline]
            fn from(value: $inner) -> Self {
                return Self(value $(, $ext)?);
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*> From<$slf<$($var),*>> for $inner {
            #[inline]
            fn from(value: $slf<$($var),*>) -> Self {
                return value.0;
            }
        }
    };
    ($inner:ty, $slf:ident $(,$ext:expr)?) => {
        impl_from_inner!($inner, $slf<> $(, $ext)?);
    };
}
pub(crate) use impl_from_inner;

/// Implement From<(f32, f32, f32)> for $slf and From<$slf> for (f32, f32, f32)
///
/// Tuple use repr(rust) so I just can't transmute.
macro_rules! impl_from_tup3 {
    ($slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* >) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> From<(f32, f32, f32)> for $slf<$($var),*> {
            #[inline]
            fn from(value: (f32, f32, f32)) -> Self {
                let a: [f32; 3] = value.into();
                return a.into();
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*> From<$slf<$($var),*>> for (f32, f32, f32) {
            #[inline]
            fn from(v: $slf<$($var),*>) -> Self {
                return (v.0[0], v.0[1], v.0[2]);
            }
        }
    };
    ($slf:ident) => {
        impl_from_tup3!($slf<>);
    };
}
pub(crate) use impl_from_tup3;

/// Implement Index<usize> for $self and IndexMut<usize> for $slf
macro_rules! impl_self_index {
    ($slf:ident < $( $var:ident $(: $bound:ident $(+$bound_n:ident)* )? ),* >) => {
        impl<$($var $(: $bound $(+$bound_n)*)?),*> core::ops::Index<usize> for $slf<$($var),*> {
            type Output = f32;
            #[inline]
            fn index(&self, index: usize) -> &f32 {
                return &<Self as AsRef<[f32]>>::as_ref(self)[index];
            }
        }

        impl<$($var $(: $bound $(+$bound_n)*)?),*> core::ops::IndexMut<usize> for $slf<$($var),*> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut f32 {
                return &mut <Self as AsMut<[f32]>>::as_mut(self)[index];
            }
        }
    };
    ($slf:ident) => {
        impl_self_index!($slf<>);
    };
}
pub(crate) use impl_self_index;
