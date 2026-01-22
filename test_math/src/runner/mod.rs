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

pub struct RandomF32([u64; 2], usize);

impl RandomF32 {
    pub const fn new(cnt: usize, seed: u64) -> Self {
        let low = seed >> 32;
        let hi = seed & 0xffffffff;

        let mut ret = Self(
            [
                0x6a09e667f3bcc908 ^ low,
                0xbb67ae8584caa73b ^ hi,
            ],
            cnt,
        );
        ret.next_u32();
        return ret;
    }

    /// xorshift128p
    const fn next_u32(&mut self) -> u32 {
        let s = self.0[1];
        let mut t = self.0[0];
        self.0[0] = s;
        t ^= t << 23;
        t ^= t >> 18;
        t ^= s ^ (s >> 5);
        self.0[1] = t;
        return t.wrapping_add(s) as u32;
    }
}

impl Iterator for RandomF32 {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.1 == 0 {
            return None;
        }
        self.1 -= 1;
        loop {
            let bits = self.next_u32();
            let exp = (bits >> 23) & 0xff;
            // reject:
            // exp == 255 -> NaN / Inf
            if exp == 0xff {
                continue;
            }
            return Some(f32::from_bits(bits));
        }
    }
}
