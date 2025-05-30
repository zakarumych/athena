#![deny(clippy::inconsistent_struct_constructor)]

mod bivector;
mod pseudo;
mod scalar;
mod trivector;
mod vector;

use core::ops::BitXor;

pub(crate) use self::{bivector::*, pseudo::*, scalar::*, trivector::*, vector::*};

/// A trait for duality operations.
pub trait Dual {
    /// Dual type of the element.
    type Output;

    /// Returns the dual of the element.
    fn dual(self) -> Self::Output;
}

pub fn regressive<T, U, R>(lhs: T, rhs: U) -> R
where
    T: Dual,
    U: Dual,
    T::Output: BitXor<U::Output>,
    <T::Output as BitXor<U::Output>>::Output: Dual<Output = R>,
{
    (lhs.dual() ^ rhs.dual()).dual()
}

pub fn regressive3<T, U, Y, R>(a: T, b: U, c: Y) -> R
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
