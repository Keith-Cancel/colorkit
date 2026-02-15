use super::ColorSpace;

pub trait ColorWrap<W> {
    type Inner: ColorSpace;
    fn wrap_inner(self, inner: Self::Inner) -> W;
    fn unwrap_inner(wrapper: W) -> Self::Inner;
}

/// Marker type making [`ColorWrap`] a no-op
pub struct WrapIdentity;

impl<S: ColorSpace> ColorWrap<S> for WrapIdentity {
    type Inner = S;
    fn wrap_inner(self, inner: Self::Inner) -> S {
        return inner;
    }
    fn unwrap_inner(wrapper: S) -> Self::Inner {
        return wrapper;
    }
}
