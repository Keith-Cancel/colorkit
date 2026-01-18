/// Types should be copy and have the base Op implemented
macro_rules! op_binary_ref {
    ($lhs:ident, $rhs:ident, $(::)? $trait:ident :: $method:ident $(,)?) => {
        impl $trait<$rhs> for &$lhs {
            // Keep same as base method
            type Output = <$lhs as $trait<$rhs>>::Output;
            #[inline]
            fn $method(self, rhs: $rhs) -> Self::Output {
                return $lhs::$method(*self, rhs);
            }
        }

        impl $trait<&$rhs> for $lhs {
            type Output = <$lhs as $trait<$rhs>>::Output;
            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                return $lhs::$method(self, *rhs);
            }
        }

        impl $trait<&$rhs> for &$lhs {
            type Output = <$lhs as $trait<$rhs>>::Output;
            #[inline]
            fn $method(self, rhs: &$rhs) -> Self::Output {
                return $lhs::$method(*self, *rhs);
            }
        }
    };
}
pub(crate) use op_binary_ref;

/// Types should be copy and have the base Op implemented
macro_rules! op_assign_ref {
    ($lhs:ident, $rhs:ident, $(::)? $trait:ident :: $method:ident $(,)?) => {
        impl $trait<&$rhs> for $lhs {
            #[inline]
            fn $method(&mut self, rhs: &$rhs) {
                return $lhs::$method(self, *rhs);
            }
        }
    };
}
pub(crate) use op_assign_ref;

macro_rules! op_binary_wrapped {
    ($name:ident, $inner:ident, $(::)? $trait:ident :: $method:ident $(,)?) => {
        impl $trait for $name {
            type Output = $inner;
            #[inline]
            fn $method(self, rhs: $name) -> Self::Output {
                return $inner::$method(Into::<$inner>::into(self), Into::<$inner>::into(rhs));
            }
        }

        impl $trait<$inner> for $name {
            type Output = $inner;
            #[inline]
            fn $method(self, rhs: $inner) -> Self::Output {
                return $inner::$method(Into::<$inner>::into(self), Into::<$inner>::into(rhs));
            }
        }

        impl $trait<$name> for $inner {
            type Output = $inner;
            #[inline]
            fn $method(self, rhs: $name) -> Self::Output {
                return $inner::$method(Into::<$inner>::into(self), Into::<$inner>::into(rhs));
            }
        }
    };
}
pub(crate) use op_binary_wrapped;

macro_rules! op_assign_wrapped {
    ($name:ident, $inner:ident, $(::)? $trait:ident :: $method:ident $(,)?) => {
        impl $trait<$name> for $inner {
            #[inline]
            fn $method(&mut self, rhs: $name) {
                return $inner::$method(self, Into::<$inner>::into(rhs));
            }
        }
    };
}
pub(crate) use op_assign_wrapped;
