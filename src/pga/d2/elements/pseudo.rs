use core::ops::{Add, BitOr, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{BiVector2, Dual, Scalar2, Vector2};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Pseudo2<T> {
    pub e012: T,
}

impl<T> Pseudo2<T> {
    #[inline]
    pub const fn new(e012: T) -> Self {
        Pseudo2 { e012 }
    }
}

impl<T> Pseudo2<T>
where
    T: Num,
{
    pub const ZERO: Self = Self { e012: T::ZERO };
}

impl<T> Neg for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn neg(self) -> Pseudo2<T> {
        Pseudo2 { e012: -self.e012 }
    }
}

impl<T> Not for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn not(self) -> Pseudo2<T> {
        -self
    }
}

impl<T> Dual for Pseudo2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn dual(self) -> Scalar2<T> {
        Scalar2(self.e012)
    }
}

impl<T> Mul<T> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn mul(self, rhs: T) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 * rhs,
        }
    }
}

impl<T> MulAssign<T> for Pseudo2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e012 *= rhs;
    }
}

impl<T> Div<T> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn div(self, rhs: T) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 / rhs,
        }
    }
}

impl<T> DivAssign<T> for Pseudo2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e012 /= rhs;
    }
}

impl<T> Add<Pseudo2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn add(self, rhs: Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 + rhs.e012,
        }
    }
}

impl<T> Sub<Pseudo2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn sub(self, rhs: Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 - rhs.e012,
        }
    }
}

impl<T> BitOr<Scalar2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 * other.0,
        }
    }
}

impl<T> BitOr<Vector2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn bitor(self, other: Vector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e012 * other.e2,
            e20: self.e012 * other.e1,
            e12: T::ZERO,
        }
    }
}

impl<T> BitOr<BiVector2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn bitor(self, other: BiVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: -(self.e012 * other.e12),
            e1: T::ZERO,
            e2: T::ZERO,
        }
    }
}

impl<T> Mul<Scalar2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn mul(self, other: Scalar2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar2<T>> for Pseudo2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e012 *= other.0;
    }
}

impl<T> Mul<Vector2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn mul(self, other: Vector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e012 * other.e2,
            e20: self.e012 * other.e1,
            e12: T::ZERO,
        }
    }
}

impl<T> Mul<BiVector2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn mul(self, other: BiVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: -(self.e012 * other.e12),
            e1: T::ZERO,
            e2: T::ZERO,
        }
    }
}

impl<T> Div<Scalar2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn div(self, other: Scalar2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e012 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar2<T>> for Pseudo2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e012 /= other.0;
    }
}
