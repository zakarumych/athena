use core::ops::{Add, BitOr, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub};

use crate::Num;

use super::{scalar::Scalar3, BiVector3, Dual, EBiVector3, Pseudo3, Vector3, XBiVector3};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct TriVector3<T> {
    pub e021: T,
    pub e013: T,
    pub e032: T,
    pub e123: T,
}

impl<T> TriVector3<T> {
    pub const fn new(e021: T, e013: T, e032: T, e123: T) -> Self {
        TriVector3 {
            e021,
            e013,
            e032,
            e123,
        }
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

    fn bitor(self, other: Scalar3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitOr<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = BiVector3<T>;

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

    fn bitor(self, other: BiVector3<T>) -> Vector3<T> {
        Vector3 {
            e0: self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            e1: -(self.e123 * other.e23),
            e2: -(self.e123 * other.e31),
            e3: -(self.e123 * other.e12),
        }
    }
}

impl<T> BitOr<TriVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Scalar3<T>;

    fn bitor(self, other: TriVector3<T>) -> Scalar3<T> {
        Scalar3(-(self.e123 * other.e123))
    }
}

impl<T> BitOr<Pseudo3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

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

    fn bitxor(self, other: Scalar3<T>) -> TriVector3<T> {
        self * other
    }
}

impl<T> BitXor<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Pseudo3<T>;

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
    fn mul_assign(&mut self, other: Scalar3<T>) {
        self.e021 *= other.0;
        self.e013 *= other.0;
        self.e032 *= other.0;
        self.e123 *= other.0;
    }
}

impl<T> Mul<Vector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = (BiVector3<T>, Pseudo3<T>);

    fn mul(self, other: Vector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        (self | other, self ^ other)
    }
}

impl<T> Mul<XBiVector3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

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

impl<T> Mul<Pseudo3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = Vector3<T>;

    fn mul(self, other: Pseudo3<T>) -> Vector3<T> {
        self | other
    }
}

impl<T> Div<Scalar3<T>> for TriVector3<T>
where
    T: Num,
{
    type Output = TriVector3<T>;

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
    fn div_assign(&mut self, other: Scalar3<T>) {
        self.e021 /= other.0;
        self.e013 /= other.0;
        self.e032 /= other.0;
        self.e123 /= other.0;
    }
}
