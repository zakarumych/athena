//! Athena is math crate tailored for game engines, rendering and physics simulations.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

macro_rules! count {
    () => { 0 };
    ($head:tt $($tail:tt)*) => { 1 + count!($($tail)*) };
}

mod scalar;
// mod simd;

mod matrix;
mod pga;
mod vector;

pub use self::{matrix::*, pga::*, scalar::*, vector::*};

#[cfg(feature = "serde")]
mod array_init;

#[cfg(feature = "serde")]
mod serde;
