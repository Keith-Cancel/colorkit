/// Specifies the order the color channels in a color.
/// Basically remaps the indexs used for `set()` and `get()`
pub trait ChannelMap: Copy {
    const MAP: &'static [usize];
    const MAP_REVERSE: &'static [usize];
}

/// Marker type specifying the order of color channels
#[derive(Clone, Copy, Debug)]
pub struct Map3<const A: usize = 0, const B: usize = 1, const C: usize = 2>;

impl<const A: usize, const B: usize, const C: usize> ChannelMap for Map3<A, B, C> {
    const MAP: &'static [usize] = &[A, B, C];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 3];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev
    };
}

/// Marker type specifying the order of color channels
#[derive(Clone, Copy, Debug)]
pub struct Map4<const A: usize = 0, const B: usize = 1, const C: usize = 2, const D: usize = 3>;

impl<const A: usize, const B: usize, const C: usize, const D: usize> ChannelMap for Map4<A, B, C, D> {
    const MAP: &'static [usize] = &[A, B, C, D];
    const MAP_REVERSE: &'static [usize] = &const {
        let mut rev = [0usize; 4];
        rev[A] = 0;
        rev[B] = 1;
        rev[C] = 2;
        rev[D] = 3;
        rev
    };
}

#[cfg(test)]
mod test {
    use super::ChannelMap;
    use super::Map3;
    use super::Map4;
    #[test]
    fn resverse() {
        assert_eq!(Map3::<1, 2, 0>::MAP[0], 1);
        assert_eq!(Map3::<1, 2, 0>::MAP[1], 2);
        assert_eq!(Map3::<1, 2, 0>::MAP[2], 0);
        assert_eq!(Map3::<1, 2, 0>::MAP_REVERSE[1], 0);
        assert_eq!(Map3::<1, 2, 0>::MAP_REVERSE[2], 1);
        assert_eq!(Map3::<1, 2, 0>::MAP_REVERSE[0], 2);

        assert_eq!(Map4::<1, 3, 0, 2>::MAP[0], 1);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP[1], 3);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP[2], 0);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP[3], 2);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP_REVERSE[1], 0);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP_REVERSE[3], 1);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP_REVERSE[0], 2);
        assert_eq!(Map4::<1, 3, 0, 2>::MAP_REVERSE[2], 3);
    }
}
