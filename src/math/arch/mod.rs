cfg_items!(
    #[cfg(target_feature = "sse2")] => {
        mod x86_64;
        pub use x86_64::ceilf;
        pub use x86_64::floorf;
        pub use x86_64::roundevenf;
        pub use x86_64::sqrtf;
        pub use x86_64::truncf;
        pub use x86_64::fma;
    }
);

cfg_items!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] => {
        mod arm64;
        pub use arm64::sqrtf;
        pub use arm64::fma;
    }
);

macro_rules! arch_fn {
    (name: $name:ident, args: $($arg:expr),+) => {
        colorkit::math::arch::arch_fn!(@inner $name, $($arg),+);
    };
    (@inner ceilf, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "sse2",
        )))]
        return colorkit::math::arch::ceilf($($arg),+);
    };
    (@inner fma, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "fma",
            all(target_arch = "aarch64", target_feature = "neon")
        )))]
        return colorkit::math::arch::fma($($arg),+);
    };

    (@inner floorf, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "sse2",
        )))]
        return colorkit::math::arch::floorf($($arg),+);
    };
    (@inner roundevenf, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "sse2",
        )))]
        return colorkit::math::arch::roundevenf($($arg),+);
    };
    (@inner sqrtf, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "sse2",
            all(target_arch = "aarch64", target_feature = "neon")
        )))]
        return colorkit::math::arch::sqrtf($($arg),+);
    };
    (@inner truncf, $($arg:expr),+) => {
        #[cfg(all(not(miri), any(
            target_feature = "sse2",
        )))]
        return colorkit::math::arch::truncf($($arg),+);
    };
}
pub(crate) use arch_fn;

macro_rules! cfg_items {
    (#[cfg($cfg:meta)] => { $($i:item)* }) => {
        $(
            #[cfg($cfg)] $i
        )*
    };
}
pub(crate) use cfg_items;
