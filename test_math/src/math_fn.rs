use rug::Float;
/// Single Agrument Math functions
pub trait MathFn {
    const NAME: &'static str;
    fn test_f32_impl(x: f32) -> f32;
    fn std_f32_impl(x: f32) -> f32;
    fn rug_impl(x: f32) -> Float;
}
