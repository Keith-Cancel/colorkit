use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::Index;
use std::ops::IndexMut;

pub trait ArraySealed {}

pub trait ArrayItem:
    Index<usize, Output = Self::Item>
    + IndexMut<usize, Output = Self::Item>
    + AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Borrow<[Self::Item]>
    + BorrowMut<[Self::Item]>
{
    type Item;
}

impl<T, const N: usize> ArrayItem for [T; N] {
    type Item = T;
}

pub trait ArrayTruncate: ArrayItem + ArraySealed {
    type Shorter: ArrayBounded;
    fn truncate(self) -> Self::Shorter;
}

pub trait ArrayExtend: ArrayItem + ArraySealed {
    type Longer: ArrayBounded;
    fn extend_with(self, value: Self::Item) -> Self::Longer;
}

macro_rules! trunc_ext {
    // Entry point
    ($head:ident, $($tail:ident),* $(,)?) => {
        impl <T> ArraySealed for [T; const { $(trunc_ext!(@one $tail) +)* 2 }] {}
        impl <T> ArrayTruncate for [T; const { $(trunc_ext!(@one $tail) +)* 2 }] {
            type Shorter = [T; const { $(trunc_ext!(@one $tail) +)* 1 }];
            #[inline]
            fn truncate(self,) -> Self::Shorter {
                let [$head, $($tail,)* _] = self;
                return [$head, $($tail),*];
            }
        }
        impl <T> ArrayExtend for [T; const { $(trunc_ext!(@one $tail) +)* 2 }] {
            type Longer = [T; 0];
            #[inline]
            fn extend_with(self, _: Self::Item) -> Self::Longer {
                panic!("extend_with() not implented for {}", core::any::type_name::<Self>());
            }
        }
        trunc_ext!(@inner $head, $($tail,)* );

    };
    (@inner $head:ident, $($tail:ident),* $(,)?) => {
        impl <T> ArraySealed for [T; const { $(trunc_ext!(@one $tail) +)* 1 }] {}
        impl <T> ArrayTruncate for [T; const {$(trunc_ext!(@one $tail) +)* 1 }] {
            type Shorter = [T; const { $(trunc_ext!(@one $tail) +)* 0 }];
            #[inline]
            fn truncate(self) -> Self::Shorter {
                let [$($tail,)* _] = self;
                return [$($tail),*];
            }
        }

        impl <T> ArrayExtend for [T; const { $(trunc_ext!(@one $tail) +)* 1 }] {
            type Longer = [T; const { $(trunc_ext!(@one $tail) +)* 2 }];
            #[inline]
            fn extend_with(self, value: Self::Item) -> Self::Longer {
                let [$head, $($tail),*] = self;
                return [$head, $($tail,)* value];
            }
        }
        trunc_ext!(@inner $($tail,)* );

    };
    (@inner) => {
        impl <T> ArraySealed for [T; 0] {}
        impl <T> ArrayTruncate for [T; 0] {
            type Shorter = [T; 0];
            #[inline]
            fn truncate(self) -> [T; 0] {
                return [];
            }
        }

        impl <T> ArrayExtend for [T; 0] {
            type Longer = [T; 1];
            #[inline]
            fn extend_with(self, value: Self::Item) -> [T; 1] {
                return [value];
            }
        }

    };
    (@one $any:tt) => { 1 }
}

trunc_ext!(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);

pub trait ArrayBounded: ArrayExtend + ArrayTruncate {
    fn as_slice(&self) -> &[Self::Item];
    fn as_mut_slice(&mut self) -> &mut [Self::Item];
    fn len(&self) -> usize;
}

impl<T, const N: usize> ArrayBounded for [T; N]
where
    Self: ArrayExtend + ArrayTruncate + ArrayItem<Item = T>
{
    #[inline]
    fn as_slice(&self) -> &[T] {
        return self;
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        return self;
    }

    #[inline]
    fn len(&self) -> usize {
        return N;
    }
}
