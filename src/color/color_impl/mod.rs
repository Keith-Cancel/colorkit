#[cfg(feature = "type_const")]
mod nightly_impl;
mod shared;
#[cfg(not(feature = "type_const"))]
mod stable_impl;

#[cfg(feature = "type_const")]
pub use nightly_impl::Color;
#[cfg(not(feature = "type_const"))]
pub use stable_impl::Color;

#[cfg(test)]
mod test {
    use colorkit::space::*;

    use super::*;
    #[test]
    fn check_repeat() {
        let c = Color::<Srgb>::repeat(0.5);
        assert_eq!(c.channels(), 3);
        assert_eq!(c.get(0), Some(&0.5f32));
        assert_eq!(c.get(1), Some(&0.5f32));
        assert_eq!(c.get(2), Some(&0.5f32));
        assert_eq!(c.get(3), None);
    }
}
