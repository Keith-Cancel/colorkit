macro_rules! color_inner {
    ($num:ident) => {
        impl ColorInner<$num> {
            /// Creates a color by filling all channels with the given value.
            #[inline]
            pub const fn repeat(value: f32) -> Self {
                return Self($num::repeat_copy(value));
            }
        }
    };
}
pub(crate) use color_inner;

// All this just for a const fn -_-
#[allow(unused)]
macro_rules! color_fn_ref {
    (mut: $self:expr, $method:ident ( $($arg:expr),* $(,)? )) => {
        color_fn_ref!(@inner $self, $method ($($arg),*), mut, mut);
    };
    (ref: $self:expr, $method:ident ( $($arg:expr),* $(,)? )) => {
        color_fn_ref!(@inner $self, $method ($($arg),*), const);
    };

    (@inner $self:expr, $method:ident ( $($arg:expr),* $(,)? ), $ptr:ident $(,$ref:ident)?) => {
        // Safety:
        // Number is a sealed trait so no other types can have these values.
        // Color is also a transparent type.
        match <S::Channels as Number>::N {
            0 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N0> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            1 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N1> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            2 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N2> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            3 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N3> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            4 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N4> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            5 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N5> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            6 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N6> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            7 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N7> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            8 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N8> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            9 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N9> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            10 => {
                let p = ( ($self) as *$ptr _ as *$ptr ColorInner<N10> );
                let r =  unsafe { &$($ref)? *p };
                return r.$method($($arg),*);
            }
            _ => panic!("Unhandled number type"),
        }
    }

}
#[allow(unused)]
pub(crate) use color_fn_ref;

macro_rules! color_fn_new {
    ($method:ident ( $($arg:expr),* $(,)? )) => {
        // Safety:
        // Number is a sealed trait so no other types can have these values.
        // Color is also a transparent type.
        match <S::Channels as Number>::N {
            0 => {
                let c = ColorInner::<N0>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            1 => {
                let c = ColorInner::<N1>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            2 => {
                let c = ColorInner::<N2>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            3 => {
                let c = ColorInner::<N3>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            4 => {
                let c = ColorInner::<N4>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            5 => {
                let c = ColorInner::<N5>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            6 => {
                let c = ColorInner::<N6>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            7 => {
                let c = ColorInner::<N7>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            8 => {
                let c = ColorInner::<N8>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            9 => {
                let c = ColorInner::<N9>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            10 => {
                let c = ColorInner::<N10>::$method($($arg),*);
                return unsafe { c.transmute() };
            }
            _ => panic!("Unhandled number type"),
        }
    };
}
pub(crate) use color_fn_new;
