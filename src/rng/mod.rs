#[macro_use]
mod macros;
pub mod bitmixer;
pub mod core;
pub mod float;
pub mod pcg;
pub mod splitmix;
pub mod squares;
// pub mod xoroshiro;
pub mod xorshift;
// pub mod xoshiro;

pub use core::RNG;
