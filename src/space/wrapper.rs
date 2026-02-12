use super::ColorSpace;

pub trait ColorWrap<W> {
    type Inner: ColorSpace;
    fn into_inner(wrapper: W) -> Self::Inner;
    fn from_inner(inner: Self::Inner) -> W;
}

/// Marker type making [`ColorWrap`] a no-op
pub struct WrapIdentity;

impl<S: ColorSpace> ColorWrap<S> for WrapIdentity {
    type Inner = S;
    fn into_inner(wrapper: S) -> Self::Inner {
        return wrapper;
    }
    fn from_inner(inner: Self::Inner) -> S {
        return inner;
    }
}
