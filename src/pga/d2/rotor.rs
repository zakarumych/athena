use core::ops::Mul;

use crate::scalar::Num;

use super::{
    elements::{EBiVector2, Scalar2},
    Line2, Motor2, Point2,
};

/// Rotor is a 2D rotation operator around the origin.
///
/// Unlike [`Motor2`], a rotor never carries a translation component.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotor2<T> {
    scalar: Scalar2<T>,
    bivector: EBiVector2<T>,
}

impl<T> Rotor2<T> {
    /// Creates a new rotor from the given scalar and bivector.
    #[inline]
    pub const fn new(scalar: Scalar2<T>, bivector: EBiVector2<T>) -> Self {
        Rotor2 { scalar, bivector }
    }
}

impl<T> Rotor2<T>
where
    T: Num,
{
    /// The identity rotor.
    pub const IDENTITY: Self = Rotor2 {
        scalar: Scalar2(T::ONE),
        bivector: EBiVector2::ZERO,
    };

    /// Returns the scalar part of this rotor.
    #[inline]
    pub const fn scalar(&self) -> Scalar2<T> {
        self.scalar
    }

    /// Returns the bivector part of this rotor.
    #[inline]
    pub const fn bivector(&self) -> EBiVector2<T> {
        self.bivector
    }

    /// Creates a new rotor from the given angle.
    ///
    /// The resulting rotor rotates counterclockwise by the given angle around the origin.
    #[inline]
    pub fn from_angle(angle: T) -> Self {
        let half_angle = angle * T::HALF;
        let (sin, cos) = half_angle.sin_cos();

        Rotor2 {
            scalar: Scalar2(cos),
            bivector: EBiVector2::new(-sin),
        }
    }

    /// Returns the angle of rotation of this rotor.
    #[inline]
    pub fn angle(&self) -> T {
        T::atan2(-self.bivector.e12, self.scalar.0) * T::TWO
    }

    /// Moves the given point by this rotor.
    #[inline]
    pub fn move_point(&self, point: Point2<T>) -> Point2<T> {
        let bv0 = self.scalar * point.bivector();
        let (s, bv1) = self.bivector * point.bivector();
        let bv = bv0 + bv1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let _zero1 = s * m_s_r;
        let a = s * m_bv_r;
        let b = bv * m_s_r;
        let (_zero2, c) = bv * m_bv_r;

        Point2::from_bivector((a + b + c).normalized())
    }

    /// Moves the given line by this rotor.
    #[inline]
    pub fn move_line(&self, line: Line2<T>) -> Line2<T> {
        let v0 = self.scalar * line.vector();
        let (v1, p) = self.bivector * line.vector();
        let v = v0 + v1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let a = v * m_s_r;
        let (b, _zero1) = v * m_bv_r;
        let _zero2 = p * m_s_r;
        let c = p * m_bv_r;

        Line2::from_vector((a + b + c).normalized())
    }

    /// Returns norm of the rotor.
    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Returns squared norm of the rotor.
    #[inline]
    pub fn norm2(&self) -> T {
        let s0 = self.scalar * !self.scalar;
        let s1 = self.bivector | !self.bivector;

        (s0 + s1).0
    }

    /// Normalizes the rotor.
    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = Scalar2(norm2.sqrt());
            self.scalar /= norm;
            self.bivector /= norm;
        }
    }

    /// Returns a normalized rotor.
    #[inline]
    pub fn normalized(&self) -> Self {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = Scalar2(norm2.sqrt());
            Rotor2 {
                scalar: self.scalar / norm,
                bivector: self.bivector / norm,
            }
        } else {
            *self
        }
    }
}

impl<T> Mul<Rotor2<T>> for Rotor2<T>
where
    T: Num,
{
    type Output = Rotor2<T>;

    #[inline]
    fn mul(self, rhs: Rotor2<T>) -> Rotor2<T> {
        let s0 = self.scalar * rhs.scalar;
        let bv0 = self.scalar * rhs.bivector;
        let bv1 = self.bivector * rhs.scalar;
        let s1 = self.bivector * rhs.bivector;

        Rotor2 {
            scalar: s0 + s1,
            bivector: bv0 + bv1,
        }
        .normalized()
    }
}

impl<T> Mul<T> for Rotor2<T>
where
    T: Num,
{
    type Output = Rotor2<T>;

    #[inline]
    fn mul(self, rhs: T) -> Rotor2<T> {
        let half_angle = T::atan2(self.bivector.e12, self.scalar.0) * rhs;
        let (sin, cos) = half_angle.sin_cos();

        Rotor2 {
            scalar: Scalar2(cos),
            bivector: EBiVector2::new(sin),
        }
        .normalized()
    }
}

impl<T> From<Rotor2<T>> for Motor2<T>
where
    T: Num,
{
    #[inline]
    fn from(rotor: Rotor2<T>) -> Self {
        Motor2::new(rotor.scalar, rotor.bivector.into())
    }
}
