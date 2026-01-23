#[cfg(target_feature = "sse2")]
mod x86_64;

#[cfg(target_feature = "sse2")]
pub use x86_64::sqrtf;
