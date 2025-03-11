//! # Scramblers
//!
//! Implementation of the scramblers defined in the Xoroshiro/xoroshi paper
//! as well as the state updates

/// Macro for updating the state in xoroshiro with state <= 128P
macro_rules! advance_xoroshiro_two_state {
    ($s0:expr, $s1:expr, [$a:expr, $b:expr, $c:expr]) => {
        let state = $s1 ^ $s0;
        $s0 = $s0.rotate_left($a) ^ state ^ (state << $b);
        $s1 = state.rotate_left($c);
    };
}

/// Macro for the Plus variant of xoroshiro
macro_rules! xoroshiro_plus {
    ($s0:expr, $s1:expr) => {
        $s0.wrapping_add($s1)
    };
}

/// Macro for the PlusPlus variant of xoroshiro
macro_rules! xoroshiro_plusplus {
    ($s0:expr, $s1:expr, [$r:expr]) => {
        $s0.wrapping_add($s1).rotate_left($r).wrapping_add($s0)
    };
}

/// Macro for the Star variant of xoroshiro
macro_rules! xoroshiro_star {
    ($s0:expr, [$s:expr]) => {
        $s0.wrapping_mul($s)
    };
}

/// Macro for the StarStar variant of xoroshiro
macro_rules! xoroshiro_starstar {
    ($s0:expr, [$s:expr, $r:expr, $t:expr]) => {
        $s0.wrapping_mul($s).rotate_left($r).wrapping_mul($t)
    };
}

/// Macro for advancing in four state xoshiro
macro_rules! advance_xoshiro_4state {
    ($state:expr, [$a:expr, $b:expr]) => {
        let t = $state[0] << $a;
        $state[2] ^= $state[0];
        $state[3] ^= $state[1];
        $state[1] ^= $state[2];
        $state[0] ^= $state[3];
        $state[2] ^= t;
        $state[3] = $state[3].rotate_left($b);
    };
}

/// Macro for advancing in eight state xoshiro
macro_rules! advance_xoshiro_8state {
    ($state:expr, [$a:expr, $b:expr]) => {
        let t = $state[0] << $a;
        $state[2] ^= $state[0];
        $state[5] ^= $state[1];
        $state[1] ^= $state[2];
        $state[7] ^= $state[3];
        $state[3] ^= $state[4];
        $state[4] ^= $state[5];
        $state[0] ^= $state[6];
        $state[6] ^= $state[7];
        $state[6] ^= t;
        $state[7] = $state[7].rotate_left($b);
    };
}
