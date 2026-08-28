use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{
    BiVector3, Dual, EBiVector3, ETriVector3, EVector3, Scalar3, TriVector3, Vector3, XBiVector3,
    XTriVector3, XVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Pseudo3<T> {
    pub e0123: T,
}

impl<T> Pseudo3<T> {
    #[inline]
    pub const fn new(e0123: T) -> Self {
        Self { e0123 }
    }
}

impl<T> Pseudo3<T>
where
    T: Num,
{
    pub const ZERO: Self = Self { e0123: T::ZERO };
}

impl<T> Neg for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn neg(self) -> Pseudo3<T> {
        Pseudo3 { e0123: -self.e0123 }
    }
}

impl<T> Not for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn not(self) -> Pseudo3<T> {
        self
    }
}

impl<T> Dual for Pseudo3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn dual(self) -> Scalar3<T> {
        Scalar3(self.e0123)
    }
}

impl<T> Mul<T> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 * rhs,
        }
    }
}

impl<T> MulAssign<T> for Pseudo3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e0123 *= rhs;
    }
}

impl<T> Div<T> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn div(self, rhs: T) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 / rhs,
        }
    }
}

impl<T> DivAssign<T> for Pseudo3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e0123 /= rhs;
    }
}

impl<T> Add<Pseudo3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn add(self, rhs: Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 + rhs.e0123,
        }
    }
}

impl<T> Sub<Pseudo3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn sub(self, rhs: Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 - rhs.e0123,
        }
    }
}

impl<T> BitOr<T> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitor(self, other: T) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 * other,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> BitXor<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e0123 * other.e3),
            e013: -(self.e0123 * other.e2),
            e032: -(self.e0123 * other.e1),
            e123: T::ZERO,
        }
    }
}

impl<T> BitOr<EVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: -(self.e0123 * other.e3),
            e013: -(self.e0123 * other.e2),
            e032: -(self.e0123 * other.e1),
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e0123 * other.e23),
            e02: -(self.e0123 * other.e31),
            e03: -(self.e0123 * other.e12),
        }
    }
}

impl<T> BitOr<BiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e0123 * other.e23),
            e02: -(self.e0123 * other.e31),
            e03: -(self.e0123 * other.e12),
        }
    }
}

impl<T> BitOr<TriVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e0123 * other.e123),
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> BitOr<ETriVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: -(self.e0123 * other.e123),
        }
    }
}

impl<T> Mul<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e0123 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e0123 * other.e3),
            e013: -(self.e0123 * other.e2),
            e032: -(self.e0123 * other.e1),
            e123: T::ZERO,
        }
    }
}

impl<T> Mul<EVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: EVector3<T>) -> XTriVector3<T> {
        self | other
    }
}

impl<T> Mul<EBiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e0123 * other.e23),
            e02: -(self.e0123 * other.e31),
            e03: -(self.e0123 * other.e12),
        }
    }
}

impl<T> Mul<BiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: BiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e0123 * other.e23),
            e02: -(self.e0123 * other.e31),
            e03: -(self.e0123 * other.e12),
        }
    }
}

impl<T> Mul<TriVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e0123 * other.e123),
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> Mul<ETriVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> XVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e0123 /= other.0;
    }
}
