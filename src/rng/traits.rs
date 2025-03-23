use crate::rng::r64;

pub(crate) trait RngCore {
    fn random_u64(&mut self) -> r64;
}
