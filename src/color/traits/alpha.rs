use super::ChannelSetError;

/// Describes how a color represents its alpha channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaKind {
    Absent = 1,
    Normal,
    PreMul,
}

/// Don't re-export to keep, basically sealed
pub trait AlphaKindEnum {
    fn kind() -> AlphaKind;
}

/// Marker type for a color with no alpha channel
///
/// Mainly exists so I can write some blankent impls
/// if #![feature(adt_const_params)] is stabilized uneeded
/// I can just my enum directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlphaKindAbsent;

impl AlphaKindEnum for AlphaKindAbsent {
    #[inline]
    fn kind() -> AlphaKind {
        return AlphaKind::Absent;
    }
}

/// Marker type for a color with an alpha channel
///
/// Mainly exists so I can write some blankent impls
/// if #![feature(adt_const_params)] is stabilized uneeded
/// I can just my enum directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlphaKindNormal;

impl AlphaKindEnum for AlphaKindNormal {
    #[inline]
    fn kind() -> AlphaKind {
        return AlphaKind::Normal;
    }
}

/// Marker type for a color with premultipled alpha
///
/// Mainly exists so I can write some blankent impls
/// if #![feature(adt_const_params)] is stabilized uneeded
/// I can just my enum directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AlphaKindPreMul;

impl AlphaKindEnum for AlphaKindPreMul {
    #[inline]
    fn kind() -> AlphaKind {
        return AlphaKind::PreMul;
    }
}

/// Info about a colors alpha channel.
pub trait AlphaKindInfo {
    /// The kind of alpha channel this color has.
    type Kind: AlphaKindEnum;

    /// The kinda of Alpha this color has.
    #[inline]
    fn kind(&self) -> AlphaKind {
        return Self::Kind::kind();
    }
}

/// Optional access to stored alpha channel in a color.
pub trait AlphaStoreTry: AlphaKindInfo {
    /// Try returning a reference to the alpha channel, if present.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha_ref(&self) -> Option<&f32>;

    /// Try to returning a mutable reference to the alpha channel, if present.
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha_mut(&mut self) -> Option<&mut f32>;
}

// Access to stored alpha channel in a color.
pub trait AlphaStore: AlphaStoreTry {
    /// Returns a reference to the alpha channel.
    fn alpha_ref(&self) -> &f32 {
        return self.try_alpha_ref().unwrap();
    }

    /// Returns a mutable reference to the alpha channel.
    fn alpha_mut(&mut self) -> &mut f32 {
        return self.try_alpha_mut().unwrap();
    }
}

/// Optional value based access to a color's alpha channel.
pub trait AlphaValueTry: AlphaKindInfo {
    /// Try to get the value of the alpha channel
    ///
    /// Returns [`None`] if this color has no alpha channel.
    fn try_alpha(&self) -> Option<f32>;

    /// Try to set the value of the alpha channel
    ///
    /// Returns a [`ChannelSetError`] error if this color does not support setting alpha.
    fn try_alpha_set(&mut self, value: f32) -> Result<(), ChannelSetError>;
}

/// Value based access to a color's alpha channel.
pub trait AlphaValue: AlphaValueTry {
    /// Get the value of the alpha channel
    fn alpha(&self) -> f32 {
        return self.try_alpha().unwrap();
    }

    /// Set the value of the alpha channel
    fn alpha_set(&mut self, value: f32) {
        self.try_alpha_set(value).unwrap();
    }
}

// Blanket impls
// ========================================================
impl<T: AlphaStoreTry> AlphaValueTry for T {
    #[inline]
    fn try_alpha(&self) -> Option<f32> {
        return self.try_alpha_ref().copied();
    }

    #[inline]
    fn try_alpha_set(&mut self, value: f32) -> Result<(), ChannelSetError> {
        let dst = self.try_alpha_mut().ok_or(ChannelSetError::Absent)?;
        *dst = value;
        return Ok(());
    }
}

impl<T: AlphaStore> AlphaValue for T {
    #[inline]
    fn alpha(&self) -> f32 {
        return *self.alpha_ref();
    }

    #[inline]
    fn alpha_set(&mut self, value: f32) {
        *self.alpha_mut() = value;
    }
}

// Use the technique here for AlphaTryStore
// https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983
mod private {
    use super::*;

    pub trait AlphaAbsent: AlphaKindInfo<Kind = AlphaKindAbsent> {}
    pub trait AlphaNormal: AlphaKindInfo<Kind = AlphaKindNormal> + AlphaStore {}
    pub trait AlphaPre: AlphaKindInfo<Kind = AlphaKindPreMul> + AlphaStore {}

    impl<T: AlphaKindInfo<Kind = AlphaKindAbsent>> AlphaAbsent for T {}
    impl<T: AlphaKindInfo<Kind = AlphaKindNormal> + AlphaStore> AlphaNormal for T {}
    impl<T: AlphaKindInfo<Kind = AlphaKindPreMul> + AlphaStore> AlphaPre for T {}

    impl<T: AlphaKindInfo + TryStoreHelper<T::Kind>> AlphaStoreTry for T {
        #[inline]
        fn try_alpha_ref(&self) -> Option<&f32> {
            return self.try_ref_imp();
        }
        #[inline]
        fn try_alpha_mut(&mut self) -> Option<&mut f32> {
            return self.try_mut_imp();
        }
    }

    trait TryStoreHelper<Kind> {
        fn try_ref_imp(&self) -> Option<&f32>;
        fn try_mut_imp(&mut self) -> Option<&mut f32>;
    }

    impl<T: AlphaAbsent> TryStoreHelper<AlphaKindAbsent> for T {
        #[inline]
        fn try_mut_imp(&mut self) -> Option<&mut f32> {
            return None;
        }
        #[inline]
        fn try_ref_imp(&self) -> Option<&f32> {
            return None;
        }
    }

    impl<T: AlphaNormal> TryStoreHelper<AlphaKindNormal> for T {
        #[inline]
        fn try_mut_imp(&mut self) -> Option<&mut f32> {
            return Some(self.alpha_mut());
        }
        #[inline]
        fn try_ref_imp(&self) -> Option<&f32> {
            return Some(self.alpha_ref());
        }
    }

    impl<T: AlphaPre> TryStoreHelper<AlphaKindPreMul> for T {
        #[inline]
        fn try_mut_imp(&mut self) -> Option<&mut f32> {
            return Some(self.alpha_mut());
        }
        #[inline]
        fn try_ref_imp(&self) -> Option<&f32> {
            return Some(self.alpha_ref());
        }
    }
}
