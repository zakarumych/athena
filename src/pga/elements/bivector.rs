use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::Num;

use super::{
    scalar::{Scalar2, Scalar3},
    Dual, Pseudo2, Pseudo3, TriVector3, Vector2, Vector3,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct BiVector2<T> {
    pub e01: T,
    pub e20: T,
    pub e12: T,
}

impl<T> BiVector2<T> {
    pub const fn new(e01: T, e20: T, e12: T) -> Self {
        BiVector2 { e01, e20, e12 }
    }
}

impl<T> BiVector2<T>
where
    T: Num,
{
    pub const ZERO: Self = Self {
        e01: T::ZERO,
        e20: T::ZERO,
        e12: T::ZERO,
    };

    pub fn norm2(&self) -> T {
        self.e12 * self.e12
    }

    pub fn norm(&self) -> T {
        self.e12.abs()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        if norm != T::ZERO {
            self.e01 /= norm;
            self.e20 /= norm;
            self.e12 /= norm;
        }
    }

    pub fn normalized(&self) -> Self {
        let mut bivector = *self;
        bivector.normalize();
        bivector
    }
}

impl<T> Neg for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn neg(self) -> BiVector2<T> {
        BiVector2 {
            e01: -self.e01,
            e20: -self.e20,
            e12: -self.e12,
        }
    }
}

impl<T> Dual for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn dual(self) -> Vector2<T> {
        Vector2 {
            e0: self.e12,
            e1: self.e20,
            e2: self.e01,
        }
    }
}

impl<T> Not for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn not(self) -> BiVector2<T> {
        -self
    }
}

impl<T> Mul<T> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn mul(self, other: T) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 * other,
            e20: self.e20 * other,
            e12: self.e12 * other,
        }
    }
}

impl<T> MulAssign<T> for BiVector2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, other: T) {
        self.e01 *= other;
        self.e20 *= other;
        self.e12 *= other;
    }
}

impl<T> Div<T> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn div(self, other: T) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 / other,
            e20: self.e20 / other,
            e12: self.e12 / other,
        }
    }
}

impl<T> DivAssign<T> for BiVector2<T>
where
    T: Num,
{
    fn div_assign(&mut self, other: T) {
        self.e01 /= other;
        self.e20 /= other;
        self.e12 /= other;
    }
}

impl<T> Add<BiVector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn add(self, other: BiVector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 + other.e01,
            e20: self.e20 + other.e20,
            e12: self.e12 + other.e12,
        }
    }
}

impl<T> Sub<BiVector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn sub(self, other: BiVector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 - other.e01,
            e20: self.e20 - other.e20,
            e12: self.e12 - other.e12,
        }
    }
}

impl<T> BitOr<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn bitor(self, other: Scalar2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitor(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e01 * other.e1 - self.e20 * other.e2,
            e1: self.e12 * other.e2,
            e2: -(self.e12 * other.e1),
        }
    }
}

impl<T> BitOr<BiVector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    fn bitor(self, other: BiVector2<T>) -> Scalar2<T> {
        Scalar2(-(self.e12 * other.e12))
    }
}

impl<T> BitOr<Pseudo2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn bitor(self, other: Pseudo2<T>) -> Vector2<T> {
        Vector2 {
            e0: -(self.e12 * other.e012),
            e1: T::ZERO,
            e2: T::ZERO,
        }
    }
}

impl<T> BitXor<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;
    fn bitxor(self, other: Scalar2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    fn bitxor(self, other: Vector2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e01 * other.e2 + self.e20 * other.e1 + self.e12 * other.e0,
        }
    }
}

impl<T> Mul<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn mul(self, other: Scalar2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 * other.0,
            e20: self.e20 * other.0,
            e12: self.e12 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e01 *= other.0;
        self.e20 *= other.0;
        self.e12 *= other.0;
    }
}

impl<T> Mul<Vector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = (Vector2<T>, Pseudo2<T>);

    fn mul(self, other: Vector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = (Scalar2<T>, BiVector2<T>);

    fn mul(self, other: BiVector2<T>) -> (Scalar2<T>, BiVector2<T>) {
        let scalar = self | other;
        let bivec = BiVector2 {
            e01: self.e20 * other.e12 - self.e12 * other.e20,
            e20: self.e12 * other.e01 - self.e01 * other.e12,
            e12: T::ZERO,
        };
        (scalar, bivec)
    }
}

impl<T> Mul<Pseudo2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    fn mul(self, other: Pseudo2<T>) -> Vector2<T> {
        self | other
    }
}

impl<T> Div<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    fn div(self, other: Scalar2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e01 / other.0,
            e20: self.e20 / other.0,
            e12: self.e12 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e01 /= other.0;
        self.e20 /= other.0;
        self.e12 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct XBiVector3<T> {
    pub e01: T,
    pub e02: T,
    pub e03: T,
}

impl<T> XBiVector3<T> {
    pub const fn new(e01: T, e02: T, e03: T) -> Self {
        XBiVector3 { e01, e02, e03 }
    }
}

impl<T> XBiVector3<T>
where
    T: Num,
{
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

    fn bitor(self, other: Scalar3<T>) -> XBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn bitor(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            e1: T::ZERO,
            e2: T::ZERO,
            e3: T::ZERO,
        }
    }
}

impl<T> BitXor<Vector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
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
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector3<T>> for XBiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

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
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e01 /= other.0;
        self.e02 /= other.0;
        self.e03 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct EBiVector3<T> {
    pub e12: T,
    pub e31: T,
    pub e23: T,
}

impl<T> EBiVector3<T> {
    pub const fn new(e12: T, e31: T, e23: T) -> Self {
        EBiVector3 { e12, e31, e23 }
    }
}

impl<T> EBiVector3<T>
where
    T: Num,
{
    pub const ZERO: Self = Self {
        e12: T::ZERO,
        e31: T::ZERO,
        e23: T::ZERO,
    };
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

    fn bitor(self, other: Scalar3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

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

    fn bitor(self, other: EBiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<BiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn bitor(self, other: BiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<TriVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

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

    fn bitor(self, other: Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3 {
            e01: -(self.e23 * other.e0123),
            e02: -(self.e31 * other.e0123),
            e03: -(self.e12 * other.e0123),
        }
    }
}

impl<T> BitXor<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;
    fn bitxor(self, other: Scalar3<T>) -> EBiVector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
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
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

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

    fn mul(self, other: Pseudo3<T>) -> XBiVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for EBiVector3<T>
where
    T: Num,
{
    type Output = EBiVector3<T>;

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
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e12 /= other.0;
        self.e31 /= other.0;
        self.e23 /= other.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct BiVector3<T> {
    pub e01: T,
    pub e02: T,
    pub e03: T,
    pub e12: T,
    pub e31: T,
    pub e23: T,
}

impl<T> BiVector3<T> {
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

    fn sub(self, other: BiVector3<T>) -> BiVector3<T> {
        BiVector3 {
            e01: self.e01 - other.e01,
            e02: self.e02 - other.e01,
            e03: self.e03 - other.e01,
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

    fn bitor(self, other: Scalar3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

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

    fn bitor(self, other: EBiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<BiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn bitor(self, other: BiVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23))
    }
}

impl<T> BitOr<TriVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

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

    fn bitxor(self, other: Scalar3<T>) -> BiVector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;
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
    fn mul(self, other: Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = (XBiVector3<T>, Pseudo3<T>);

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

    fn mul(self, other: Pseudo3<T>) -> XBiVector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for BiVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

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
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e01 /= other.0;
        self.e02 /= other.0;
        self.e03 /= other.0;
        self.e12 /= other.0;
        self.e31 /= other.0;
        self.e23 /= other.0;
    }
}
