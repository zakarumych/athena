use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{BiVector2, Dual, EBiVector2, Pseudo2, Scalar2, XBiVector2};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector2<T> {
    pub e0: T,
    pub e1: T,
    pub e2: T,
}

impl<T> Vector2<T> {
    #[inline]
    pub const fn new(e0: T, e1: T, e2: T) -> Self {
        Vector2 { e0, e1, e2 }
    }

    #[inline]
    pub fn is_near_zero(&self) -> bool
    where
        T: Num,
    {
        let epsilon = T::EPSILON;
        self.e0.abs() < epsilon && self.e1.abs() < epsilon && self.e2.abs() < epsilon
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

    #[inline]
    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2
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
        }
    }

    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn bitor(self, other: Vector2<T>) -> Scalar2<T> {
        Scalar2(self.e1 * other.e1 + self.e2 * other.e2)
    }
}

impl<T> BitOr<XBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn bitor(self, other: XBiVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e2 * other.e20 - self.e1 * other.e01,
            e1: T::ZERO,
            e2: T::ZERO,
        }
    }
}

impl<T> BitOr<EBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn bitor(self, other: EBiVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: T::ZERO,
            e1: -(self.e2 * other.e12),
            e2: self.e1 * other.e12,
        }
    }
}

impl<T> BitOr<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitxor(self, other: Scalar2<T>) -> Vector2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn bitxor(self, other: Vector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e0 * other.e1 - self.e1 * other.e0,
            e20: self.e2 * other.e0 - self.e0 * other.e2,
            e12: self.e1 * other.e2 - self.e2 * other.e1,
        }
    }
}

impl<T> BitXor<XBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn bitxor(self, other: XBiVector2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e1 * other.e20 + self.e2 * other.e01,
        }
    }
}

impl<T> BitXor<EBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
    fn bitxor(self, other: EBiVector2<T>) -> Pseudo2<T> {
        Pseudo2 {
            e012: self.e0 * other.e12,
        }
    }
}

impl<T> BitXor<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
    fn mul(self, other: Vector2<T>) -> (Scalar2<T>, BiVector2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = (Vector2<T>, Pseudo2<T>);

    #[inline]
    fn mul(self, other: XBiVector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<EBiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = (Vector2<T>, Pseudo2<T>);

    #[inline]
    fn mul(self, other: EBiVector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = (Vector2<T>, Pseudo2<T>);

    #[inline]
    fn mul(self, other: BiVector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<Pseudo2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn mul(self, other: Pseudo2<T>) -> BiVector2<T> {
        self | other
    }
}

impl<T> Div<Scalar2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
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
    #[inline]
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e0 /= other.0;
        self.e1 /= other.0;
        self.e2 /= other.0;
    }
}

impl<T> Add<XVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: XVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 + other.e0,
            e1: self.e1,
            e2: self.e2,
        }
    }
}

impl<T> Add<EVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: EVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl<T> Sub<XVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: XVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 - other.e0,
            e1: self.e1,
            e2: self.e2,
        }
    }
}

impl<T> Sub<EVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: EVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl<T> BitOr<EVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn bitor(self, other: EVector2<T>) -> Scalar2<T> {
        Scalar2(self.e1 * other.e1 + self.e2 * other.e2)
    }
}

impl<T> BitXor<XVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn bitxor(self, other: XVector2<T>) -> XBiVector2<T> {
        XBiVector2 {
            e01: -(self.e1 * other.e0),
            e20: self.e2 * other.e0,
        }
    }
}

impl<T> BitXor<EVector2<T>> for Vector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
    fn bitxor(self, other: EVector2<T>) -> BiVector2<T> {
        BiVector2 {
            e01: self.e0 * other.e1,
            e20: -(self.e0 * other.e2),
            e12: self.e1 * other.e2 - self.e2 * other.e1,
        }
    }
}

/// Ideal (offset-only) part of a 2D vector (line at infinity).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct XVector2<T> {
    /// Offset component.
    pub e0: T,
}

impl<T> XVector2<T> {
    /// Creates a new ideal vector from its offset component.
    #[inline]
    pub const fn new(e0: T) -> Self {
        XVector2 { e0 }
    }
}

impl<T> XVector2<T>
where
    T: Num,
{
    /// The zero ideal vector.
    pub const ZERO: Self = Self { e0: T::ZERO };
}

impl<T> Neg for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn neg(self) -> XVector2<T> {
        XVector2 { e0: -self.e0 }
    }
}

impl<T> Not for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn not(self) -> XVector2<T> {
        self
    }
}

impl<T> Dual for XVector2<T>
where
    T: Num,
{
    type Output = EBiVector2<T>;

    #[inline]
    fn dual(self) -> EBiVector2<T> {
        EBiVector2 { e12: self.e0 }
    }
}

impl<T> Mul<T> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn mul(self, rhs: T) -> XVector2<T> {
        XVector2 { e0: self.e0 * rhs }
    }
}

impl<T> MulAssign<T> for XVector2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e0 *= rhs;
    }
}

impl<T> Div<T> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn div(self, rhs: T) -> XVector2<T> {
        XVector2 { e0: self.e0 / rhs }
    }
}

impl<T> DivAssign<T> for XVector2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e0 /= rhs;
    }
}

impl<T> Add<XVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn add(self, other: XVector2<T>) -> XVector2<T> {
        XVector2 {
            e0: self.e0 + other.e0,
        }
    }
}

impl<T> Add<EVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: EVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0,
            e1: other.e1,
            e2: other.e2,
        }
    }
}

impl<T> Add<Vector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 + other.e0,
            e1: other.e1,
            e2: other.e2,
        }
    }
}

impl<T> Sub<XVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn sub(self, other: XVector2<T>) -> XVector2<T> {
        XVector2 {
            e0: self.e0 - other.e0,
        }
    }
}

impl<T> Sub<EVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: EVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0,
            e1: -other.e1,
            e2: -other.e2,
        }
    }
}

impl<T> Sub<Vector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: self.e0 - other.e0,
            e1: -other.e1,
            e2: -other.e2,
        }
    }
}

impl<T> BitOr<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> XVector2<T> {
        self * other
    }
}

impl<T> BitXor<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn bitxor(self, other: Scalar2<T>) -> XVector2<T> {
        self * other
    }
}

impl<T> BitXor<EVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn bitxor(self, other: EVector2<T>) -> XBiVector2<T> {
        XBiVector2 {
            e01: self.e0 * other.e1,
            e20: -(self.e0 * other.e2),
        }
    }
}

impl<T> Mul<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn mul(self, other: Scalar2<T>) -> XVector2<T> {
        XVector2 {
            e0: self.e0 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e0 *= other.0;
    }
}

impl<T> Mul<EVector2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn mul(self, other: EVector2<T>) -> XBiVector2<T> {
        self ^ other
    }
}

impl<T> Div<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    type Output = XVector2<T>;

    #[inline]
    fn div(self, other: Scalar2<T>) -> XVector2<T> {
        XVector2 {
            e0: self.e0 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar2<T>> for XVector2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e0 /= other.0;
    }
}

/// Euclidean (direction-only) part of a 2D vector.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EVector2<T> {
    /// X direction component.
    pub e1: T,
    /// Y direction component.
    pub e2: T,
}

impl<T> EVector2<T> {
    /// Creates a new Euclidean vector from its direction components.
    #[inline]
    pub const fn new(e1: T, e2: T) -> Self {
        EVector2 { e1, e2 }
    }
}

impl<T> EVector2<T>
where
    T: Num,
{
    /// The zero Euclidean vector.
    pub const ZERO: Self = Self {
        e1: T::ZERO,
        e2: T::ZERO,
    };

    /// Returns squared norm of the vector.
    #[inline]
    pub fn norm2(&self) -> T {
        self.e1 * self.e1 + self.e2 * self.e2
    }

    /// Returns norm of the vector.
    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Normalizes the vector.
    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.e1 /= norm;
            self.e2 /= norm;
        }
    }

    /// Returns a normalized vector.
    #[inline]
    pub fn normalized(&self) -> Self {
        let mut vector = *self;
        vector.normalize();
        vector
    }
}

impl<T> Neg for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn neg(self) -> EVector2<T> {
        EVector2 {
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl<T> Not for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn not(self) -> EVector2<T> {
        self
    }
}

impl<T> Dual for EVector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn dual(self) -> XBiVector2<T> {
        XBiVector2 {
            e01: self.e2,
            e20: self.e1,
        }
    }
}

impl<T> Mul<T> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn mul(self, rhs: T) -> EVector2<T> {
        EVector2 {
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl<T> MulAssign<T> for EVector2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl<T> Div<T> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn div(self, rhs: T) -> EVector2<T> {
        EVector2 {
            e1: self.e1 / rhs,
            e2: self.e2 / rhs,
        }
    }
}

impl<T> DivAssign<T> for EVector2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e1 /= rhs;
        self.e2 /= rhs;
    }
}

impl<T> Add<XVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: XVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: other.e0,
            e1: self.e1,
            e2: self.e2,
        }
    }
}

impl<T> Add<EVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn add(self, other: EVector2<T>) -> EVector2<T> {
        EVector2 {
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl<T> Add<Vector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl<T> Sub<XVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: XVector2<T>) -> Vector2<T> {
        Vector2 {
            e0: -other.e0,
            e1: self.e1,
            e2: self.e2,
        }
    }
}

impl<T> Sub<EVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn sub(self, other: EVector2<T>) -> EVector2<T> {
        EVector2 {
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl<T> Sub<Vector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            e0: -other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl<T> BitOr<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> EVector2<T> {
        self * other
    }
}

impl<T> BitOr<EVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = Scalar2<T>;

    #[inline]
    fn bitor(self, other: EVector2<T>) -> Scalar2<T> {
        Scalar2(self.e1 * other.e1 + self.e2 * other.e2)
    }
}

impl<T> BitXor<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn bitxor(self, other: Scalar2<T>) -> EVector2<T> {
        self * other
    }
}

impl<T> BitXor<XVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn bitxor(self, other: XVector2<T>) -> XBiVector2<T> {
        XBiVector2 {
            e01: -(self.e1 * other.e0),
            e20: self.e2 * other.e0,
        }
    }
}

impl<T> BitXor<EVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EBiVector2<T>;

    #[inline]
    fn bitxor(self, other: EVector2<T>) -> EBiVector2<T> {
        EBiVector2 {
            e12: self.e1 * other.e2 - self.e2 * other.e1,
        }
    }
}

impl<T> Mul<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn mul(self, other: Scalar2<T>) -> EVector2<T> {
        EVector2 {
            e1: self.e1 * other.0,
            e2: self.e2 * other.0,
        }
    }
}

impl<T> MulAssign<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, other: Scalar2<T>) {
        self.e1 *= other.0;
        self.e2 *= other.0;
    }
}

impl<T> Mul<XVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = XBiVector2<T>;

    #[inline]
    fn mul(self, other: XVector2<T>) -> XBiVector2<T> {
        self ^ other
    }
}

impl<T> Mul<EVector2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = (Scalar2<T>, EBiVector2<T>);

    #[inline]
    fn mul(self, other: EVector2<T>) -> (Scalar2<T>, EBiVector2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Div<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    type Output = EVector2<T>;

    #[inline]
    fn div(self, other: Scalar2<T>) -> EVector2<T> {
        EVector2 {
            e1: self.e1 / other.0,
            e2: self.e2 / other.0,
        }
    }
}

impl<T> DivAssign<Scalar2<T>> for EVector2<T>
where
    T: Num,
{
    #[inline]
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e1 /= other.0;
        self.e2 /= other.0;
    }
}

impl<T> From<XVector2<T>> for Vector2<T>
where
    T: Num,
{
    #[inline]
    fn from(x: XVector2<T>) -> Self {
        Vector2 {
            e0: x.e0,
            e1: T::ZERO,
            e2: T::ZERO,
        }
    }
}

impl<T> From<EVector2<T>> for Vector2<T>
where
    T: Num,
{
    #[inline]
    fn from(x: EVector2<T>) -> Self {
        Vector2 {
            e0: T::ZERO,
            e1: x.e1,
            e2: x.e2,
        }
    }
}
