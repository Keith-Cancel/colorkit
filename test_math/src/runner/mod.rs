mod perf;
mod primes;
mod relative;
pub use perf::Perf;
pub use primes::PRIMES;
pub use relative::Relative;

pub struct Ansi;
impl Ansi {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
}

pub struct PowersF32(u32);

impl PowersF32 {
    pub const fn new() -> Self {
        return Self(0);
    }
}

impl Iterator for PowersF32 {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let b = self.0;
        if b >= 0x7f800000 {
            return None;
        }
        let n = f32::from_bits(b);
        self.0 = b + 0x800000;
        return Some(n);
    }
}
