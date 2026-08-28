//! Projective Geometric Algebra (PGA) is a mathematical framework
//! that extends traditional Euclidean geometry to include points at infinity and allows for the representation of geometric transformations in a unified way.
//!
//! It provides a powerful toolset for working with lines, points, and other geometric entities in two-dimensional and three-dimensional space.

use core::ops::BitXor;

#[cfg(feature = "pga2")]
mod d2;

#[cfg(feature = "pga3")]
mod d3;

#[cfg(feature = "pga2")]
pub use self::d2::*;

#[cfg(feature = "pga3")]
pub use self::d3::*;

/// A trait for duality operations.
pub trait Dual {
    /// Dual type of the element.
    type Output;

    /// Returns the dual of the element.
    fn dual(self) -> Self::Output;
}

/// Computes the regressive product of two elements in projective geometric algebra.
fn regressive<T, U, R>(lhs: T, rhs: U) -> R
where
    T: Dual,
    U: Dual,
    T::Output: BitXor<U::Output>,
    <T::Output as BitXor<U::Output>>::Output: Dual<Output = R>,
{
    (lhs.dual() ^ rhs.dual()).dual()
}

/// Computes the regressive product of three elements in projective geometric algebra.
#[cfg(feature = "pga3")]
fn regressive3<T, U, Y, R>(a: T, b: U, c: Y) -> R
where
    T: Dual,
    U: Dual,
    Y: Dual,
    T::Output: BitXor<U::Output>,
    <T::Output as BitXor<U::Output>>::Output: BitXor<Y::Output>,
    <<T::Output as BitXor<U::Output>>::Output as BitXor<Y::Output>>::Output: Dual<Output = R>,
{
    (a.dual() ^ b.dual() ^ c.dual()).dual()
}
