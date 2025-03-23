use std::marker::PhantomData;

use super::iter::Iter;
use crate::rng::r64;
pub(crate) trait RngCore {
    fn random_u64(&mut self) -> r64;
}

pub(crate) trait Rng: RngCore {
    fn random<T: From<r64>>(&mut self) -> T {
        let v = self.random_u64();
        T::from(v)
    }
    fn to_iter<T: From<r64>>(self) -> Iter<Self, T>
    where
        Self: Sized,
    {
        Iter {
            rng: self,
            data: PhantomData::<T>,
        }
    }
}

impl<U: RngCore> Rng for U {}
