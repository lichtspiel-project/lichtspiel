use std::marker::PhantomData;

use super::{r64, traits::Rng};

pub(crate) struct Iter<R, T> {
    pub(crate) rng: R,
    pub(crate) data: PhantomData<T>,
}

impl<R: Rng, T: From<r64>> Iterator for Iter<R, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Squares;

    #[test]
    fn first() {
        let rng = Squares::with(0, 0x548c9decbce65297_u64);
        let it = rng.to_iter::<u64>();
        let v: Vec<_> = it.take(10).collect();
        let expect = [
            0x36d88366cee633a5_u64,
            0x944716e00e60dfaa_u64,
            0xc8a8f4e0678654bf_u64,
            0x35cc666aab11c80d_u64,
            0x7094eab1cbae8747_u64,
            0xa2a1b6f56e92a96f_u64,
            0xd884957d48007552_u64,
            0xe61f37b97d593453_u64,
            0xe4d45c4762b10dad_u64,
            0xb0dbd071f201dd2a_u64,
        ];
        assert_eq!(v, expect);
    }
}
