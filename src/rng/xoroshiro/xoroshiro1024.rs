use crate::rng::R64;

macro_rules! advance_state_xoroshiro_1024 {
    ($self:expr, $q:expr, $s0:expr, $s15:expr, [$a:expr, $b:expr, $c:expr]) => {
        let s15 = $s15 ^ $s0;

        $self.state[$q] = $s0.rotate_left($a) ^ s15 ^ (s15 << $b);
        $self.state[$self.p] = s15.rotate_right($c);
    };
}

macro_rules! advance_p_xoroshiro_1024 {
    ($self:expr) => {{
        let q = $self.p;
        $self.p = ($self.p + 1) & 15;
        let s0 = $self.state[$self.p];
        let s15 = $self.state[q];

        (q, s0, s15)
    }};
}

struct Xoroshiro1024plus {
    state: [u64; 16],
    p: usize,
}

impl R64 for Xoroshiro1024plus {
    fn random_u64(&mut self) -> u64 {
        let (q, s0, s15) = advance_p_xoroshiro_1024!(self);

        let result = xoroshiro_plus!(s0, s15);
        advance_state_xoroshiro_1024!(self, q, s0, s15, [25, 27, 36]);
        result
    }
}
