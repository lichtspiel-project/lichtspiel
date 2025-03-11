macro_rules! advance_xoshiro_4state {
    ($state:expr, [$a:expr, $b:expr]) => {
        let t = $state[0] << $a;
        $state[2] ^= $state[0];
        $state[3] ^= $state[1];
        $state[1] ^= $state[2];
        $state[0] ^= $state[3];
        $state[2] ^= t;
        $state[3] = rotl($state[3], $b);
    };
}
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
        $state[7] = rotl($state[7], $b);
    };
}
