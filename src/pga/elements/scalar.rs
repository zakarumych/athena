use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::Num;

use super::{
    BiVector2, BiVector3, Dual, EBiVector3, Pseudo2, Pseudo3, TriVector3, Vector2, Vector3,
    XBiVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Scalar2<T>(pub T);

impl<T> Scalar2<T> {
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

    fn add(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 + rhs)
    }
}

impl<T> Sub<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn sub(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 - rhs)
    }
}

impl<T> Mul<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn mul(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 * rhs)
    }
}

impl<T> MulAssign<T> for Scalar2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T> Div<T> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn div(self, rhs: T) -> Scalar2<T> {
        Scalar2(self.0 / rhs)
    }
}

impl<T> DivAssign<T> for Scalar2<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T> Add<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn add(self, rhs: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 + rhs.0)
    }
}

impl<T> Sub<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn sub(self, rhs: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 - rhs.0)
    }
}

impl<T> BitOr<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn bitor(self, other: Scalar2<T>) -> Scalar2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitor(self, other: Vector2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitOr<BiVector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn bitor(self, other: BiVector2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitOr<Pseudo2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    fn bitor(self, other: Pseudo2<T>) -> Pseudo2<T> {
        self * other
    }
}

impl<T> BitXor<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn bitxor(self, other: Scalar2<T>) -> Scalar2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitxor(self, other: Vector2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitXor<BiVector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn bitxor(self, other: BiVector2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitXor<Pseudo2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    fn bitxor(self, other: Pseudo2<T>) -> Pseudo2<T> {
        self * other
    }
}

impl<T> Mul<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn mul(self, other: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 * other.0)
    }
}

impl<T> MulAssign<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: Scalar2<T>) {
        self.0 *= rhs.0;
    }
}

impl<T> Mul<Vector2<T>> for Scalar2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

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

    fn div(self, other: Scalar2<T>) -> Scalar2<T> {
        Scalar2(self.0 / other.0)
    }
}

impl<T> DivAssign<Scalar2<T>> for Scalar2<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: Scalar2<T>) {
        self.0 /= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Scalar3<T>(pub T);

impl<T> Scalar3<T> {
    pub const fn new(s: T) -> Self {
        Scalar3(s)
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

    fn add(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 + rhs)
    }
}

impl<T> Sub<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn sub(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 - rhs)
    }
}

impl<T> Mul<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn mul(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 * rhs)
    }
}

impl<T> MulAssign<T> for Scalar3<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
    }
}

impl<T> Div<T> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn div(self, rhs: T) -> Scalar3<T> {
        Scalar3(self.0 / rhs)
    }
}

impl<T> DivAssign<T> for Scalar3<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
    }
}

impl<T> Add<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn add(self, rhs: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 + rhs.0)
    }
}

impl<T> Sub<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn sub(self, rhs: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 - rhs.0)
    }
}

impl<T> BitOr<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn bitor(self, other: Scalar3<T>) -> Scalar3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitOr<XBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    fn bitor(self, other: XBiVector3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<EBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    fn bitor(self, other: EBiVector3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<BiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    fn bitor(self, other: BiVector3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitOr<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    fn bitor(self, other: TriVector3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Pseudo3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    fn bitor(self, other: Pseudo3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> BitXor<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn bitxor(self, other: Scalar3<T>) -> Scalar3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitxor(self, other: Vector3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitXor<XBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    fn bitxor(self, other: XBiVector3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<EBiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    fn bitxor(self, other: EBiVector3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<BiVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    fn bitxor(self, other: BiVector3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitXor<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    fn bitxor(self, other: TriVector3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitXor<Pseudo3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    fn bitxor(self, other: Pseudo3<T>) -> Pseudo3<T> {
        self * other
    }
}

impl<T> Mul<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn mul(self, other: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 * other.0)
    }
}

impl<T> MulAssign<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: Scalar3<T>) {
        self.0 *= rhs.0;
    }
}

impl<T> Mul<Vector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.0 * other.e0,
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

impl<T> Mul<TriVector3<T>> for Scalar3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

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

    fn div(self, other: Scalar3<T>) -> Scalar3<T> {
        Scalar3(self.0 / other.0)
    }
}

impl<T> DivAssign<Scalar3<T>> for Scalar3<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: Scalar3<T>) {
        self.0 /= rhs.0;
    }
}
