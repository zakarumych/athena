use core::ops::Mul;

use crate::scalar::Num;

use super::{
    elements::{EBiVector3, Pseudo3, Scalar3},
    Line3, Motor3, Plane3, Point3,
};

/// Rotor is a 3D rotation operator around the origin.
///
/// Unlike [`Motor3`], a rotor never carries a translation component.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotor3<T> {
    scalar: Scalar3<T>,
    bivector: EBiVector3<T>,
}

impl<T> Rotor3<T> {
    /// Creates a new rotor from the given scalar and bivector.
    #[inline]
    pub const fn new(scalar: Scalar3<T>, bivector: EBiVector3<T>) -> Self {
        Rotor3 { scalar, bivector }
    }
}

impl<T> Rotor3<T>
where
    T: Num,
{
    /// The identity rotor.
    pub const IDENTITY: Self = Rotor3 {
        scalar: Scalar3(T::ONE),
        bivector: EBiVector3::ZERO,
    };

    /// Returns the scalar part of this rotor.
    #[inline]
    pub const fn scalar(&self) -> Scalar3<T> {
        self.scalar
    }

    /// Returns the bivector part of this rotor.
    #[inline]
    pub const fn bivector(&self) -> EBiVector3<T> {
        self.bivector
    }

    #[inline]
    fn from_plane(plane: EBiVector3<T>, angle: T) -> Self {
        let half_angle = angle * T::HALF;
        let (sin, cos) = half_angle.sin_cos();

        Rotor3 {
            scalar: Scalar3(cos),
            bivector: plane.normalized() * -sin,
        }
    }

    /// Creates a new rotor from the given axis and angle.
    ///
    /// The resulting rotor rotates counterclockwise by the given angle around the
    /// axis through the origin and point, following the right-hand rule.
    /// Point may be an ideal point, in which case it directly names the axis direction.
    #[inline]
    pub fn from_axis_angle(axis: Point3<T>, angle: T) -> Self {
        let tv = axis.trivector();
        Self::from_plane(EBiVector3::new(tv.e021, tv.e013, tv.e032), angle)
    }

    /// Creates a new rotor that rotates within the given plane by the given angle.
    ///
    /// The rotation axis is the normal of `plane`, following the right-hand rule.
    /// Only the orientation of `plane` matters: its position is ignored, since a
    /// rotor always rotates around an axis through the origin.
    #[inline]
    pub fn in_plane(plane: Plane3<T>, angle: T) -> Self {
        let v = plane.vector();
        Self::from_plane(EBiVector3::new(v.e3, v.e2, v.e1), angle)
    }

    /// Moves the given point by this rotor.
    #[inline]
    pub fn move_point(&self, point: Point3<T>) -> Point3<T> {
        let tv0 = self.scalar * point.trivector();
        let (v, tv1) = self.bivector * point.trivector();
        let tv = tv0 + tv1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let _zero1 = v * m_s_r;
        let (_zero2, a) = v * m_bv_r;
        let b = tv * m_s_r;
        let (_zero3, c) = tv * m_bv_r;

        Point3::from_trivector((a + b + c).normalized())
    }

    /// Moves the given line by this rotor.
    #[inline]
    pub fn move_line(&self, line: Line3<T>) -> Line3<T> {
        let bv0 = self.scalar * line.bivector();
        let (s, bv1, p) = self.bivector * line.bivector();
        let bv = bv0 + bv1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let _zero1 = s * m_s_r;
        let a = s * m_bv_r;

        let b = bv * m_s_r;
        let (_zero2, c, _zero3) = bv * m_bv_r;

        let _zero4 = p * m_s_r;
        let d = p * m_bv_r;

        Line3::from_bivector((a + b + c + d).normalized())
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
            let norm = norm2.sqrt();
            self.scalar /= norm;
            self.bivector /= norm;
        }
    }

    /// Returns a normalized rotor.
    #[inline]
    pub fn normalized(&self) -> Self {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            Rotor3 {
                scalar: self.scalar / norm,
                bivector: self.bivector / norm,
            }
        } else {
            *self
        }
    }
}

impl<T> Mul<Rotor3<T>> for Rotor3<T>
where
    T: Num,
{
    type Output = Rotor3<T>;

    #[inline]
    fn mul(self, rhs: Rotor3<T>) -> Rotor3<T> {
        let s0 = self.scalar * rhs.scalar;
        let bv0 = self.scalar * rhs.bivector;
        let bv1 = self.bivector * rhs.scalar;
        let (s1, bv2) = self.bivector * rhs.bivector;

        Rotor3 {
            scalar: s0 + s1,
            bivector: bv0 + bv1 + bv2,
        }
        .normalized()
    }
}

impl<T> Mul<T> for Rotor3<T>
where
    T: Num,
{
    type Output = Rotor3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Rotor3<T> {
        if self.bivector.norm2() == T::ZERO {
            self
        } else {
            let atan = T::atan2(self.bivector.norm(), self.scalar.0);
            let log = self.bivector.normalized() * atan;

            let alpha_halved = log.norm() * rhs;
            let p = log.normalized();
            let (sin, cos) = alpha_halved.sin_cos();

            Rotor3 {
                scalar: Scalar3(cos),
                bivector: p * sin,
            }
            .normalized()
        }
    }
}

impl<T> From<Rotor3<T>> for Motor3<T>
where
    T: Num,
{
    #[inline]
    fn from(rotor: Rotor3<T>) -> Self {
        Motor3::new(rotor.scalar, rotor.bivector.into(), Pseudo3::ZERO)
    }
}

impl<T> From<Motor3<T>> for Rotor3<T>
where
    T: Num,
{
    /// Extracts the rotational part of the motor, discarding any translation.
    #[inline]
    fn from(motor: Motor3<T>) -> Self {
        let bv = motor.bivector();
        Rotor3 {
            scalar: motor.scalar(),
            bivector: EBiVector3::new(bv.e12, bv.e31, bv.e23),
        }
    }
}
