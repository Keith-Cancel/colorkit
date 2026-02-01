pub use colorkit::space::ColorSpace;

pub trait Interpolation {}

impl<C: ColorSpace> Interpolation for C {}
