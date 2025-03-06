use std::u64;

#[derive(Debug)]
pub struct PCG32 {
    state: u64,
    seq: u64,
}

const PCG32_DEFAULT_STATE: u64 = 0x853c49e6748fea9bu64;
const PCG32_DEFAULT_SEQ: u64 = 0xda3e39cb94b95bdbu64;

impl PCG32 {
    pub fn new() -> Self {
        PCG32 {
            state: PCG32_DEFAULT_STATE,
            seq: PCG32_DEFAULT_SEQ,
        }
    }
    pub fn srandom(initstate: u64, initseq: u64) -> Self {
        let state = 0u64;
        let seq = (initseq << 1) | 1;
        let mut pcg32 = PCG32 { state, seq };
        let _ = pcg32.random();
        pcg32.state += initstate;
        let _ = pcg32.random();
        pcg32
    }
    pub fn random(&mut self) -> u32 {
        let oldstate = self.state;
        log::debug!(" old: {:x}", oldstate);
        self.state = oldstate
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(self.seq);
        log::debug!(" new: {:x} {0}", self.state);
        let xorshifted: u32 = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        let nrot: u32 = 0u32.wrapping_sub(rot);
        log::debug!("vals: {:x} {:x} {:x}", xorshifted, rot, nrot);
        let result = (xorshifted >> rot) | (xorshifted << (nrot & 31));
        log::debug!(" res: {:x}", result);
        result
    }
}

fn u64_to_u32_high(num: u64) -> u32 {
    u64_to_u32(num, true)
}

fn u64_to_u32_low(num: u64) -> u32 {
    u64_to_u32(num, false)
}

fn u64_to_u32(num: u64, from_high: bool) -> u32 {
    let bytes = num.to_ne_bytes();
    let (mut high, mut low) = bytes.split_at(4);

    if cfg!(target_endian = "little") {
        std::mem::swap(&mut high, &mut low);
    }
    if from_high {
        u32::from_ne_bytes(high.try_into().unwrap())
    } else {
        u32::from_ne_bytes(low.try_into().unwrap())
    }
}

impl Default for PCG32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PCG32 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let pcg_32 = PCG32::new();
        assert_eq!(pcg_32.state, PCG32_DEFAULT_STATE);
        assert_eq!(pcg_32.seq, PCG32_DEFAULT_SEQ);
    }

    #[test]
    fn default_works() {
        let pcg_32 = PCG32::default();
        assert_eq!(pcg_32.state, PCG32_DEFAULT_STATE);
        assert_eq!(pcg_32.seq, PCG32_DEFAULT_SEQ);
    }

    #[test]
    fn random_works() {
        let mut pcg32 = PCG32::srandom(42, 54);

        assert_eq!(pcg32.random(), 0xa15c02b7);
        assert_eq!(pcg32.random(), 0x7b47f409);
        assert_eq!(pcg32.random(), 0xba1d3330);
        assert_eq!(pcg32.random(), 0x83d2f293);
        assert_eq!(pcg32.random(), 0xbfa4784b);
        assert_eq!(pcg32.random(), 0xcbed606e);
        assert_eq!(pcg32.random(), 0xbfc6a3ad);
        assert_eq!(pcg32.random(), 0x812fff6d);
    }

    #[test]
    fn from_high_to_low_works() {
        let mut pcg32 = PCG32::srandom(42, 54);
        _ = pcg32.random();
        let h = u64_to_u32_high(pcg32.state);
        let l = u64_to_u32_low(pcg32.state);
        assert_eq!(h, 0x2b47fed8);
        assert_eq!(l, 0x8766bb05);

        _ = pcg32.random();
        let h = u64_to_u32_high(pcg32.state);
        let l = u64_to_u32_low(pcg32.state);
        assert_eq!(h, 0x8b33296d);
        assert_eq!(l, 0x19bf5b4e);
    }

    #[test]
    fn iterator_works() {
        let pcg32 = PCG32::srandom(42, 54);
        let exp = [
            0xba1d3330_u32,
            0x83d2f293_u32,
            0xbfa4784b_u32,
            0xcbed606e_u32,
        ];

        for (rnd, ex) in pcg32.skip(2).take(4).zip(exp.iter()) {
            assert_eq!(rnd, *ex);
        }
    }
}
