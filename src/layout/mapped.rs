use core::marker::PhantomData;

use super::*;

/// This struct remaps the index access of the wrapped layout.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MappedLayout<L: Layout<Channels = M::Channels>, M: LayoutMap>(pub L, PhantomData<M>);

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> MappedLayout<L, M> {
    /// Create a new Mapped layout around an existing layout.
    pub const fn new(layout: L) -> Self {
        return Self(layout, PhantomData);
    }

    pub const fn as_mapped(layout: &L) -> &Self {
        let ptr = layout as *const _ as *const Self;
        // Safety: Mapped layout is transparent
        return unsafe { &*ptr };
    }

    pub const fn as_mut_mapped(layout: &mut L) -> &mut Self {
        let ptr = layout as *mut _ as *mut Self;
        // Safety: Mapped layout is transparent
        return unsafe { &mut *ptr };
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> Default for MappedLayout<L, M> {
    fn default() -> Self {
        return Self::new(L::DEFAULT);
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> AsRef<L> for MappedLayout<L, M> {
    fn as_ref(&self) -> &L {
        return &self.0;
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> AsMut<L> for MappedLayout<L, M> {
    fn as_mut(&mut self) -> &mut L {
        return &mut self.0;
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> From<L> for MappedLayout<L, M> {
    fn from(value: L) -> Self {
        return Self(value, PhantomData);
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> LayoutStorage for MappedLayout<L, M> {
    type Storage = L::Storage;

    fn into_storage(self) -> Self::Storage {
        return self.0.into_storage();
    }

    fn as_storage(&self) -> &Self::Storage {
        return self.0.as_storage();
    }

    fn as_mut_storage(&mut self) -> &mut Self::Storage {
        return self.0.as_mut_storage();
    }
}

impl<L: Layout<Channels = M::Channels>, M: LayoutMap> Layout for MappedLayout<L, M> {
    const DEFAULT: Self = Self(L::DEFAULT, PhantomData);
    type Channels = L::Channels;
    type ChannelType = L::ChannelType;

    fn from_fn_raw<F: FnMut(usize) -> Self::ChannelType>(mut fun: F) -> Self {
        let lay = L::from_fn_raw(|i| fun(M::unmap(i)));
        return Self::new(lay);
    }

    fn from_fn_norm<F: FnMut(usize) -> NormF32>(mut fun: F, round: Rounding) -> Self {
        let lay = L::from_fn_norm(|i| fun(M::unmap(i)), round);
        return Self::new(lay);
    }

    fn from_fn_norm_dither<F: FnMut(usize) -> NormF32, D: Dither>(mut fun: F, round: Rounding, dither: &mut D) -> Self {
        let lay = L::from_fn_norm_dither(|i| fun(M::unmap(i)), round, dither);
        return Self::new(lay);
    }

    fn get_norm(&self, index: usize) -> NormF32 {
        return self.0.get_norm(M::MAP[index]);
    }

    fn set_norm(&mut self, value: NormF32, index: usize, round: Rounding) {
        self.0.set_norm(value, M::MAP[index], round);
    }

    fn set_norm_dither<D: Dither>(&mut self, value: NormF32, index: usize, round: Rounding, dither: &mut D) {
        self.0.set_norm_dither(value, M::MAP[index], round, dither);
    }

    fn get_raw(&self, index: usize) -> Self::ChannelType {
        return self.0.get_raw(M::MAP[index]);
    }

    fn set_raw(&mut self, index: usize, value: Self::ChannelType) {
        self.0.set_raw(M::MAP[index], value);
    }
}

impl<L: LayoutScalar<Channels = M::Channels>, M: LayoutMap> LayoutScalar for MappedLayout<L, M> {}
