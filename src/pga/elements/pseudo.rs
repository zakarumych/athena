use core::ops::{Add, BitOr, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::Num;

use super::{
    BiVector2, BiVector3, Dual, EBiVector3, Scalar2, Scalar3, TriVector3, Vector2, Vector3,
    XBiVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Pseudo2<T> {
    pub e012: T,
}

impl<T> Pseudo2<T> {
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
    fn mul_assign(&mut self, rhs: T) {
        self.e012 *= rhs;
    }
}

impl<T> Div<T> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

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
    fn div_assign(&mut self, rhs: T) {
        self.e012 /= rhs;
    }
}

impl<T> Add<Pseudo2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

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
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e012 *= other.0;
    }
}

impl<T> Mul<Vector2<T>> for Pseudo2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

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
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e012 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Pseudo3<T> {
    pub e0123: T,
}

impl<T> Pseudo3<T> {
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
    fn mul_assign(&mut self, rhs: T) {
        self.e0123 *= rhs;
    }
}

impl<T> Div<T> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

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
    fn div_assign(&mut self, rhs: T) {
        self.e0123 /= rhs;
    }
}

impl<T> Add<Pseudo3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

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

    fn bitor(self, other: T) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0123 * other,
        }
    }
}

impl<T> BitOr<Vector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    fn bitor(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e0123 * other.e3),
            e013: -(self.e0123 * other.e2),
            e032: -(self.e0123 * other.e1),
            e123: T::ZERO,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

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

    fn bitor(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e0123 * other.e123),
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> Mul<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

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
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e0123 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    fn mul(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e0123 * other.e3),
            e013: -(self.e0123 * other.e2),
            e032: -(self.e0123 * other.e1),
            e123: T::ZERO,
        }
    }
}

impl<T> Mul<EBiVector3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

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

    fn mul(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e0123 * other.e123),
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> Div<Scalar3<T>> for Pseudo3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

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
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e0123 /= other.0;
    }
}
