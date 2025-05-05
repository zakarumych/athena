use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Scalars are the basic building blocks of the algebraic structures.
///
/// Note that simd arrays of primitives scalars are also considered scalars,
/// in which case all operations are performed element-wise.
pub trait Num:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
    + core::fmt::Debug
{
    /// The additive identity.
    const ZERO: Self;

    /// Near zero value.
    const EPSILON: Self;

    /// The multiplicative identity.
    const ONE: Self;

    /// Constant value of two.
    /// x * T::TWO = x + x
    /// x / T::TWO + x / T::TWO = x
    const TWO: Self;

    /// Returns the reciprocal of the scalar.
    #[inline]
    fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Returns the square root of the scalar.
    fn sqrt(self) -> Self;

    /// Returns the absolute value of the scalar.
    fn abs(self) -> Self;

    /// Returns the sine of the scalar.
    fn sin(self) -> Self;

    /// Returns the cosine of the scalar.
    fn cos(self) -> Self;

    /// Returns the sine and cosine of the scalar.
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    /// Returns the arcsine of the scalar.
    fn asin(self) -> Self;

    /// Returns the arccosine of the scalar.
    fn acos(self) -> Self;

    /// Returns the tangent of the scalar.
    fn tan(self) -> Self;

    /// Returns the arctangent of the scalar.
    fn atan(self) -> Self;

    /// Returns the arctangent between two scalars.
    fn atan2(self, rhs: Self) -> Self;
}

#[cfg(feature = "std")]
impl Num for f32 {
    const ZERO: Self = 0.0;
    const EPSILON: Self = f32::EPSILON;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    #[inline]
    fn atan2(self, rhs: Self) -> Self {
        self.atan2(rhs)
    }
}

#[cfg(feature = "std")]
impl Num for f64 {
    const ZERO: Self = 0.0;
    const EPSILON: Self = f64::EPSILON;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;

    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    #[inline]
    fn atan2(self, rhs: Self) -> Self {
        self.atan2(rhs)
    }
}
