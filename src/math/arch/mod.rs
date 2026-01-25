#[cfg(target_feature = "sse2")]
mod x86_64;

#[cfg(target_feature = "sse2")]
pub use x86_64::sqrtf;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod arm64;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use arm64::sqrtf;

macro_rules! arch_fn {
    (name: $name:ident, args: $($arg:expr),+) => {
        colorkit::math::arch::arch_fn!(@inner $name, $($arg),+);
    };
    (@inner sqrtf, $($arg:expr),+) => {
        #[cfg(any(
            target_feature = "sse2",
            all(target_arch = "aarch64", target_feature = "neon")
        ))]
        return colorkit::math::arch::sqrtf($($arg),+);
    };
}
pub(crate) use arch_fn;
