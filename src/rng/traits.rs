use crate::rng::r64;

pub(crate) trait RngCore {
    fn random_u64(&mut self) -> r64;
}

pub(crate) trait Rng: RngCore {
    fn random<T: From<r64>>(&mut self) -> T {
        let v = self.random_u64();
        T::from(v)
    }
}

impl<U: RngCore> Rng for U {}
