use crate::rng::splitmix::splitmix64::SplitMix64;
use crate::rng::{R32, R64};

struct Xoshiro128 {
    state: [u32; 4],
}

impl Xoshiro128 {
    fn new(state: [u32; 4]) -> Self {
        if not_all_zeros!(state) {
            Xoshiro128 { state }
        } else {
            Xoshiro128::with(0)
        }
    }
    fn with(seed: u64) -> Self {
        let state = get_state_from_splitmix!(4, seed, u32);
        Xoshiro128::new(state)
    }
}

impl Default for Xoshiro128 {
    fn default() -> Self {
        Xoshiro128::with(0)
    }
}

impl R32 for Xoshiro128 {
    fn random_u32(&mut self) -> u32 {
        let result = self.state[0];
        advance_xoshiro_4state!(self.state, [9, 11]);
        result
    }
}

struct Xoshiro128plus {
    state: [u32; 4],
}

impl R32 for Xoshiro128plus {
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_plus!(self.state[0], self.state[3]);
        advance_xoshiro_4state!(self.state, [9, 11]);
        result
    }
}

struct Xoshiro128plusplus {
    state: [u32; 4],
}

impl R32 for Xoshiro128plusplus {
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_plusplus!(self.state[0], self.state[3], [17]);
        advance_xoshiro_4state!(self.state, [9, 11]);
        result
    }
}

struct Xoshiro128starstar {
    state: [u32; 4],
}

impl R32 for Xoshiro128starstar {
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_starstar!(self.state[1], [5, 7, 9]);
        advance_xoshiro_4state!(self.state, [9, 11]);
        result
    }
}
