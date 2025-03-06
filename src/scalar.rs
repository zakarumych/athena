use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Scalars are the basic building blocks of the algebraic structures.
pub trait Scalar: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Sized {
    /// The additive identity.
    const ZERO: Self;

    /// The multiplicative identity.
    const ONE: Self;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
