/// Helper trait for [`AlphaSome`] and [`AlphaNone`] specifying where
/// the alpha channel is in the [`Layout`](image::layout::Layout).
pub trait AlphaChannel: Copy {
    /// What channel in the color [`Layout`](image::layout::Layout) is the alpha channel at.
    const CHANNEL: Option<usize>;
}

/// Trait that marks the type is an alpha channel and it's index
pub trait IsAlpha: AlphaChannel {
    const CHAN_IDX: usize;
}

/// Marker type indicating the presence of an alpha channel.
///
/// `N` specifies the channel index of alpha in the layo
#[derive(Clone, Copy, Debug)]
pub struct AlphaSome<const N: usize = 3>;

impl<const N: usize> IsAlpha for AlphaSome<N> {
    const CHAN_IDX: usize = N;
}

impl<const N: usize> AlphaChannel for AlphaSome<N> {
    const CHANNEL: Option<usize> = Some(N);
}

/// Marker type indicating the absence of an alpha channel.
#[derive(Clone, Copy, Debug)]
pub struct AlphaNone;

impl AlphaChannel for AlphaNone {
    const CHANNEL: Option<usize> = None;
}
