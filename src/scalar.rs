use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Scalars are the basic building blocks of the algebraic structures.
///
/// Note that simd arrays of primitives scalars are also considered scalars,
/// in which case all operations are performed element-wise.
pub trait Scalar:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Copy
{
    /// The additive identity.
    const ZERO: Self;

    /// The multiplicative identity.
    const ONE: Self;

    /// Returns the reciprocal of the scalar.
    #[inline]
    fn recip(self) -> Self {
        Self::ONE / self
    }
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn recip(self) -> Self {
        self.recip()
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn recip(self) -> Self {
        self.recip()
    }
}
