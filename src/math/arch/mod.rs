#[cfg(target_feature = "sse2")]
mod x86_64;

#[cfg(target_feature = "sse2")]
pub use x86_64::sqrtf;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod arm64;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use arm64::sqrtf;
