use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{
    scalar::Scalar3, BiVector3, Dual, EBiVector3, ETriVector3, Pseudo3, TriVector3, XBiVector3,
    XTriVector3,
};

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
    #[inline]
    pub const fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        Vector3 { e0, e1, e2, e3 }
    }

    #[inline]
    pub fn is_near_zero(&self) -> bool
    where
        T: Num,
    {
        let epsilon = T::EPSILON;
        self.e0.abs() < epsilon
            && self.e1.abs() < epsilon
            && self.e2.abs() < epsilon
            && self.e3.abs() < epsilon
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

    #[inline]
    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }

    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Scalar3<T> {
        Scalar3(self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3)
    }
}

impl<T> BitOr<XBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e3 * other.e013 - self.e2 * other.e021,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e03: self.e2 * other.e032 - self.e1 * other.e013,
            e12: self.e3 * other.e123,
            e31: self.e2 * other.e123,
            e23: self.e1 * other.e123,
        }
    }
}

impl<T> BitOr<Pseudo3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e3 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
            e123: T::ZERO,
        }
    }
}

impl<T> BitOr<XTriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: XTriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e3 * other.e013 - self.e2 * other.e021,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e03: self.e2 * other.e032 - self.e1 * other.e013,
        }
    }
}

impl<T> BitOr<ETriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e3 * other.e123,
            e31: self.e2 * other.e123,
            e23: self.e1 * other.e123,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> Vector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitxor(self, other: TriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0 * other.e123
                + self.e1 * other.e032
                + self.e2 * other.e013
                + self.e3 * other.e021,
        }
    }
}

impl<T> BitXor<XTriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XTriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl<T> BitXor<ETriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: ETriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0 * other.e123,
        }
    }
}

impl<T> Mul<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
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

    #[inline]
    fn mul(self, other: Vector3<T>) -> (Scalar3<T>, BiVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<TriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XTriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: XTriVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<ETriVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (EBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> (EBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Pseudo3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> TriVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
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
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e0 /= other.0;
        self.e1 /= other.0;
        self.e2 /= other.0;
        self.e3 /= other.0;
    }
}

impl<T> Add<XVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: XVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 + other.e0,
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
        }
    }
}

impl<T> Add<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: EVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e3: self.e3 + other.e3,
        }
    }
}

impl<T> Sub<XVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: XVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 - other.e0,
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
        }
    }
}

impl<T> Sub<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: EVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e3: self.e3 - other.e3,
        }
    }
}

impl<T> BitOr<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> Scalar3<T> {
        Scalar3(self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3)
    }
}

impl<T> BitXor<XVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e1 * other.e0),
            e02: -(self.e2 * other.e0),
            e03: -(self.e3 * other.e0),
        }
    }
}

impl<T> BitXor<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e0 * other.e1,
            e02: self.e0 * other.e2,
            e03: self.e0 * other.e3,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e31: self.e3 * other.e1 - self.e1 * other.e3,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
        }
    }
}

impl<T> Mul<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (Scalar3<T>, BiVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XVector3<T>> for Vector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> XBiVector3<T> {
        self ^ other
    }
}

/// Ideal (moment-only) part of a 3D vector (plane offset weight).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct XVector3<T> {
    /// Offset component.
    pub e0: T,
}

impl<T> XVector3<T> {
    /// Creates a new ideal vector from its offset component.
    #[inline]
    pub const fn new(e0: T) -> Self {
        XVector3 { e0 }
    }
}

impl<T> XVector3<T>
where
    T: Num,
{
    /// The zero ideal vector.
    pub const ZERO: Self = Self { e0: T::ZERO };
}

impl<T> Neg for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn neg(self) -> XVector3<T> {
        XVector3 { e0: -self.e0 }
    }
}

impl<T> Not for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn not(self) -> XVector3<T> {
        self
    }
}

impl<T> Dual for XVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn dual(self) -> ETriVector3<T> {
        ETriVector3 { e123: self.e0 }
    }
}

impl<T> Mul<T> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, rhs: T) -> XVector3<T> {
        XVector3 { e0: self.e0 * rhs }
    }
}

impl<T> MulAssign<T> for XVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e0 *= rhs;
    }
}

impl<T> Div<T> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn div(self, rhs: T) -> XVector3<T> {
        XVector3 { e0: self.e0 / rhs }
    }
}

impl<T> DivAssign<T> for XVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e0 /= rhs;
    }
}

impl<T> Add<XVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn add(self, other: XVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e0 + other.e0,
        }
    }
}

impl<T> Add<EVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: EVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0,
            e1: other.e1,
            e2: other.e2,
            e3: other.e3,
        }
    }
}

impl<T> Add<Vector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 + other.e0,
            e1: other.e1,
            e2: other.e2,
            e3: other.e3,
        }
    }
}

impl<T> Sub<XVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn sub(self, other: XVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e0 - other.e0,
        }
    }
}

impl<T> Sub<EVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: EVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0,
            e1: -other.e1,
            e2: -other.e2,
            e3: -other.e3,
        }
    }
}

impl<T> Sub<Vector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e0 - other.e0,
            e1: -other.e1,
            e2: -other.e2,
            e3: -other.e3,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> XVector3<T> {
        self * other
    }
}

impl<T> BitXor<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> XVector3<T> {
        self * other
    }
}

impl<T> BitXor<EVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e0 * other.e1,
            e02: self.e0 * other.e2,
            e03: self.e0 * other.e3,
        }
    }
}

impl<T> BitXor<Vector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e0 * other.e1,
            e02: self.e0 * other.e2,
            e03: self.e0 * other.e3,
        }
    }
}

impl<T> BitXor<EBiVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: EBiVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: -(self.e0 * other.e12),
            e013: -(self.e0 * other.e31),
            e032: -(self.e0 * other.e23),
        }
    }
}

impl<T> BitXor<BiVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: -(self.e0 * other.e12),
            e013: -(self.e0 * other.e31),
            e032: -(self.e0 * other.e23),
        }
    }
}

impl<T> BitXor<TriVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: TriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0 * other.e123,
        }
    }
}

impl<T> BitXor<ETriVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: ETriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e0 * other.e123,
        }
    }
}

impl<T> Mul<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e0 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e0 *= other.0;
    }
}

impl<T> Mul<EVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: EVector3<T>) -> XBiVector3<T> {
        self ^ other
    }
}

impl<T> Mul<Vector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: Vector3<T>) -> XBiVector3<T> {
        self ^ other
    }
}

impl<T> Mul<XBiVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, _other: XBiVector3<T>) -> XVector3<T> {
        XVector3::ZERO
    }
}

impl<T> Mul<EBiVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> XTriVector3<T> {
        self ^ other
    }
}

impl<T> Mul<BiVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: BiVector3<T>) -> XTriVector3<T> {
        self ^ other
    }
}

impl<T> Mul<TriVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: TriVector3<T>) -> Pseudo3<T> {
        self ^ other
    }
}

impl<T> Mul<ETriVector3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> Pseudo3<T> {
        self ^ other
    }
}

impl<T> Div<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e0 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for XVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e0 /= other.0;
    }
}

/// Euclidean (direction-only) part of a 3D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EVector3<T> {
    pub e1: T,
    pub e2: T,
    pub e3: T,
}

impl<T> EVector3<T> {
    #[inline]
    pub const fn new(e1: T, e2: T, e3: T) -> Self {
        EVector3 { e1, e2, e3 }
    }
}

impl<T> EVector3<T>
where
    T: Num,
{
    pub const ZERO: Self = Self {
        e1: T::ZERO,
        e2: T::ZERO,
        e3: T::ZERO,
    };

    #[inline]
    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3
    }

    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e1 /= norm;
            self.e2 /= norm;
            self.e3 /= norm;
        }
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut vector = *self;
        vector.normalize();
        vector
    }
}

impl<T> Neg for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn neg(self) -> EVector3<T> {
        EVector3 {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl<T> Not for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn not(self) -> EVector3<T> {
        self
    }
}

impl<T> Dual for EVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn dual(self) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e3,
            e013: self.e2,
            e032: self.e1,
        }
    }
}

impl<T> Mul<T> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn mul(self, rhs: T) -> EVector3<T> {
        EVector3 {
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
            e3: self.e3 * rhs,
        }
    }
}

impl<T> MulAssign<T> for EVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e1 *= rhs;
        self.e2 *= rhs;
        self.e3 *= rhs;
    }
}

impl<T> Div<T> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn div(self, rhs: T) -> EVector3<T> {
        EVector3 {
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
            e3: self.e3 / rhs,
        }
    }
}

impl<T> DivAssign<T> for EVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e1 /= rhs;
        self.e2 /= rhs;
        self.e3 /= rhs;
    }
}

impl<T> Add<XVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: XVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: other.e0,
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
        }
    }
}

impl<T> Add<EVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn add(self, other: EVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e3: self.e3 + other.e3,
        }
    }
}

impl<T> Add<Vector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
            e3: self.e3 + other.e3,
        }
    }
}

impl<T> Sub<XVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: XVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -other.e0,
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
        }
    }
}

impl<T> Sub<EVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn sub(self, other: EVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e3: self.e3 - other.e3,
        }
    }
}

impl<T> Sub<Vector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
            e3: self.e3 - other.e3,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> EVector3<T> {
        self * other
    }
}

impl<T> BitOr<EVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> Scalar3<T> {
        Scalar3(self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3)
    }
}

impl<T> BitOr<Vector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Scalar3<T> {
        Scalar3(self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3)
    }
}

impl<T> BitOr<XBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: XBiVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e3 * other.e31 - self.e2 * other.e12,
            e2: self.e1 * other.e12 - self.e3 * other.e23,
            e3: self.e2 * other.e23 - self.e1 * other.e31,
        }
    }
}

impl<T> BitOr<BiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            e1: self.e3 * other.e31 - self.e2 * other.e12,
            e2: self.e1 * other.e12 - self.e3 * other.e23,
            e3: self.e2 * other.e23 - self.e1 * other.e31,
        }
    }
}

impl<T> BitOr<TriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e3 * other.e013 - self.e2 * other.e021,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e03: self.e2 * other.e032 - self.e1 * other.e013,
            e12: self.e3 * other.e123,
            e31: self.e2 * other.e123,
            e23: self.e1 * other.e123,
        }
    }
}

impl<T> BitOr<XTriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: XTriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e3 * other.e013 - self.e2 * other.e021,
            e02: self.e1 * other.e021 - self.e3 * other.e032,
            e03: self.e2 * other.e032 - self.e1 * other.e013,
        }
    }
}

impl<T> BitOr<ETriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e3 * other.e123,
            e31: self.e2 * other.e123,
            e23: self.e1 * other.e123,
        }
    }
}

impl<T> BitOr<Pseudo3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e3 * other.e0123,
            e013: self.e2 * other.e0123,
            e032: self.e1 * other.e0123,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> EVector3<T> {
        self * other
    }
}

impl<T> BitXor<XVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e1 * other.e0),
            e02: -(self.e2 * other.e0),
            e03: -(self.e3 * other.e0),
        }
    }
}

impl<T> BitXor<EVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e31: self.e3 * other.e1 - self.e1 * other.e3,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
        }
    }
}

impl<T> BitXor<Vector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: -(self.e1 * other.e0),
            e02: -(self.e2 * other.e0),
            e03: -(self.e3 * other.e0),
            e12: self.e1 * other.e2 - self.e2 * other.e1,
            e31: self.e3 * other.e1 - self.e1 * other.e3,
            e23: self.e2 * other.e3 - self.e3 * other.e2,
        }
    }
}

impl<T> BitXor<XBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: XBiVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e1 * other.e02 - self.e2 * other.e01,
            e013: self.e3 * other.e01 - self.e1 * other.e03,
            e032: self.e2 * other.e03 - self.e3 * other.e02,
        }
    }
}

impl<T> BitXor<EBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitxor(self, other: EBiVector3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl<T> BitXor<BiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e1 * other.e02 - self.e2 * other.e01,
            e013: self.e3 * other.e01 - self.e1 * other.e03,
            e032: self.e2 * other.e03 - self.e3 * other.e02,
            e123: self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        }
    }
}

impl<T> BitXor<TriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: TriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl<T> BitXor<XTriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XTriVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e1 * other.e032 + self.e2 * other.e013 + self.e3 * other.e021,
        }
    }
}

impl<T> Mul<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e1 * other.0,
            e2: self.e2 * other.0,
            e3: self.e3 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e1 *= other.0;
        self.e2 *= other.0;
        self.e3 *= other.0;
    }
}

impl<T> Mul<XVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> XBiVector3<T> {
        self ^ other
    }
}

impl<T> Mul<EVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, EBiVector3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (Scalar3<T>, EBiVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Vector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (Scalar3<T>, BiVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (XVector3<T>, XTriVector3<T>);

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> (XVector3<T>, XTriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (EVector3<T>, ETriVector3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (EVector3<T>, ETriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<TriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XTriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: XTriVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<ETriVector3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> EBiVector3<T> {
        self | other
    }
}

impl<T> Mul<Pseudo3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> XTriVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e1 / other.0,
            e2: self.e2 / other.0,
            e3: self.e3 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for EVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e1 /= other.0;
        self.e2 /= other.0;
        self.e3 /= other.0;
    }
}

impl<T> From<XVector3<T>> for Vector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: XVector3<T>) -> Self {
        Vector3 {
            e0: x.e0,
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> From<EVector3<T>> for Vector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: EVector3<T>) -> Self {
        Vector3 {
            e0: T::ZERO,
            e1: x.e1,
            e2: x.e2,
            e3: x.e3,
        }
    }
}
