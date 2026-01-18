mod alpha;
mod array;
mod common;

pub use alpha::AlphaKind;
pub use alpha::AlphaKindAbsent;
pub use alpha::AlphaKindInfo;
pub use alpha::AlphaKindNormal;
pub use alpha::AlphaKindPreMul;
pub use alpha::AlphaStore;
pub use alpha::AlphaStoreTry;
pub use alpha::AlphaValue;
pub use alpha::AlphaValueTry;
pub use array::ColorArray;
pub use common::ColorCommon;

/// Error returned when attempting to set a channel value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelSetError {
    /// No such channel exists to set
    Absent = 1,
    /// Such a channel exists conceptually, but is computed from other channels.
    Computed,
    /// Such a channel exists, but can not be updated independently of other channels.
    Dependent,
    /// The provided value was rejected by validation/invariant rules.
    InvalidValue,
}
