struct Xoroshiro1024State {
    state: [u64; 16],
    p: usize,
}

pub struct Xoroshiro1024plus {
    state: Xoroshiro1024State,
}

impl Xoroshiro1024plus {
    fn step(&mut self) {
        let q = self.state.p;
        self.state.p = (self.state.p + 1) & 15;

        let s0 = self.state.state[self.state.p];
        let mut s15 = self.state.state[q];

        let result = xoroshiro_plus!(s0, s15);

        s15 = s15 ^ s0;

        let (a, b, c) = (0u32, 0u32, 0u32);
        self.state.state[q] = s0.rotate_left(a) ^ s15 ^ (s15 << b);
        self.state.state[self.state.p] = s15.rotate_right(c);

        // rotl(s0, A) ^ s15 ^ (s15 << B);
        // s[p] = rotl(s15, C);

        // $state.s0 = $state.s0.rotate_left($a) ^ state ^ (state << $b);
        // $state.s1 = state.rotate_left($c);

        //         const int q = p;
        // const uint64_t s0 = s[p = (p + 1) & 15];
        // uint64_t s15 = s[q];
    }
}
