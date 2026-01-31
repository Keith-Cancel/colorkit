cfg_items!(
    #[cfg(target_feature = "sse2")] => {
        mod x86_64;
        pub use x86_64::sqrtf;
        pub use x86_64::truncf;
    }
);

cfg_items!(
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] => {
        mod arm64;
        pub use arm64::sqrtf;
    }
);

macro_rules! arch_fn {
    (name: $name:ident, args: $($arg:expr),+) => {
        colorkit::math::arch::arch_fn!(@inner $name, $($arg),+);
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
