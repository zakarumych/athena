use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{
    scalar::Scalar3, BiVector3, Dual, EBiVector3, EVector3, Pseudo3, Vector3, XBiVector3, XVector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriVector3<T> {
    pub e021: T,
    pub e013: T,
    pub e032: T,
    pub e123: T,
}

impl<T> TriVector3<T> {
    #[inline]
    pub const fn new(e021: T, e013: T, e032: T, e123: T) -> Self {
        TriVector3 {
            e021,
            e013,
            e032,
            e123,
        }
    }
}

impl<T> TriVector3<T>
where
    T: Num,
{
    pub const ZERO: Self = TriVector3 {
        e021: T::ZERO,
        e013: T::ZERO,
        e032: T::ZERO,
        e123: T::ZERO,
    };

    #[inline]
    pub fn norm2(&self) -> T {
        self.e123 * self.e123
    }

    #[inline]
    pub fn norm(&self) -> T {
        self.e123.abs()
    }

    #[inline]
    pub fn signed_norm(&self) -> T {
        self.e123
    }

    #[inline]
    pub fn normalize(&mut self) {
        let norm = self.signed_norm();
        if norm != T::ZERO {
            self.e021 /= norm;
            self.e013 /= norm;
            self.e032 /= norm;
            self.e123 = T::ONE;
        } else {
            let s = norm.sign();
            let m = (self.e021 * self.e021
                + self.e013 * self.e013
                + self.e032 * self.e032
                + self.e123 * self.e123)
                .sqrt()
                .recip()
                * s;

            self.e021 *= m;
            self.e013 *= m;
            self.e032 *= m;
            self.e123 = T::ZERO;
        }
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut trivector = *self;
        trivector.normalize();
        trivector
    }
}

impl<T> Neg for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn neg(self) -> TriVector3<T> {
        TriVector3 {
            e021: -self.e021,
            e013: -self.e013,
            e032: -self.e032,
            e123: -self.e123,
        }
    }
}

impl<T> Not for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn not(self) -> TriVector3<T> {
        -self
    }
}

impl<T> Dual for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn dual(self) -> Vector3<T> {
        Vector3 {
            e0: self.e123,
            e1: self.e032,
            e2: self.e013,
            e3: self.e021,
        }
    }
}

impl<T> Mul<T> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, rhs: T) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 * rhs,
            e013: self.e013 * rhs,
            e032: self.e032 * rhs,
            e123: self.e123 * rhs,
        }
    }
}

impl<T> MulAssign<T> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e021 *= rhs;
        self.e013 *= rhs;
        self.e032 *= rhs;
        self.e123 *= rhs;
    }
}

impl<T> Div<T> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn div(self, rhs: T) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 / rhs,
            e013: self.e013 / rhs,
            e032: self.e032 / rhs,
            e123: self.e123 / rhs,
        }
    }
}

impl<T> DivAssign<T> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e021 /= rhs;
        self.e013 /= rhs;
        self.e032 /= rhs;
        self.e123 /= rhs;
    }
}

impl<T> Add<TriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, rhs: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 + rhs.e021,
            e013: self.e013 + rhs.e013,
            e032: self.e032 + rhs.e032,
            e123: self.e123 + rhs.e123,
        }
    }
}

impl<T> Sub<TriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, rhs: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 - rhs.e021,
            e013: self.e013 - rhs.e013,
            e032: self.e032 - rhs.e032,
            e123: self.e123 - rhs.e123,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e013 * other.e3 - self.e021 * other.e2,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e03: self.e032 * other.e2 - self.e013 * other.e1,
            e12: self.e123 * other.e3,
            e31: self.e123 * other.e2,
            e23: self.e123 * other.e1,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -(self.e123 * other.e23),
            e2: -(self.e123 * other.e31),
            e3: -(self.e123 * other.e12),
        }
    }
}

impl<T> BitOr<BiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -(self.e123 * other.e23),
            e2: -(self.e123 * other.e31),
            e3: -(self.e123 * other.e12),
        }
    }
}

impl<T> BitOr<EVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e013 * other.e3 - self.e021 * other.e2,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e03: self.e032 * other.e2 - self.e013 * other.e1,
            e12: self.e123 * other.e3,
            e31: self.e123 * other.e2,
            e23: self.e123 * other.e1,
        }
    }
}

impl<T> BitOr<TriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e123 * other.e123))
    }
}

impl<T> BitOr<ETriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e123 * other.e123))
    }
}

impl<T> BitOr<Pseudo3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e123 * other.e0123,
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitXor<XVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e123 * other.e0),
        }
    }
}

impl<T> BitXor<EVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e021 * other.e3 + self.e013 * other.e2 + self.e032 * other.e1),
        }
    }
}

impl<T> BitXor<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e021 * other.e3
                + self.e013 * other.e2
                + self.e032 * other.e1
                + self.e123 * other.e0),
        }
    }
}

impl<T> Mul<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 * other.0,
            e013: self.e013 * other.0,
            e032: self.e032 * other.0,
            e123: self.e123 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e021 *= other.0;
        self.e013 *= other.0;
        self.e032 *= other.0;
        self.e123 *= other.0;
    }
}

impl<T> Mul<XVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> Pseudo3<T> {
        self ^ other
    }
}

impl<T> Mul<EVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e123 * other.e03,
            e013: self.e123 * other.e02,
            e032: self.e123 * other.e01,
            e123: T::ZERO,
        }
    }
}

impl<T> Mul<EBiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = self | other;

        let triv = TriVector3 {
            e021: self.e013 * other.e23 - self.e032 * other.e31,
            e013: self.e032 * other.e12 - self.e021 * other.e23,
            e032: self.e021 * other.e31 - self.e013 * other.e12,
            e123: T::ZERO,
        };

        (vec, triv)
    }
}

impl<T> Mul<BiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = self | other;

        let triv = TriVector3 {
            e021: self.e013 * other.e23 - self.e032 * other.e31 + self.e123 * other.e03,
            e013: self.e032 * other.e12 - self.e021 * other.e23 + self.e123 * other.e02,
            e032: self.e021 * other.e31 - self.e013 * other.e12 + self.e123 * other.e01,
            e123: T::ZERO,
        };

        (vec, triv)
    }
}

impl<T> Mul<TriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, XBiVector3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (Scalar3<T>, XBiVector3<T>) {
        let scalar = self | other;

        let bivec = XBiVector3 {
            e01: self.e032 * other.e123 - self.e123 * other.e032,
            e02: self.e013 * other.e123 - self.e123 * other.e013,
            e03: self.e021 * other.e123 - self.e123 * other.e021,
        };

        (scalar, bivec)
    }
}

impl<T> Mul<XTriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: XTriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e123 * other.e032),
            e02: -(self.e123 * other.e013),
            e03: -(self.e123 * other.e021),
        }
    }
}

impl<T> Mul<ETriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, XBiVector3<T>);

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> (Scalar3<T>, XBiVector3<T>) {
        let scalar = self | other;

        let bivec = XBiVector3 {
            e01: self.e032 * other.e123,
            e02: self.e013 * other.e123,
            e03: self.e021 * other.e123,
        };

        (scalar, bivec)
    }
}

impl<T> Mul<Pseudo3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> Vector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 / other.0,
            e013: self.e013 / other.0,
            e032: self.e032 / other.0,
            e123: self.e123 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e021 /= other.0;
        self.e013 /= other.0;
        self.e032 /= other.0;
        self.e123 /= other.0;
    }
}

impl<T> Add<XTriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: XTriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 + other.e021,
            e013: self.e013 + other.e013,
            e032: self.e032 + other.e032,
            e123: self.e123,
        }
    }
}

impl<T> Add<ETriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: ETriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: self.e123 + other.e123,
        }
    }
}

impl<T> Sub<XTriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: XTriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 - other.e021,
            e013: self.e013 - other.e013,
            e032: self.e032 - other.e032,
            e123: self.e123,
        }
    }
}

impl<T> Sub<ETriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: ETriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: self.e123 - other.e123,
        }
    }
}

/// Ideal (moment-only) part of a 3D trivector (point at infinity).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XTriVector3<T> {
    /// Moment component paired with `e3`.
    pub e021: T,
    /// Moment component paired with `e2`.
    pub e013: T,
    /// Moment component paired with `e1`.
    pub e032: T,
}

impl<T> XTriVector3<T> {
    /// Creates a new ideal trivector from its moment components.
    #[inline]
    pub const fn new(e021: T, e013: T, e032: T) -> Self {
        XTriVector3 { e021, e013, e032 }
    }
}

impl<T> XTriVector3<T>
where
    T: Num,
{
    /// The zero ideal trivector.
    pub const ZERO: Self = Self {
        e021: T::ZERO,
        e013: T::ZERO,
        e032: T::ZERO,
    };
}

impl<T> Neg for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn neg(self) -> XTriVector3<T> {
        XTriVector3 {
            e021: -self.e021,
            e013: -self.e013,
            e032: -self.e032,
        }
    }
}

impl<T> Not for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn not(self) -> XTriVector3<T> {
        -self
    }
}

impl<T> Dual for XTriVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn dual(self) -> EVector3<T> {
        EVector3 {
            e1: self.e032,
            e2: self.e013,
            e3: self.e021,
        }
    }
}

impl<T> Mul<T> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, rhs: T) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 * rhs,
            e013: self.e013 * rhs,
            e032: self.e032 * rhs,
        }
    }
}

impl<T> MulAssign<T> for XTriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e021 *= rhs;
        self.e013 *= rhs;
        self.e032 *= rhs;
    }
}

impl<T> Div<T> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn div(self, rhs: T) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 / rhs,
            e013: self.e013 / rhs,
            e032: self.e032 / rhs,
        }
    }
}

impl<T> DivAssign<T> for XTriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e021 /= rhs;
        self.e013 /= rhs;
        self.e032 /= rhs;
    }
}

impl<T> Add<XTriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn add(self, other: XTriVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 + other.e021,
            e013: self.e013 + other.e013,
            e032: self.e032 + other.e032,
        }
    }
}

impl<T> Add<ETriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: ETriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: other.e123,
        }
    }
}

impl<T> Add<TriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 + other.e021,
            e013: self.e013 + other.e013,
            e032: self.e032 + other.e032,
            e123: other.e123,
        }
    }
}

impl<T> Sub<XTriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn sub(self, other: XTriVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 - other.e021,
            e013: self.e013 - other.e013,
            e032: self.e032 - other.e032,
        }
    }
}

impl<T> Sub<ETriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: ETriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021,
            e013: self.e013,
            e032: self.e032,
            e123: -other.e123,
        }
    }
}

impl<T> Sub<TriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e021 - other.e021,
            e013: self.e013 - other.e013,
            e032: self.e032 - other.e032,
            e123: -other.e123,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> XTriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e013 * other.e3 - self.e021 * other.e2,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e03: self.e032 * other.e2 - self.e013 * other.e1,
        }
    }
}

impl<T> BitOr<EVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e013 * other.e3 - self.e021 * other.e2,
            e02: self.e021 * other.e1 - self.e032 * other.e3,
            e03: self.e032 * other.e2 - self.e013 * other.e1,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
        }
    }
}

impl<T> BitOr<BiVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> XTriVector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e021 * other.e3 + self.e013 * other.e2 + self.e032 * other.e1),
        }
    }
}

impl<T> BitXor<EVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e021 * other.e3 + self.e013 * other.e2 + self.e032 * other.e1),
        }
    }
}

impl<T> Mul<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 * other.0,
            e013: self.e013 * other.0,
            e032: self.e032 * other.0,
        }
    }
}

impl<T> Mul<Vector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = (XVector3<T>, XTriVector3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (XVector3<T>, XTriVector3<T>) {
        let vector = self | other;

        let triv = XTriVector3 {
            e021: self.e013 * other.e23 - self.e032 * other.e31,
            e013: self.e032 * other.e12 - self.e021 * other.e23,
            e032: self.e021 * other.e31 - self.e013 * other.e12,
        };

        (vector, triv)
    }
}

impl<T> Mul<BiVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = (XVector3<T>, XTriVector3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (XVector3<T>, XTriVector3<T>) {
        let vector = self | other;

        let triv = XTriVector3 {
            e021: self.e013 * other.e23 - self.e032 * other.e31,
            e013: self.e032 * other.e12 - self.e021 * other.e23,
            e032: self.e021 * other.e31 - self.e013 * other.e12,
        };

        (vector, triv)
    }
}

impl<T> Mul<TriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: TriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e032 * other.e123,
            e02: self.e013 * other.e123,
            e03: self.e021 * other.e123,
        }
    }
}

impl<T> Mul<ETriVector3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e032 * other.e123,
            e02: self.e013 * other.e123,
            e03: self.e021 * other.e123,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e021 *= other.0;
        self.e013 *= other.0;
        self.e032 *= other.0;
    }
}

impl<T> Div<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e021 / other.0,
            e013: self.e013 / other.0,
            e032: self.e032 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for XTriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e021 /= other.0;
        self.e013 /= other.0;
        self.e032 /= other.0;
    }
}

/// Euclidean (weight-only) part of a 3D trivector (finite point weight).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ETriVector3<T> {
    pub e123: T,
}

impl<T> ETriVector3<T> {
    #[inline]
    pub const fn new(e123: T) -> Self {
        ETriVector3 { e123 }
    }
}

impl<T> ETriVector3<T>
where
    T: Num,
{
    pub const ZERO: Self = Self { e123: T::ZERO };

    #[inline]
    pub fn norm2(&self) -> T {
        self.e123 * self.e123
    }

    #[inline]
    pub fn norm(&self) -> T {
        self.e123.abs()
    }

    #[inline]
    pub fn signed_norm(&self) -> T {
        self.e123
    }

    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e123 /= norm;
        }
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut trivector = *self;
        trivector.normalize();
        trivector
    }
}

impl<T> Neg for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn neg(self) -> ETriVector3<T> {
        ETriVector3 { e123: -self.e123 }
    }
}

impl<T> Not for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn not(self) -> ETriVector3<T> {
        -self
    }
}

impl<T> Dual for ETriVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn dual(self) -> XVector3<T> {
        XVector3 { e0: self.e123 }
    }
}

impl<T> Mul<T> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn mul(self, rhs: T) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 * rhs,
        }
    }
}

impl<T> MulAssign<T> for ETriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e123 *= rhs;
    }
}

impl<T> Div<T> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn div(self, rhs: T) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 / rhs,
        }
    }
}

impl<T> DivAssign<T> for ETriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e123 /= rhs;
    }
}

impl<T> Add<XTriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: XTriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: other.e021,
            e013: other.e013,
            e032: other.e032,
            e123: self.e123,
        }
    }
}

impl<T> Add<ETriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn add(self, other: ETriVector3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 + other.e123,
        }
    }
}

impl<T> Add<TriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn add(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: other.e021,
            e013: other.e013,
            e032: other.e032,
            e123: self.e123 + other.e123,
        }
    }
}

impl<T> Sub<XTriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: XTriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -other.e021,
            e013: -other.e013,
            e032: -other.e032,
            e123: self.e123,
        }
    }
}

impl<T> Sub<ETriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn sub(self, other: ETriVector3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 - other.e123,
        }
    }
}

impl<T> Sub<TriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn sub(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -other.e021,
            e013: -other.e013,
            e032: -other.e032,
            e123: self.e123 - other.e123,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> ETriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e123 * other.e3,
            e31: self.e123 * other.e2,
            e23: self.e123 * other.e1,
        }
    }
}

impl<T> BitOr<EVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e123 * other.e3,
            e31: self.e123 * other.e2,
            e23: self.e123 * other.e1,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: -(self.e123 * other.e23),
            e2: -(self.e123 * other.e31),
            e3: -(self.e123 * other.e12),
        }
    }
}

impl<T> BitOr<BiVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: -(self.e123 * other.e23),
            e2: -(self.e123 * other.e31),
            e3: -(self.e123 * other.e12),
        }
    }
}

impl<T> BitOr<TriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e123 * other.e123))
    }
}

impl<T> BitOr<ETriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: ETriVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e123 * other.e123))
    }
}

impl<T> BitOr<Pseudo3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e123 * other.e0123,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> ETriVector3<T> {
        self * other
    }
}

impl<T> BitXor<XVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e123 * other.e0),
        }
    }
}

impl<T> BitXor<Vector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: -(self.e123 * other.e0),
        }
    }
}

impl<T> Mul<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 * other.0,
        }
    }
}

impl<T> Mul<XVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> Pseudo3<T> {
        self ^ other
    }
}

impl<T> Mul<EVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn mul(self, other: EVector3<T>) -> EBiVector3<T> {
        self | other
    }
}

impl<T> Mul<Vector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = (EBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (EBiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e123 * other.e03,
            e013: self.e123 * other.e02,
            e032: self.e123 * other.e01,
        }
    }
}

impl<T> Mul<EBiVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> EVector3<T> {
        self | other
    }
}

impl<T> Mul<BiVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = (EVector3<T>, XTriVector3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (EVector3<T>, XTriVector3<T>) {
        let vector = self | other;

        let triv = XTriVector3 {
            e021: self.e123 * other.e03,
            e013: self.e123 * other.e02,
            e032: self.e123 * other.e01,
        };

        (vector, triv)
    }
}

impl<T> Mul<XTriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: XTriVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e123 * other.e032),
            e02: -(self.e123 * other.e013),
            e03: -(self.e123 * other.e021),
        }
    }
}

impl<T> Mul<TriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, XBiVector3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (Scalar3<T>, XBiVector3<T>) {
        let scalar = self | other;

        let bivec = XBiVector3 {
            e01: -(self.e123 * other.e032),
            e02: -(self.e123 * other.e013),
            e03: -(self.e123 * other.e021),
        };

        (scalar, bivec)
    }
}

impl<T> Mul<ETriVector3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn mul(self, other: ETriVector3<T>) -> Scalar3<T> {
        self | other
    }
}

impl<T> Mul<Pseudo3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> XVector3<T> {
        self | other
    }
}

impl<T> MulAssign<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e123 *= other.0;
    }
}

impl<T> Div<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e123 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for ETriVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e123 /= other.0;
    }
}

impl<T> From<XTriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: XTriVector3<T>) -> Self {
        TriVector3 {
            e021: x.e021,
            e013: x.e013,
            e032: x.e032,
            e123: T::ZERO,
        }
    }
}

impl<T> From<ETriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: ETriVector3<T>) -> Self {
        TriVector3 {
            e021: T::ZERO,
            e013: T::ZERO,
            e032: T::ZERO,
            e123: x.e123,
        }
    }
}
