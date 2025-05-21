use core::ops::Mul;

use crate::Num;

use super::{
    elements::{BiVector2, BiVector3, Pseudo3, Scalar2, Scalar3},
    Line2, Line3, Plane3, Point2, Point3,
};

/// Motor is a 2D rotation and translation operator.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Motor2<T> {
    scalar: Scalar2<T>,
    bivector: BiVector2<T>,
}

impl<T> Motor2<T> {
    /// Creates a new motor from the given scalar and bivector.
    pub const fn new(scalar: Scalar2<T>, bivector: BiVector2<T>) -> Self {
        Motor2 { scalar, bivector }
    }
}

impl<T> Motor2<T>
where
    T: Num,
{
    /// Returns the scalar part of this motor.
    pub const fn scalar(&self) -> Scalar2<T> {
        self.scalar
    }

    /// Returns the bivector part of this motor.
    pub const fn bivector(&self) -> BiVector2<T> {
        self.bivector
    }

    /// Creates a new motor from the given points.
    ///
    /// The resulting motor moves by the double the distance between the two points.
    pub fn point_point(a: Point2<T>, b: Point2<T>) -> Self {
        let (s, bv) = b.bivector() * !a.bivector();

        let double = Motor2 {
            scalar: s,
            bivector: bv,
        };

        double.normalized().sqrt()
    }

    /// Reconstructs a motor that brings points `a` to points `b`.
    ///
    /// The resulting motor will move a[0] to b[0] and a[1] to the line through b[0] and b[1].
    pub fn reconstruct(a: [Point2<T>; 2], b: [Point2<T>; 2]) -> Self {
        // Construct translation motor to move a[0] to b[0].
        let v1 = Self::point_point(a[0], b[0]);

        // Translate a[1].
        let a1 = v1.move_point(a[1]).normalized();

        // Construct rotation motor that rotates translated line through a[0] and a[1] to line through b[0] and b[1]
        // while preserving b[0].
        let al = b[0].join(b[1]);
        let al1 = b[0].join(a1).normalized();

        let v2 = Self::line_line(al1, al).normalized();

        v2 * v1
    }

    /// Creates a new motor from the given lines.
    ///
    /// The resulting motor translates by the distance between the two lines if they are parallel.
    /// If they are not parallel, the motor rotates around the intersection point of the two lines by the angle between them.
    pub fn line_line(a: Line2<T>, b: Line2<T>) -> Self {
        let (s, bv) = !b.vector() * !a.vector();

        let double = Motor2 {
            scalar: s,
            bivector: bv,
        };

        double.normalized().sqrt()
    }

    /// Moves the given point by this motor.
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

    /// Moves the given line by this motor.
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

    /// Returns norm of the motor.
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Returns squared norm of the motor.
    pub fn norm2(&self) -> T {
        let s0 = self.scalar * !self.scalar;
        let s1 = self.bivector | !self.bivector;

        (s0 + s1).0
    }

    /// Normalizes the motor.
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = Scalar2(norm2.sqrt());
            self.scalar /= norm;
            self.bivector /= norm;
        }
    }

    /// Returns a normalized motor.
    pub fn normalized(&self) -> Self {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = Scalar2(norm2.sqrt());
            Motor2 {
                scalar: self.scalar / norm,
                bivector: self.bivector / norm,
            }
        } else {
            *self
        }
    }

    /// Halves the motor.
    pub fn sqrt(&self) -> Self {
        Motor2 {
            scalar: (self.scalar + T::ONE),
            bivector: self.bivector,
        }
        .normalized()
    }
}

impl<T> Mul<Motor2<T>> for Motor2<T>
where
    T: Num,
{
    type Output = Motor2<T>;

    fn mul(self, rhs: Motor2<T>) -> Motor2<T> {
        let s0 = self.scalar * rhs.scalar;
        let bv0 = self.scalar * rhs.bivector;
        let bv1 = self.bivector * rhs.scalar;
        let (s1, bv2) = self.bivector * rhs.bivector;

        Motor2 {
            scalar: s0 + s1,
            bivector: bv0 + bv1 + bv2,
        }
        .normalized()
    }
}

impl<T> Mul<T> for Motor2<T>
where
    T: Num,
{
    type Output = Motor2<T>;

    fn mul(self, rhs: T) -> Motor2<T> {
        if self.bivector.e12 == T::ZERO {
            let log = self.bivector;

            let scalar = Scalar2(T::ONE);
            let distance_halved = log.norm() * rhs;

            let bivector = log.normalized() * distance_halved;

            Motor2 { scalar, bivector }.normalized()
        } else {
            let atan = T::atan2(self.bivector.norm(), self.scalar.0);
            let log = self.bivector.normalized() * atan;

            let alpha_halved = log.norm() * rhs;
            let p = log.normalized();
            let (sin, cos) = alpha_halved.sin_cos();

            let scalar = Scalar2(cos);
            let bivector = p * sin;

            Motor2 { scalar, bivector }.normalized()
        }
    }
}

/// Motor is a 2D rotation and translation operator.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Motor3<T> {
    scalar: Scalar3<T>,
    bivector: BiVector3<T>,
    pseudo: Pseudo3<T>,
}

impl<T> Motor3<T> {
    /// Creates a new motor from the given scalar and bivector.
    pub const fn new(scalar: Scalar3<T>, bivector: BiVector3<T>, pseudo: Pseudo3<T>) -> Self {
        Motor3 {
            scalar,
            bivector,
            pseudo,
        }
    }
}

impl<T> Motor3<T>
where
    T: Num,
{
    /// Returns the scalar part of this motor.
    pub const fn scalar(&self) -> Scalar3<T> {
        self.scalar
    }

    /// Returns the bivector part of this motor.
    pub const fn bivector(&self) -> BiVector3<T> {
        self.bivector
    }

    /// Creates a new motor from the given points.
    ///
    /// The resulting motor moves by the double the distance between the two points.
    pub fn point_point(a: Point3<T>, b: Point3<T>) -> Self {
        let (s, bv) = b.trivector() * !a.trivector();

        let double = Motor3 {
            scalar: s,
            bivector: bv.into(),
            pseudo: Pseudo3::ZERO,
        };

        double.normalized().sqrt()
    }

    /// Reconstructs a motor that brings points `a` to points `b`.
    ///
    /// The resulting motor will move a[0] to b[0] and a[1] to the line through b[0] and b[1].
    pub fn reconstruct(a: [Point3<T>; 3], b: [Point3<T>; 3]) -> Self {
        // Construct translation motor to move a[0] to b[0].
        let v1 = Self::point_point(a[0], b[0]);

        // Translate a[1].
        let a1 = v1.move_point(a[1]).normalized();

        // Construct rotation motor that rotates translated line through a[0] and a[1] to line through b[0] and b[1]
        // while preserving b[0].
        let al = b[0].join(b[1]);
        let al1 = b[0].join(a1).normalized();

        let v2 = Self::line_line(al1, al).normalized();

        let v21 = v2 * v1;

        let a1 = v21.move_point(a[1]).normalized();
        let a2 = v21.move_point(a[2]).normalized();

        let al = b[0].join3(b[1], b[2]);
        let al1 = b[0].join3(a1, a2).normalized();

        let v3 = Self::plane_plane(al1, al).normalized();

        v3 * v21
    }

    /// Creates a new motor from the given lines.
    ///
    /// The resulting motor translates by the distance between the two lines if they are parallel.
    /// If they are not parallel, the motor rotates around the intersection point of the two lines by the angle between them.
    pub fn line_line(a: Line3<T>, b: Line3<T>) -> Self {
        let (s, bv, p) = !b.bivector() * !a.bivector();

        let double = Motor3 {
            scalar: s,
            bivector: bv,
            pseudo: p,
        };

        double.normalized().sqrt()
    }

    /// Creates a new motor from the given lines.
    ///
    /// The resulting motor translates by the distance between the two lines if they are parallel.
    /// If they are not parallel, the motor rotates around the intersection point of the two lines by the angle between them.
    pub fn plane_plane(a: Plane3<T>, b: Plane3<T>) -> Self {
        let (s, bv) = !b.vector() * !a.vector();

        let double = Motor3 {
            scalar: s,
            bivector: bv,
            pseudo: Pseudo3::ZERO,
        };

        double.normalized().sqrt()
    }

    /// Moves the given point by this motor.
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

    /// Moves the given line by this motor.
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

    /// Returns norm of the motor.
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Returns squared norm of the motor.
    pub fn norm2(&self) -> T {
        let s0 = self.scalar * !self.scalar;
        let s1 = self.bivector | !self.bivector;

        (s0 + s1).0
    }

    /// Normalizes the motor.
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.scalar /= norm;
            self.bivector /= norm;
        }
    }

    /// Returns a normalized motor.
    pub fn normalized(&self) -> Self {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            Motor3 {
                scalar: self.scalar / norm,
                bivector: self.bivector / norm,
                pseudo: self.pseudo / norm,
            }
        } else {
            *self
        }
    }

    /// Halves the motor.
    pub fn sqrt(&self) -> Self {
        let a = Motor3 {
            scalar: (self.scalar + T::ONE),
            bivector: self.bivector,
            pseudo: self.pseudo,
        };

        let mut b = a.normalized();

        b.pseudo = self.pseudo * T::HALF;
        a * b.normalized()
    }
}

impl<T> Mul<Motor3<T>> for Motor3<T>
where
    T: Num,
{
    type Output = Motor3<T>;

    fn mul(self, rhs: Motor3<T>) -> Motor3<T> {
        let s0 = self.scalar * rhs.scalar;
        let bv0 = self.scalar * rhs.bivector;
        let p0 = self.scalar * rhs.pseudo;

        let bv1 = self.bivector * rhs.scalar;
        let (s1, bv2, p1) = self.bivector * rhs.bivector;

        let p2 = self.pseudo * rhs.scalar;
        let bv3 = self.pseudo * rhs.bivector;

        Motor3 {
            scalar: s0 + s1,
            bivector: bv0 + bv1 + bv2 + bv3,
            pseudo: p0 + p1 + p2,
        }
        .normalized()
    }
}

impl<T> Mul<T> for Motor3<T>
where
    T: Num,
{
    type Output = Motor3<T>;

    fn mul(self, rhs: T) -> Motor3<T> {
        todo!()
    }
}
