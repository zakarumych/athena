use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{
    scalar::Scalar3, Dual, EVector3, ETriVector3, Pseudo3, TriVector3, Vector3, XTriVector3,
    XVector3,
};

/// Ideal (moment-only) part of a 3D bivector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct XBiVector3<T> {
    /// e0^e1 component.
    pub e01: T,
    /// e0^e2 component.
    pub e02: T,
    /// e0^e3 component.
    pub e03: T,
}

impl<T> XBiVector3<T> {
    /// Creates a new ideal bivector from its components.
    #[inline]
    pub const fn new(e01: T, e02: T, e03: T) -> Self {
        XBiVector3 { e01, e02, e03 }
    }
}

impl<T> XBiVector3<T>
where
    T: Num,
{
    /// The zero ideal bivector.
    pub const ZERO: Self = Self {
        e01: T::ZERO,
        e02: T::ZERO,
        e03: T::ZERO,
    };
}

impl<T> Neg for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn neg(self) -> XBiVector3<T> {
        XBiVector3 {
            e01: -self.e01,
            e02: -self.e02,
            e03: -self.e03,
        }
    }
}

impl<T> Not for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn not(self) -> XBiVector3<T> {
        -self
    }
}

impl<T> Dual for XBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn dual(self) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e03,
            e31: self.e02,
            e23: self.e01,
        }
    }
}

impl<T> Mul<T> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: T) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 * other,
            e02: self.e02 * other,
            e03: self.e03 * other,
        }
    }
}

impl<T> MulAssign<T> for XBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.e01 *= other;
        self.e02 *= other;
        self.e03 *= other;
    }
}

impl<T> Div<T> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn div(self, other: T) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 / other,
            e02: self.e02 / other,
            e03: self.e03 / other,
        }
    }
}

impl<T> DivAssign<T> for XBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.e01 /= other;
        self.e02 /= other;
        self.e03 /= other;
    }
}

impl<T> Add<XBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn add(self, other: XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 + other.e01,
            e02: self.e02 + other.e02,
            e03: self.e03 + other.e03,
        }
    }
}

impl<T> Add<EBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: EBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01,
            e02: self.e02,
            e03: self.e03,
            e12: other.e12,
            e31: other.e31,
            e23: other.e23,
        }
    }
}

impl<T> Add<BiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 + other.e01,
            e02: self.e02 + other.e02,
            e03: self.e03 + other.e03,
            e12: other.e12,
            e31: other.e31,
            e23: other.e23,
        }
    }
}

impl<T> Sub<XBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn sub(self, other: XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 - other.e01,
            e02: self.e02 - other.e02,
            e03: self.e03 - other.e03,
        }
    }
}

impl<T> Sub<EBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: EBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01,
            e02: self.e02,
            e03: self.e03,
            e12: -other.e12,
            e31: -other.e31,
            e23: -other.e23,
        }
    }
}

impl<T> Sub<BiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 - other.e01,
            e02: self.e02 - other.e02,
            e03: self.e03 - other.e03,
            e12: -other.e12,
            e31: -other.e31,
            e23: -other.e23,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<EVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> XVector3<T> {
        XVector3 {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
        }
    }
}

impl<T> BitOr<Vector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> BitXor<EVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: self.e02 * other.e1 - self.e01 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1,
            e032: self.e03 * other.e2 - self.e02 * other.e3,
        }
    }
}

impl<T> BitXor<Vector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e02 * other.e1 - self.e01 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1,
            e032: self.e03 * other.e2 - self.e02 * other.e3,
            e123: T::ZERO,
        }
    }
}

impl<T> BitXor<EBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: EBiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e03 * other.e12,
        }
    }
}

impl<T> BitXor<BiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e03 * other.e12,
        }
    }
}

impl<T> Mul<Scalar3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 * other.0,
            e02: self.e02 * other.0,
            e03: self.e03 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for XBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e01 *= other.0;
        self.e02 *= other.0;
        self.e03 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = (XVector3<T>, XTriVector3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (XVector3<T>, XTriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        let bivec = XBiVector3 {
            e01: self.e03 * other.e31 - self.e02 * other.e12,
            e02: self.e01 * other.e12 - self.e03 * other.e23,
            e03: self.e02 * other.e23 - self.e01 * other.e31,
        };
        let pseudo = self ^ other;
        (bivec, pseudo)
    }
}

impl<T> Mul<TriVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn mul(self, other: TriVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e03 * other.e123),
            e013: -(self.e02 * other.e123),
            e032: -(self.e01 * other.e123),
            e123: T::ZERO,
        }
    }
}

impl<T> Div<Scalar3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e01 / other.0,
            e02: self.e02 / other.0,
            e03: self.e03 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for XBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e01 /= other.0;
        self.e02 /= other.0;
        self.e03 /= other.0;
    }
}

/// Euclidean (direction-only) part of a 3D bivector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EBiVector3<T> {
    /// e1^e2 component.
    pub e12: T,
    /// e3^e1 component.
    pub e31: T,
    /// e2^e3 component.
    pub e23: T,
}

impl<T> EBiVector3<T> {
    /// Creates a new Euclidean bivector from its components.
    #[inline]
    pub const fn new(e12: T, e31: T, e23: T) -> Self {
        EBiVector3 { e12, e31, e23 }
    }
}

impl<T> EBiVector3<T>
where
    T: Num,
{
    /// The zero Euclidean bivector.
    pub const ZERO: Self = Self {
        e12: T::ZERO,
        e31: T::ZERO,
        e23: T::ZERO,
    };

    /// Returns the squared norm.
    #[inline]
    pub fn norm2(&self) -> T {
        self.e12 * self.e12 + self.e31 * self.e31 + self.e23 * self.e23
    }

    /// Returns the norm.
    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Normalizes this bivector in place.
    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e12 /= norm;
            self.e31 /= norm;
            self.e23 /= norm;
        }
    }

    /// Returns a normalized copy.
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut bivector = *self;
        bivector.normalize();
        bivector
    }
}

impl<T> Neg for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn neg(self) -> EBiVector3<T> {
        EBiVector3 {
            e12: -self.e12,
            e31: -self.e31,
            e23: -self.e23,
        }
    }
}

impl<T> Not for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn not(self) -> EBiVector3<T> {
        -self
    }
}

impl<T> Dual for EBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn dual(self) -> XBiVector3<T> {
        XBiVector3 {
            e01: self.e23,
            e02: self.e31,
            e03: self.e12,
        }
    }
}

impl<T> Mul<T> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn mul(self, other: T) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 * other,
            e31: self.e31 * other,
            e23: self.e23 * other,
        }
    }
}

impl<T> MulAssign<T> for EBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.e12 *= other;
        self.e31 *= other;
        self.e23 *= other;
    }
}

impl<T> Div<T> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn div(self, other: T) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 / other,
            e31: self.e31 / other,
            e23: self.e23 / other,
        }
    }
}

impl<T> DivAssign<T> for EBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.e12 /= other;
        self.e31 /= other;
        self.e23 /= other;
    }
}

impl<T> Add<XBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: XBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: other.e01,
            e02: other.e02,
            e03: other.e03,
            e12: self.e12,
            e31: self.e31,
            e23: self.e23,
        }
    }
}

impl<T> Add<EBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn add(self, other: EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 + other.e12,
            e31: self.e31 + other.e31,
            e23: self.e23 + other.e23,
        }
    }
}

impl<T> Add<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: other.e01,
            e02: other.e02,
            e03: other.e03,
            e12: self.e12 + other.e12,
            e31: self.e31 + other.e31,
            e23: self.e23 + other.e23,
        }
    }
}

impl<T> Sub<XBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: XBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: -other.e01,
            e02: -other.e02,
            e03: -other.e03,
            e12: self.e12,
            e31: self.e31,
            e23: self.e23,
        }
    }
}

impl<T> Sub<EBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn sub(self, other: EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 - other.e12,
            e31: self.e31 - other.e31,
            e23: self.e23 - other.e23,
        }
    }
}

impl<T> Sub<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: -other.e01,
            e02: -other.e02,
            e03: -other.e03,
            e12: self.e12 - other.e12,
            e31: self.e31 - other.e31,
            e23: self.e23 - other.e23,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<EVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EVector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> EVector3<T> {
        EVector3 {
            e1: self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e23 * other.e3 - self.e12 * other.e1,
            e3: self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl<T> BitOr<Vector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: T::ZERO,
            e1: self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e23 * other.e3 - self.e12 * other.e1,
            e3: self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<TriVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: -(self.e23 * other.e123),
            e2: -(self.e31 * other.e123),
            e3: -(self.e12 * other.e123),
        }
    }
}

impl<T> BitOr<Pseudo3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e23 * other.e0123),
            e02: -(self.e31 * other.e0123),
            e03: -(self.e12 * other.e0123),
        }
    }
}

impl<T> BitXor<XVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: -(self.e12 * other.e0),
            e013: -(self.e31 * other.e0),
            e032: -(self.e23 * other.e0),
        }
    }
}

impl<T> BitXor<EVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = ETriVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> ETriVector3<T> {
        ETriVector3 {
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl<T> BitXor<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: -(self.e12 * other.e0),
            e013: -(self.e31 * other.e0),
            e032: -(self.e23 * other.e0),
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl<T> BitXor<XBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XBiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl<T> BitXor<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl<T> Mul<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 * other.0,
            e31: self.e31 * other.0,
            e23: self.e23 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e12 *= other.0;
        self.e31 *= other.0;
        self.e23 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> XTriVector3<T> {
        self ^ other
    }
}

impl<T> Mul<EVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (EVector3<T>, ETriVector3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (EVector3<T>, ETriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        let bivec = XBiVector3 {
            e01: self.e12 * other.e02 - self.e31 * other.e03,
            e02: self.e23 * other.e03 - self.e12 * other.e01,
            e03: self.e31 * other.e01 - self.e23 * other.e02,
        };
        let pseudo = self ^ other;

        (bivec, pseudo)
    }
}

impl<T> Mul<EBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, EBiVector3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (Scalar3<T>, EBiVector3<T>) {
        let scalar = self | other;
        let bivec = EBiVector3 {
            e12: self.e31 * other.e23 - self.e23 * other.e31,
            e31: self.e23 * other.e12 - self.e12 * other.e23,
            e23: self.e12 * other.e31 - self.e31 * other.e12,
        };
        (scalar, bivec)
    }
}

impl<T> Mul<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (Scalar3<T>, BiVector3<T>, Pseudo3<T>) {
        let scalar = self | other;

        let bivec = BiVector3 {
            e01: self.e12 * other.e02 - self.e31 * other.e03,
            e02: self.e23 * other.e03 - self.e12 * other.e01,
            e03: self.e31 * other.e01 - self.e23 * other.e02,
            e12: self.e31 * other.e23 - self.e23 * other.e31,
            e31: self.e23 * other.e12 - self.e12 * other.e23,
            e23: self.e12 * other.e31 - self.e31 * other.e12,
        };
        let pseudo = self ^ other;

        (scalar, bivec, pseudo)
    }
}

impl<T> Mul<TriVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vector = self | other;

        let trivector = TriVector3 {
            e021: self.e31 * other.e032 - self.e23 * other.e013,
            e013: self.e23 * other.e021 - self.e12 * other.e032,
            e032: self.e12 * other.e013 - self.e31 * other.e021,
            e123: T::ZERO,
        };

        (vector, trivector)
    }
}

impl<T> Mul<Pseudo3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> XBiVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> EBiVector3<T> {
        EBiVector3 {
            e12: self.e12 / other.0,
            e31: self.e31 / other.0,
            e23: self.e23 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e12 /= other.0;
        self.e31 /= other.0;
        self.e23 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BiVector3<T> {
    pub e01: T,
    pub e02: T,
    pub e03: T,
    pub e12: T,
    pub e31: T,
    pub e23: T,
}

impl<T> From<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: XBiVector3<T>) -> Self {
        BiVector3 {
            e01: x.e01,
            e02: x.e02,
            e03: x.e03,
            e12: T::ZERO,
            e31: T::ZERO,
            e23: T::ZERO,
        }
    }
}

impl<T> From<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn from(x: EBiVector3<T>) -> Self {
        BiVector3 {
            e01: T::ZERO,
            e02: T::ZERO,
            e03: T::ZERO,
            e12: x.e12,
            e31: x.e31,
            e23: x.e23,
        }
    }
}

impl<T> BiVector3<T> {
    #[inline]
    pub const fn new(e01: T, e02: T, e03: T, e12: T, e31: T, e23: T) -> Self {
        BiVector3 {
            e01,
            e02,
            e03,
            e12,
            e31,
            e23,
        }
    }
}

impl<T> BiVector3<T>
where
    T: Num,
{
    pub const ZERO: Self = Self {
        e01: T::ZERO,
        e02: T::ZERO,
        e03: T::ZERO,
        e12: T::ZERO,
        e31: T::ZERO,
        e23: T::ZERO,
    };

    #[inline]
    pub fn norm2(&self) -> T {
        self.e12 * self.e12 + self.e31 * self.e31 + self.e23 * self.e23
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
            self.e01 /= norm;
            self.e02 /= norm;
            self.e03 /= norm;
            self.e12 /= norm;
            self.e31 /= norm;
            self.e23 /= norm;
        } else {
            let ideal2 = self.e01 * self.e01 + self.e02 * self.e02 + self.e03 * self.e03;
            if ideal2 != T::ZERO {
                let m = ideal2.sqrt().recip();
                self.e01 *= m;
                self.e02 *= m;
                self.e03 *= m;
            }
        }
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let mut bivector = *self;
        bivector.normalize();
        bivector
    }
}

impl<T> Neg for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn neg(self) -> BiVector3<T> {
        BiVector3 {
            e01: -self.e01,
            e02: -self.e02,
            e03: -self.e03,
            e12: -self.e12,
            e31: -self.e31,
            e23: -self.e23,
        }
    }
}

impl<T> Not for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn not(self) -> BiVector3<T> {
        -self
    }
}

impl<T> Dual for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn dual(self) -> BiVector3<T> {
        BiVector3 {
            e01: self.e23,
            e02: self.e31,
            e03: self.e12,
            e12: self.e03,
            e31: self.e02,
            e23: self.e01,
        }
    }
}

impl<T> Mul<T> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn mul(self, other: T) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 * other,
            e02: self.e02 * other,
            e03: self.e03 * other,
            e12: self.e12 * other,
            e31: self.e31 * other,
            e23: self.e23 * other,
        }
    }
}

impl<T> MulAssign<T> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.e01 *= other;
        self.e02 *= other;
        self.e03 *= other;
        self.e12 *= other;
        self.e31 *= other;
        self.e23 *= other;
    }
}

impl<T> Div<T> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn div(self, other: T) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 / other,
            e02: self.e02 / other,
            e03: self.e03 / other,
            e12: self.e12 / other,
            e31: self.e31 / other,
            e23: self.e23 / other,
        }
    }
}

impl<T> DivAssign<T> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.e01 /= other;
        self.e02 /= other;
        self.e03 /= other;
        self.e12 /= other;
        self.e31 /= other;
        self.e23 /= other;
    }
}

impl<T> Add<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: XBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 + other.e01,
            e02: self.e02 + other.e02,
            e03: self.e03 + other.e03,
            e12: self.e12,
            e31: self.e31,
            e23: self.e23,
        }
    }
}

impl<T> Add<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: EBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01,
            e02: self.e02,
            e03: self.e03,
            e12: self.e12 + other.e12,
            e31: self.e31 + other.e31,
            e23: self.e23 + other.e23,
        }
    }
}

impl<T> Add<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn add(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 + other.e01,
            e02: self.e02 + other.e02,
            e03: self.e03 + other.e03,
            e12: self.e12 + other.e12,
            e31: self.e31 + other.e31,
            e23: self.e23 + other.e23,
        }
    }
}

impl<T> Sub<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: XBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 - other.e01,
            e02: self.e02 - other.e02,
            e03: self.e03 - other.e03,
            e12: self.e12,
            e31: self.e31,
            e23: self.e23,
        }
    }
}

impl<T> Sub<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: EBiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01,
            e02: self.e02,
            e03: self.e03,
            e12: self.e12 - other.e12,
            e31: self.e31 - other.e31,
            e23: self.e23 - other.e23,
        }
    }
}

impl<T> Sub<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn sub(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 - other.e01,
            e02: self.e02 - other.e02,
            e03: self.e03 - other.e03,
            e12: self.e12 - other.e12,
            e31: self.e31 - other.e31,
            e23: self.e23 - other.e23,
        }
    }
}

impl<T> BitOr<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitor(self, other: Scalar3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitOr<EVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: EVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e23 * other.e3 - self.e12 * other.e1,
            e3: self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl<T> BitOr<Vector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: self.e12 * other.e2 - self.e31 * other.e3,
            e2: self.e23 * other.e3 - self.e12 * other.e1,
            e3: self.e31 * other.e1 - self.e23 * other.e2,
        }
    }
}

impl<T> BitOr<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: EBiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    #[inline]
    fn bitor(self, other: BiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<TriVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    #[inline]
    fn bitor(self, other: TriVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            e1: -(self.e23 * other.e123),
            e2: -(self.e31 * other.e123),
            e3: -(self.e12 * other.e123),
        }
    }
}

impl<T> BitOr<Pseudo3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn bitor(self, other: Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e23 * other.e0123),
            e02: -(self.e31 * other.e0123),
            e03: -(self.e12 * other.e0123),
        }
    }
}

impl<T> BitXor<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn bitxor(self, other: Scalar3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitXor<XVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn bitxor(self, other: XVector3<T>) -> XTriVector3<T> {
        XTriVector3 {
            e021: -(self.e12 * other.e0),
            e013: -(self.e31 * other.e0),
            e032: -(self.e23 * other.e0),
        }
    }
}

impl<T> BitXor<EVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: EVector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e02 * other.e1 - self.e01 * other.e2,
            e013: self.e01 * other.e3 - self.e03 * other.e1,
            e032: self.e03 * other.e2 - self.e02 * other.e3,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl<T> BitXor<Vector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

    #[inline]
    fn bitxor(self, other: Vector3<T>) -> TriVector3<T> {
        TriVector3 {
            e021: self.e02 * other.e1 - self.e01 * other.e2 - self.e12 * other.e0,
            e013: self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            e032: self.e03 * other.e2 - self.e02 * other.e3 - self.e23 * other.e0,
            e123: self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        }
    }
}

impl<T> BitXor<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: XBiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01,
        }
    }
}

impl<T> BitXor<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: EBiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e01 * other.e23 + self.e02 * other.e31 + self.e03 * other.e12,
        }
    }
}

impl<T> BitXor<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

    #[inline]
    fn bitxor(self, other: BiVector3<T>) -> Pseudo3<T> {
        Pseudo3 {
            e0123: self.e12 * other.e03
                + self.e31 * other.e02
                + self.e23 * other.e01
                + self.e01 * other.e23
                + self.e02 * other.e31
                + self.e03 * other.e12,
        }
    }
}

impl<T> Mul<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn mul(self, other: Scalar3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 * other.0,
            e02: self.e02 * other.0,
            e03: self.e03 * other.0,
            e12: self.e12 * other.0,
            e31: self.e31 * other.0,
            e23: self.e23 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e01 *= other.0;
        self.e02 *= other.0;
        self.e03 *= other.0;
        self.e12 *= other.0;
        self.e31 *= other.0;
        self.e23 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = XTriVector3<T>;

    #[inline]
    fn mul(self, other: XVector3<T>) -> XTriVector3<T> {
        self ^ other
    }
}

impl<T> Mul<EVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: EVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: XBiVector3<T>) -> (XBiVector3<T>, Pseudo3<T>) {
        let bivec = XBiVector3 {
            e01: self.e12 * other.e02 - self.e31 * other.e03,
            e02: self.e23 * other.e03 - self.e12 * other.e01,
            e03: self.e31 * other.e01 - self.e23 * other.e02,
        };

        let pseudo = self ^ other;

        (bivec, pseudo)
    }
}

impl<T> Mul<EBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: EBiVector3<T>) -> (Scalar3<T>, BiVector3<T>, Pseudo3<T>) {
        let scalar = self | other;

        let bivec = BiVector3 {
            e01: self.e03 * other.e31 - self.e02 * other.e12,
            e02: self.e01 * other.e12 - self.e03 * other.e23,
            e03: self.e02 * other.e23 - self.e01 * other.e31,
            e12: self.e31 * other.e23 - self.e23 * other.e31,
            e31: self.e23 * other.e12 - self.e12 * other.e23,
            e23: self.e12 * other.e31 - self.e31 * other.e12,
        };

        let pseudo = self ^ other;

        (scalar, bivec, pseudo)
    }
}

impl<T> Mul<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (Scalar3<T>, BiVector3<T>, Pseudo3<T>);

    #[inline]
    fn mul(self, other: BiVector3<T>) -> (Scalar3<T>, BiVector3<T>, Pseudo3<T>) {
        let scalar = self | other;

        let bivec = BiVector3 {
            e01: self.e03 * other.e31 - self.e02 * other.e12 + self.e12 * other.e02
                - self.e31 * other.e03,
            e02: self.e01 * other.e12 - self.e03 * other.e23 + self.e23 * other.e03
                - self.e12 * other.e01,
            e03: self.e02 * other.e23 - self.e01 * other.e31 + self.e31 * other.e01
                - self.e23 * other.e02,
            e12: self.e31 * other.e23 - self.e23 * other.e31,
            e31: self.e23 * other.e12 - self.e12 * other.e23,
            e23: self.e12 * other.e31 - self.e31 * other.e12,
        };

        let pseudo = self ^ other;

        (scalar, bivec, pseudo)
    }
}

impl<T> Mul<TriVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (Vector3<T>, TriVector3<T>);

    #[inline]
    fn mul(self, other: TriVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vector = self | other;

        let trivector = TriVector3 {
            e021: self.e31 * other.e032 - self.e23 * other.e013 - self.e03 * other.e123,
            e013: self.e23 * other.e021 - self.e12 * other.e032 - self.e02 * other.e123,
            e032: self.e12 * other.e013 - self.e31 * other.e021 - self.e01 * other.e123,
            e123: T::ZERO,
        };

        (vector, trivector)
    }
}

impl<T> Mul<Pseudo3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = XBiVector3<T>;

    #[inline]
    fn mul(self, other: Pseudo3<T>) -> XBiVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

    #[inline]
    fn div(self, other: Scalar3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 / other.0,
            e02: self.e02 / other.0,
            e03: self.e03 / other.0,
            e12: self.e12 / other.0,
            e31: self.e31 / other.0,
            e23: self.e23 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e01 /= other.0;
        self.e02 /= other.0;
        self.e03 /= other.0;
        self.e12 /= other.0;
        self.e31 /= other.0;
        self.e23 /= other.0;
    }
}
