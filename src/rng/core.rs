use super::{bitmixer, pcg, squares, xorshift};

pub trait R64 {
    fn random_u64(&mut self) -> u64;
    fn random_u32(&mut self) -> u32 {
        let num = self.random_u64();
        (num >> 32) as u32
    }
}

pub trait R32 {
    fn random_u32(&mut self) -> u32;
    fn random_u64(&mut self) -> u64 {
        let x = self.random_u32() as u64;
        let y = self.random_u32() as u64;
        (x << 32) | y
    }
}

pub enum RNG {
    PCG32(pcg::PCG32),
    XorshiftStar(xorshift::XorshiftStar32),
    Squares(squares::Squares),
}

impl RNG {
    pub fn with_pcg32() -> Self {
        RNG::PCG32(pcg::PCG32::new())
    }
    pub fn with_xorshift() -> Self {
        RNG::XorshiftStar(xorshift::XorshiftStar32::new())
    }
    pub fn with_squares() -> Self {
        RNG::Squares(squares::Squares::default())
    }
    pub fn random(&mut self) -> u32 {
        match self {
            RNG::PCG32(pcg) => pcg.random(),
            RNG::XorshiftStar(xorshift) => xorshift.random(),
            RNG::Squares(sq) => sq.random_u32(),
        }
    }
    pub fn random_bitmix(&mut self) -> u32 {
        let v = self.random();
        bitmixer::bitmix32(v)
    }
    pub fn bounded_random(&mut self, bound: u32) -> u32 {
        let threshold = 0u32.wrapping_sub(bound) % bound;
        loop {
            let r = self.random();
            if r >= threshold {
                return r % bound;
            }
        }
    }
    /// Implementation based on https://www.pcg-random.org/posts/bounded-rands.html
    pub fn bounded_random_fast(&mut self, bound: u32) -> u32 {
        let mut x = self.random();
        let mut m = x as u64 * bound as u64;
        let mut l = m as u32;

        if l < bound {
            let mut t = 0u32.wrapping_sub(bound);
            if t >= bound {
                t -= bound;
                if t >= bound {
                    t %= bound;
                }
            }
            while l < t {
                x = self.random();
                m = x as u64 * bound as u64;
                l = m as u32;
            }
        }
        (m >> 32) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_works() {
        let mut pcg32 = RNG::with_pcg32();
        let b = 9373u32;

        for _ in 0..10 {
            assert!(pcg32.bounded_random(b) <= b)
        }
    }

    #[test]
    fn bounded_fast_works() {
        let mut xor = RNG::with_xorshift();
        let b = 9373u32;

        for _ in 0..10 {
            assert!(xor.bounded_random(b) <= b)
        }
    }
}
