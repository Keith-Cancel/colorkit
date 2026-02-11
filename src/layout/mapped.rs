use core::marker::PhantomData;

use super::*;

/// This struct remaps the index access of the wrapped layout.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MappedLayout<M: LayoutMap, L: Layout<Channels = M::Channels>>(pub L, PhantomData<M>);

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> MappedLayout<M, L> {
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

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> Default for MappedLayout<M, L> {
    fn default() -> Self {
        return Self::new(L::DEFAULT);
    }
}

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> AsRef<L> for MappedLayout<M, L> {
    fn as_ref(&self) -> &L {
        return &self.0;
    }
}

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> AsMut<L> for MappedLayout<M, L> {
    fn as_mut(&mut self) -> &mut L {
        return &mut self.0;
    }
}

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> From<L> for MappedLayout<M, L> {
    fn from(value: L) -> Self {
        return Self(value, PhantomData);
    }
}

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> LayoutStorage for MappedLayout<M, L> {
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

impl<M: LayoutMap, L: Layout<Channels = M::Channels>> Layout for MappedLayout<M, L> {
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

impl<L: LayoutScalar<Channels = M::Channels>, M: LayoutMap> LayoutScalar for MappedLayout<M, L> {}

#[cfg(test)]
mod test {
    use colorkit::layout::maps::*;

    use super::*;

    #[test]
    fn check_get() {
        let p = <Planar3<u8>>::from_array([10, 11, 12]);
        // Should reverse order.
        let mut m = MappedLayout::<Map3<2, 1, 0>, _>::new(p);
        assert_eq!(m.get(0), 12);
        assert_eq!(m.get(1), 11);
        assert_eq!(m.get(2), 10);

        m.set(0, 22);
        m.set(1, 21);
        m.set(2, 20);
        assert_eq!(m.as_storage(), &[20, 21, 22]);

        let m = MappedLayout::<Map3<2, 0, 1>, _>::new(m.0);
        assert_eq!(m.get(0), 22);
        assert_eq!(m.get(1), 20);
        assert_eq!(m.get(2), 21);

        let m = <MappedLayout<Map3<2, 0, 1>, Planar3<u8>>>::from_fn(|i| ((i + 1) * 30) as u8);
        assert_eq!(m.get(0), 30);
        assert_eq!(m.get(1), 60);
        assert_eq!(m.get(2), 90);

        assert_eq!(m.as_storage(), &[60, 90, 30]);

        let p: Planar3<NormF32> = m.requantize(Rounding::Nearest);
        assert_eq!(p.get(0), 30.0 / 255.0);
        assert_eq!(p.get(1), 60.0 / 255.0);
        assert_eq!(p.get(2), 90.0 / 255.0);
    }
}
