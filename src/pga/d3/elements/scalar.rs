use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{
    BiVector3, Dual, EBiVector3, ETriVector3, EVector3, Pseudo3, TriVector3, Vector3, XBiVector3,
    XTriVector3, XVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Scalar3<T>(pub T);

impl<T> Scalar3<T> {
    #[inline]
    pub const fn new(s: T) -> Self {
        Scalar3(s)
    }

    #[inline]
    pub fn is_near_zero(&self) -> bool
    where
        T: Num,
    {
        let epsilon = T::EPSILON;
        self.0.abs() < epsilon
    }
}

impl<T> Neg for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn neg(self) -> Scalar3<T> {
        Scalar3(-self.0)
    }
}

impl<T> Not for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn not(self) -> Scalar3<T> {
        self
    }
}

impl<T> Dual for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn dual(self) -> Pseudo3<T> {
        Pseudo3 { e0123: self.0 }
    }
}

impl<T> Add<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn add(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 + rhs)
    }
}

impl<T> Sub<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn sub(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 - rhs)
    }
}

impl<T> Mul<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 * rhs)
    }
}

impl<T> MulAssign<T> for Scalar3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T> Div<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn div(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 / rhs)
    }
}

impl<T> DivAssign<T> for Scalar3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T> Add<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn add(self, rhs: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 + rhs.0)
    }
}

impl<T> Sub<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn sub(self, rhs: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 - rhs.0)
    }
}

impl<T> BitOr<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> Scalar3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitOr<XVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: XVector3<T>) -> XVector3<T> {
        self * other
    }
}

impl<T> BitOr<EVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> EVector3<T> {
        self * other
    }
}

impl<T> BitOr<XBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: XBiVector3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<EBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<BiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitOr<XTriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitor(self, other: XTriVector3<T>) -> XTriVector3<T> {
        self * other
    }
}

impl<T> BitOr<ETriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> ETriVector3<T> {
        self * other
    }
}

impl<T> BitOr<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Pseudo3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> BitXor<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> Scalar3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitXor<XVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> XVector3<T> {
        self * other
    }
}

impl<T> BitXor<EVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> EVector3<T> {
        self * other
    }
}

impl<T> BitXor<XBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitxor(self, other: XBiVector3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<EBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitxor(self, other: EBiVector3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<BiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitXor<XTriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: XTriVector3<T>) -> XTriVector3<T> {
        self * other
    }
}

impl<T> BitXor<ETriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitxor(self, other: ETriVector3<T>) -> ETriVector3<T> {
        self * other
    }
}

impl<T> BitXor<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: TriVector3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitXor<Pseudo3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: Pseudo3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> Mul<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 * other.0)
    }
}

impl<T> MulAssign<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Scalar3<T>) {
        self.0 *= rhs.0;
    }
}

impl<T> Mul<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.0 * other.e0,
            e1: self.0 * other.e1,
            e2: self.0 * other.e2,
            e3: self.0 * other.e3,
        }
    }
}

impl<T> Mul<XVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.0 * other.e0,
        }
    }
}

impl<T> Mul<EVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn mul(self, other: EVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.0 * other.e1,
            e2: self.0 * other.e2,
            e3: self.0 * other.e3,
        }
    }
}

impl<T> Mul<XBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.0 * other.e01,
            e02: self.0 * other.e02,
            e03: self.0 * other.e03,
        }
    }
}

impl<T> Mul<EBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.0 * other.e12,
            e31: self.0 * other.e31,
            e23: self.0 * other.e23,
        }
    }
}

impl<T> Mul<BiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn mul(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.0 * other.e01,
            e02: self.0 * other.e02,
            e03: self.0 * other.e03,
            e12: self.0 * other.e12,
            e31: self.0 * other.e31,
            e23: self.0 * other.e23,
        }
    }
}

impl<T> Mul<XTriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: XTriVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.0 * other.e021,
            e013: self.0 * other.e013,
            e032: self.0 * other.e032,
        }
    }
}

impl<T> Mul<ETriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.0 * other.e123,
        }
    }
}

impl<T> Mul<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.0 * other.e021,
            e013: self.0 * other.e013,
            e032: self.0 * other.e032,
            e123: self.0 * other.e123,
        }
    }
}

impl<T> Mul<Pseudo3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.0 * other.e0123,
        }
    }
}

impl<T> Div<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 / other.0)
    }
}

impl<T> DivAssign<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: Scalar3<T>) {
        self.0 /= rhs.0;
    }
}
