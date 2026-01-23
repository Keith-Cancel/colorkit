mod quirt;

pub use quirt::quirt_f32;

/// Common math functions
///
/// This is to make up for the fact #[no_std] does
/// have all math functions for f32, so I have reimplemented
/// some for a no standard enviroment.
pub trait MathFuncs {
    /// Computes the quintic root or 5th root.
    fn quirt(self) -> Self;
}

impl MathFuncs for f32 {
    fn quirt(self) -> Self {
        return quirt_f32(self);
    }
}
