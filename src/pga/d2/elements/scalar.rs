use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{BiVector2, Dual, Pseudo2, Vector2};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Scalar2<T>(pub T);

impl<T> Scalar2<T> {
    #[inline]
    pub const fn new(s: T) -> Self {
        Scalar2(s)
    }
}

impl<T> Neg for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn neg(self) -> Scalar2<T> {
        Scalar2(-self.0)
    }
}

impl<T> Not for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn not(self) -> Scalar2<T> {
        self
    }
}

impl<T> Dual for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn dual(self) -> Pseudo2<T> {
        Pseudo2 { e012: self.0 }
    }
}

impl<T> Add<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn add(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 + rhs)
    }
}

impl<T> Sub<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn sub(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 - rhs)
    }
}

impl<T> Mul<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn mul(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 * rhs)
    }
}

impl<T> MulAssign<T> for Scalar2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T> Div<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn div(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 / rhs)
    }
}

impl<T> DivAssign<T> for Scalar2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T> Add<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn add(self, rhs: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 + rhs.0)
    }
}

impl<T> Sub<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn sub(self, rhs: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 - rhs.0)
    }
}

impl<T> BitOr<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> Scalar2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn bitor(self, other: Vector2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitOr<BiVector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn bitor(self, other: BiVector2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitOr<Pseudo2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn bitor(self, other: Pseudo2<T>) -> Pseudo2<T> {
        self * other
    }
}

impl<T> BitXor<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn bitxor(self, other: Scalar2<T>) -> Scalar2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn bitxor(self, other: Vector2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitXor<BiVector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn bitxor(self, other: BiVector2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitXor<Pseudo2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn bitxor(self, other: Pseudo2<T>) -> Pseudo2<T> {
        self * other
    }
}

impl<T> Mul<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn mul(self, other: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 * other.0)
    }
}

impl<T> MulAssign<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scalar2<T>) {
        self.0 *= rhs.0;
    }
}

impl<T> Mul<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.0 * other.e0,
            e1: self.0 * other.e1,
            e2: self.0 * other.e2,
        }
    }
}

impl<T> Mul<BiVector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn mul(self, other: BiVector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.0 * other.e01,
            e20: self.0 * other.e20,
            e12: self.0 * other.e12,
        }
    }
}

impl<T> Mul<Pseudo2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn mul(self, other: Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.0 * other.e012,
        }
    }
}

impl<T> Div<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn div(self, other: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 / other.0)
    }
}

impl<T> DivAssign<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scalar2<T>) {
        self.0 /= rhs.0;
    }
}
