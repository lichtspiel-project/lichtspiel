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

/// Macro for a parameterized splitmix algorithm
macro_rules! splitmix {
    ($x:expr, [$s0:expr, $s1:expr, $s2:expr], [$m0:expr, $m1:expr]) => {{
        let mut z = $x;
        z = (z ^ (z >> $s0)).wrapping_mul($m0);
        z = (z ^ (z >> $s1)).wrapping_mul($m1);
        z = z ^ (z >> $s2);
        z
    }};
}

/// Macro to check if an iterator has all zeros (all zero state needs to be avoid by some RNG)
macro_rules! not_all_zeros {
    ($iter:expr) => {
        !$iter.iter().all(|&x| x == 0)
    };
}

/// Macro to get a state of certain size using splitmix
macro_rules! get_state_from_splitmix {
    ($count:expr, $seed:expr, u32) => {{
        let mut rng = SplitMix64::with($seed);
        let mut result = [0u32; $count];
        for v in result.iter_mut() {
            *v = rng.random_u32();
        }
        result
    }};
    ($count:expr, $seed:expr, u64) => {{
        let mut rng = SplitMix64::with($seed);
        let mut result = [0u64; $count];
        for v in result.iter_mut() {
            *v = rng.random_u64();
        }
        result
    }};
}

macro_rules! init_with_state {
    ($state:expr) => {
        if not_all_zeros!($state) {
            Self { state: $state }
        } else {
            Self::with(0)
        }
    };
}

#[cfg(test)]
macro_rules! test_first_5 {
    ($struct:ident, $seed:expr, [$e0:expr,$e1:expr,$e2:expr,$e3:expr,$e4:expr], u32) => {
        let mut rng = $struct::with($seed);
        assert_eq!(rng.random_u32(), $e0);
        assert_eq!(rng.random_u32(), $e1);
        assert_eq!(rng.random_u32(), $e2);
        assert_eq!(rng.random_u32(), $e3);
        assert_eq!(rng.random_u32(), $e4);
    };
    ($struct:ident, $seed:expr, [$e0:expr,$e1:expr,$e2:expr,$e3:expr,$e4:expr], u64) => {
        let mut rng = $struct::with($seed);
        assert_eq!(rng.random_u64(), $e0);
        assert_eq!(rng.random_u64(), $e1);
        assert_eq!(rng.random_u64(), $e2);
        assert_eq!(rng.random_u64(), $e3);
        assert_eq!(rng.random_u64(), $e4);
    };
}
