use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::scalar::Num;

use super::{Dual, Pseudo2, Scalar2, Vector2};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BiVector2<T> {
    pub e01: T,
    pub e20: T,
    pub e12: T,
}

impl<T> BiVector2<T> {
    #[inline]
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

    #[inline]
    pub fn norm2(&self) -> T {
        self.e12 * self.e12
    }

    #[inline]
    pub fn norm(&self) -> T {
        self.e12.abs()
    }

    #[inline]
    pub fn signed_norm(&self) -> T {
        self.e12
    }

    #[inline]
    pub fn normalize(&mut self) {
        let norm = self.signed_norm();
        if norm != T::ZERO {
            self.e01 /= norm;
            self.e20 /= norm;
            self.e12 = T::ONE;
        } else {
            let s = norm.sign();
            let m = (self.e01 * self.e01 + self.e20 * self.e20 + self.e12 * self.e12)
                .sqrt()
                .recip()
                * s;

            self.e01 *= m;
            self.e20 *= m;
            self.e12 = T::ZERO;
        }
    }

    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn bitor(self, other: Scalar2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitOr<Vector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
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

    #[inline]
    fn bitor(self, other: BiVector2<T>) -> Scalar2<T> {
        Scalar2(-(self.e12 * other.e12))
    }
}

impl<T> BitOr<Pseudo2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Vector2<T>;

    #[inline]
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

    #[inline]
    fn bitxor(self, other: Scalar2<T>) -> BiVector2<T> {
        self * other
    }
}

impl<T> BitXor<Vector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = Pseudo2<T>;

    #[inline]
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

    #[inline]
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
    #[inline]
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

    #[inline]
    fn mul(self, other: Vector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<BiVector2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = (Scalar2<T>, BiVector2<T>);

    #[inline]
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

    #[inline]
    fn mul(self, other: Pseudo2<T>) -> Vector2<T> {
        self | other
    }
}

impl<T> Div<Scalar2<T>> for BiVector2<T>
where
    T: Num,
{
    type Output = BiVector2<T>;

    #[inline]
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
    #[inline]
    fn div_assign(&mut self, other: Scalar2<T>) {
        self.e01 /= other.0;
        self.e20 /= other.0;
        self.e12 /= other.0;
    }
}
