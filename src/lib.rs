//! Athena is math crate tailored for game engines, rendering and physics simulations.
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod scalar;
// mod simd;

#[cfg(feature = "lina")]
mod lina;

#[cfg(any(feature = "pga2", feature = "pga3"))]
mod pga;

pub use self::scalar::*;

#[cfg(feature = "lina")]
pub use self::lina::*;

#[cfg(any(feature = "pga2", feature = "pga3"))]
pub use self::pga::*;

#[cfg(feature = "serde")]
mod array_init;

#[cfg(feature = "serde")]
mod serde;
