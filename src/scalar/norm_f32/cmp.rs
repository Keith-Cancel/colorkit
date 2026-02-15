use core::cmp::Ordering;
use core::cmp::PartialEq;
use core::cmp::PartialOrd;

use super::NormF32;

// Implement PartialEq Traits
// ==================================================
impl PartialEq for NormF32 {
    #[inline]
    fn eq(&self, rhs: &NormF32) -> bool {
        return f32::eq(&self.0, &rhs.0);
    }
}

impl PartialEq<f32> for NormF32 {
    #[inline]
    fn eq(&self, rhs: &f32) -> bool {
        return f32::eq(&self.0, rhs);
    }
}

impl PartialEq<NormF32> for f32 {
    #[inline]
    fn eq(&self, rhs: &NormF32) -> bool {
        return f32::eq(self, &rhs.0);
    }
}

// Implement PartialOrd Traits
// ==================================================
impl PartialOrd for NormF32 {
    #[inline]
    fn partial_cmp(&self, rhs: &NormF32) -> Option<Ordering> {
        return f32::partial_cmp(&self.0, &rhs.0);
    }
    #[inline]
    fn lt(&self, rhs: &NormF32) -> bool {
        return f32::lt(&self.0, &rhs.0);
    }
    #[inline]
    fn le(&self, rhs: &NormF32) -> bool {
        return f32::le(&self.0, &rhs.0);
    }
    #[inline]
    fn gt(&self, rhs: &NormF32) -> bool {
        return f32::gt(&self.0, &rhs.0);
    }
    #[inline]
    fn ge(&self, rhs: &NormF32) -> bool {
        return f32::ge(&self.0, &rhs.0);
    }
}

impl PartialOrd<f32> for NormF32 {
    #[inline]
    fn partial_cmp(&self, rhs: &f32) -> Option<Ordering> {
        return f32::partial_cmp(&self.0, rhs);
    }
    #[inline]
    fn lt(&self, rhs: &f32) -> bool {
        return f32::lt(&self.0, rhs);
    }
    #[inline]
    fn le(&self, rhs: &f32) -> bool {
        return f32::le(&self.0, rhs);
    }
    #[inline]
    fn gt(&self, rhs: &f32) -> bool {
        return f32::gt(&self.0, rhs);
    }
    #[inline]
    fn ge(&self, rhs: &f32) -> bool {
        return f32::ge(&self.0, rhs);
    }
}

impl PartialOrd<NormF32> for f32 {
    #[inline]
    fn partial_cmp(&self, rhs: &NormF32) -> Option<Ordering> {
        return f32::partial_cmp(self, &rhs.0);
    }
    #[inline]
    fn lt(&self, rhs: &NormF32) -> bool {
        return f32::lt(self, &rhs.0);
    }
    #[inline]
    fn le(&self, rhs: &NormF32) -> bool {
        return f32::le(self, &rhs.0);
    }
    #[inline]
    fn gt(&self, rhs: &NormF32) -> bool {
        return f32::gt(self, &rhs.0);
    }
    #[inline]
    fn ge(&self, rhs: &NormF32) -> bool {
        return f32::ge(self, &rhs.0);
    }
}
