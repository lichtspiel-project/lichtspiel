use super::core::Random64;

/// Golden Ratio, prime version: 0x9e3779b97f4a7c55;
const GOLDEN_RATIO: u64 = 0x9e3779b97f4a7c15;

pub(crate) struct Splitmix {
    state: u64,
}

impl Splitmix {
    fn new(state: u64) -> Self {
        Self { state }
    }
    fn random_u64(&mut self) -> Random64 {
        self.state = self.state.wrapping_add(GOLDEN_RATIO);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z = z ^ (z >> 31);
        Random64::from(z)
    }
    pub fn random<T: From<Random64>>(&mut self) -> T {
        let v = self.random_u64();
        T::from(v)
    }
}

impl Default for Splitmix {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let seed = 1234567u64;
        let mut rng = Splitmix::new(seed);
        let expected = [
            6457827717110365317_u64,
            3203168211198807973_u64,
            9817491932198370423_u64,
            4593380528125082431_u64,
            16408922859458223821_u64,
        ];
        for val in expected.iter() {
            assert_eq!(*val, rng.random())
        }
    }

    #[test]
    fn zero_seed() {
        let seed = 0u64;
        let mut rng = Splitmix::new(seed);
        assert_ne!(rng.random::<u64>(), 0u64);
        assert_ne!(rng.random::<u64>(), 0u64);
    }
}
