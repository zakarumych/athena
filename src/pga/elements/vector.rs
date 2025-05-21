use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::Num;

use super::{
    scalar::{Scalar2, Scalar3},
    BiVector2, BiVector3, Dual, EBiVector3, Pseudo2, Pseudo3, TriVector3, XBiVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector2<T> {
    pub e0: T,
    pub e1: T,
    pub e2: T,
}

impl<T> Vector2<T> {
    pub const fn new(e0: T, e1: T, e2: T) -> Self {
        Vector2 { e0, e1, e2 }
    }
}

impl<T> Vector2<T>
where
    T: Num,
{
    pub const ZERO: Self = Vector2 {
        e0: T::ZERO,
        e1: T::ZERO,
        e2: T::ZERO,
    };

    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2
    }

    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e0 /= norm;
            self.e1 /= norm;
            self.e2 /= norm;
        }
    }

    pub fn normalized(&self) -> Self {
        let mut vector = *self;
        vector.normalize();
        vector
    }
}

impl<T> Neg for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn neg(self) -> Vector2<T> {
        Vector2 {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl<T> Not for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn not(self) -> Vector2<T> {
        self
    }
}

impl<T> Dual for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn dual(self) -> BiVector2<T> {
        BiVector2 {
            e01: self.e2,
            e20: self.e1,
            e12: self.e0,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2 {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: T) {
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Vector2<T> {
        Vector2 {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector2<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: T) {
        self.e0 /= rhs;
        self.e1 /= rhs;
        self.e2 /= rhs;
    }
}

impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl<T> Sub<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl<T> BitOr<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitor(self, other: Scalar2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn bitor(self, other: Vector2<T>) -> Scalar2<T> {
        Scalar2(self.e1 * other.e1 + self.e2 * other.e2)
    }
}

impl<T> BitOr<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitor(self, other: BiVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e2 * other.e20 - self.e1 * other.e01,
            e1: -(self.e2 * other.e12),
            e2: self.e1 * other.e12,
        }
    }
}

impl<T> BitOr<Pseudo2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn bitor(self, other: Pseudo2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e2 * other.e012,
            e20: self.e1 * other.e012,
            e12: T::ZERO,
        }
    }
}

impl<T> BitXor<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitxor(self, other: Scalar2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;
    fn bitxor(self, other: Vector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e20: self.e2 * other.e0 - self.e0 * other.e2,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
        }
    }
}

impl<T> BitXor<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    fn bitxor(self, other: BiVector2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e0 * other.e12 + self.e1 * other.e20 + self.e2 * other.e01,
        }
    }
}

impl<T> Mul<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn mul(self, other: Scalar2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 * other.0,
            e1: self.e1 * other.0,
            e2: self.e2 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e0 *= other.0;
        self.e1 *= other.0;
        self.e2 *= other.0;
    }
}

impl<T> Mul<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = (Scalar2<T>, BiVector2<T>);

    fn mul(self, other: Vector2<T>) -> (Scalar2<T>, BiVector2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = (Vector2<T>, Pseudo2<T>);

    fn mul(self, other: BiVector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Pseudo2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn mul(self, other: Pseudo2<T>) -> BiVector2<T> {
        self | other
    }
}

impl<T> Div<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn div(self, other: Scalar2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 / other.0,
            e1: self.e1 / other.0,
            e2: self.e2 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e0 /= other.0;
        self.e1 /= other.0;
        self.e2 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector3<T> {
    pub e0: T,
    pub e1: T,
    pub e2: T,
    pub e3: T,
}

impl<T> Vector3<T> {
    pub const fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        Vector3 { e0, e1, e2, e3 }
    }
}

impl<T> Vector3<T>
where
    T: Num,
{
    pub const ZERO: Self = Vector3 {
        e0: T::ZERO,
        e1: T::ZERO,
        e2: T::ZERO,
        e3: T::ZERO,
    };

    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }

    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e0 /= norm;
            self.e1 /= norm;
            self.e2 /= norm;
            self.e3 /= norm;
        }
    }

    pub fn normalized(&self) -> Self {
        let mut vector = *self;
        vector.normalize();
        vector
    }
}

impl<T> Neg for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn neg(self) -> Vector3<T> {
        Vector3 {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl<T> Not for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn not(self) -> Vector3<T> {
        self
    }
}

impl<T> Dual for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn dual(self) -> TriVector3<T> {
        TriVector3 {
            e021: self.e3,
            e013: self.e2,
            e032: self.e1,
            e123: self.e0,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Vector3<T> {
        Vector3 {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
            e3: self.e3 * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: Num,
{
    fn mul_assign(&mut self, rhs: T) {
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
        self.e3 *= rhs;
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Vector3<T> {
        Vector3 {
            e0: self.e0 / rhs,
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
            e3: self.e3 / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector3<T>
where
    T: Num,
{
    fn div_assign(&mut self, rhs: T) {
        self.e0 /= rhs;
        self.e1 /= rhs;
        self.e2 /= rhs;
        self.e3 /= rhs;
    }
}

impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e3: self.e3 + other.e3,
        }
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e3: self.e3 - other.e3,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: Scalar3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;
    fn bitor(self, other: Vector3<T>) -> Scalar3<T> {
        Scalar3(self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3)
    }
}

impl<T> BitOr<XBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: XBiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: EBiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: T::ZERO,
            e1: self.e3 * other.e31 - self.e2 * other.e12,
            e2: self.e1 * other.e12 - self.e3 * other.e23,
            e3: self.e2 * other.e23 - self.e1 * other.e31,
        }
    }
}

impl<T> BitOr<BiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: BiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            e1: self.e3 * other.e31 - self.e2 * other.e12,
            e2: self.e1 * other.e12 - self.e3 * other.e23,
            e3: self.e2 * other.e23 - self.e1 * other.e31,
        }
    }
}

impl<T> BitOr<TriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;
    fn bitor(self, other: TriVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e3 * other.e013 - self.e2 * other.e021,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e03: self.e2 * other.e032 - self.e1 * other.e013,
            e12: self.e1 * other.e123,
            e31: self.e2 * other.e123,
            e23: self.e3 * other.e123,
        }
    }
}

impl<T> BitOr<Pseudo3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
    fn bitor(self, other: Pseudo3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e3 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: T::ZERO,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;
    fn bitxor(self, other: Scalar3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;
    fn bitxor(self, other: Vector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e02: self.e0 * other.e2 - self.e2 * other.e0,
            e03: self.e0 * other.e3 - self.e3 * other.e0,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e31: self.e3 * other.e1 - self.e1 * other.e3,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
        }
    }
}

impl<T> BitXor<XBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
    fn bitxor(self, other: XBiVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e1 * other.e02 - self.e2 * other.e01,
            e013: self.e3 * other.e01 - self.e1 * other.e03,
            e032: self.e2 * other.e03 - self.e3 * other.e02,
            e123: T::ZERO,
        }
    }
}

impl<T> BitXor<EBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
    fn bitxor(self, other: EBiVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e0 * other.e12),
            e013: -(self.e0 * other.e31),
            e032: -(self.e0 * other.e23),
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl<T> BitXor<BiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
    fn bitxor(self, other: BiVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e1 * other.e02 - self.e2 * other.e01 - self.e0 * other.e12,
            e013: self.e3 * other.e01 - self.e1 * other.e03 - self.e0 * other.e31,
            e032: self.e2 * other.e03 - self.e3 * other.e02 - self.e0 * other.e23,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl<T> BitXor<TriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    fn bitxor(self, other: TriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0 * other.e123
                + self.e1 * other.e032
                + self.e2 * other.e013
                + self.e3 * other.e021,
        }
    }
}

impl<T> Mul<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;
    fn mul(self, other: Scalar3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 * other.0,
            e1: self.e1 * other.0,
            e2: self.e2 * other.0,
            e3: self.e3 * other.0,
        }
    }
}

impl<T> Mul<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>);
    fn mul(self, other: Vector3<T>) -> (Scalar3<T>, BiVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn mul(self, other: XBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn mul(self, other: EBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn mul(self, other: BiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<TriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);
    fn mul(self, other: TriVector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Pseudo3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
    fn mul(self, other: Pseudo3<T>) -> TriVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn div(self, other: Scalar3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 / other.0,
            e1: self.e1 / other.0,
            e2: self.e2 / other.0,
            e3: self.e3 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e0 /= other.0;
        self.e1 /= other.0;
        self.e2 /= other.0;
        self.e3 /= other.0;
    }
}
