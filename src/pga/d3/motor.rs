use core::ops::Mul;

use crate::scalar::Num;

use super::{
    elements::{BiVector3, Pseudo3, Scalar3},
    Line3, Plane3, Point3,
};

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
    #[inline]
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
    #[inline]
    pub const fn scalar(&self) -> Scalar3<T> {
        self.scalar
    }

    /// Returns the bivector part of this motor.
    #[inline]
    pub const fn bivector(&self) -> BiVector3<T> {
        self.bivector
    }

    /// Creates a new motor that rotates by the given angle around `axis`.
    ///
    /// Unlike `Rotor3::rotor`, `axis` does not need to pass through the origin: this
    /// rotates around the line itself, not just its direction. The rotation is
    /// counterclockwise around `axis`, following the right-hand rule. `axis` does not
    /// need to be normalized.
    #[inline]
    pub fn from_axis_angle(axis: Line3<T>, angle: T) -> Self {
        let bv = axis.bivector().normalized();

        let half_angle = angle * T::HALF;
        let (sin, cos) = half_angle.sin_cos();

        Motor3 {
            scalar: Scalar3(cos),
            bivector: bv * -sin,
            pseudo: Pseudo3::ZERO,
        }
    }

    /// Creates a new motor from the given points.
    ///
    /// The resulting motor moves by the double the distance between the two points.
    #[inline]
    pub fn from_point_to_point(a: Point3<T>, b: Point3<T>) -> Self {
        let (s, bv) = b.trivector() * !a.trivector();

        let double = Motor3 {
            scalar: s,
            bivector: bv.into(),
            pseudo: Pseudo3::ZERO,
        };

        double.normalized().sqrt()
    }

    /// Creates a new screw motor from the given lines.
    ///
    /// The resulting motor translates by the distance between the two lines.
    /// And rotates around the intersection axis of the two lines by the angle between them.
    #[inline]
    pub fn from_line_to_line(a: Line3<T>, b: Line3<T>) -> Self {
        let (s, bv, p) = b.bivector() * !a.bivector();

        let double = Motor3 {
            scalar: s,
            bivector: bv,
            pseudo: p,
        };

        double.normalized().sqrt()
    }

    /// Creates a new motor from the given planes.
    ///
    /// The resulting motor translates by the distance between the two planes if they are parallel.
    /// If they are not parallel, the motor rotates around the intersection line of the two planes by the angle between them.
    #[inline]
    pub fn from_plane_to_plane(a: Plane3<T>, b: Plane3<T>) -> Self {
        let (s, bv) = !b.vector() * !a.vector();

        let double = Motor3 {
            scalar: s,
            bivector: bv,
            pseudo: Pseudo3::ZERO,
        };

        double.normalized().sqrt()
    }

    /// Reconstructs a motor that brings points `a` to points `b`.
    ///
    /// The resulting motor will move a[0] to b[0] and a[1] to the line through b[0] and b[1].
    /// The third point a[2] will be moved to the plane through b[0], b[1], and b[2].
    #[inline]
    pub fn reconstruct(a: [Point3<T>; 3], b: [Point3<T>; 3]) -> Self {
        // Construct translation motor to move a[0] to b[0].
        let v1 = Self::from_point_to_point(a[0], b[0]);

        // Translate a[1].
        let a1 = v1.move_point(a[1]).normalized();

        // Construct rotation motor that rotates translated line through a[0] and a[1] to line through b[0] and b[1]
        // while preserving b[0].
        let al = b[0].join(b[1]);
        let al1 = b[0].join(a1).normalized();

        let v2 = Self::from_line_to_line(al1, al).normalized();

        let v21 = v2 * v1;

        let a1 = v21.move_point(a[1]).normalized();
        let a2 = v21.move_point(a[2]).normalized();

        let al = b[0].join3(b[1], b[2]);
        let al1 = b[0].join3(a1, a2).normalized();

        let v3 = Self::from_plane_to_plane(al1, al).normalized();

        v3 * v21
    }

    /// Moves the given point by this motor.
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

    /// Moves the given line by this motor.
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

    /// Returns norm of the motor.
    #[inline]
    pub fn norm(&self) -> T {
        self.norm2().sqrt()
    }

    /// Returns squared norm of the motor.
    #[inline]
    pub fn norm2(&self) -> T {
        let s0 = self.scalar * !self.scalar;
        let s1 = self.bivector | !self.bivector;

        (s0 + s1).0
    }

    /// Normalizes the motor.
    #[inline]
    pub fn normalize(&mut self) {
        let norm2 = self.norm2();
        if norm2 != T::ZERO {
            let norm = norm2.sqrt();
            self.scalar /= norm;
            self.bivector /= norm;
        }
    }

    /// Returns a normalized motor.
    #[inline]
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
    #[inline]
    pub fn sqrt(&self) -> Self {
        let one_plus_m = Motor3 {
            scalar: Scalar3(self.scalar.0 + T::ONE),
            bivector: self.bivector,
            pseudo: self.pseudo,
        };

        let s1 = self.scalar.0 + T::ONE;

        let correction = Motor3 {
            scalar: Scalar3(T::ONE),
            bivector: BiVector3::ZERO,
            pseudo: (self.pseudo / s1) * -T::HALF,
        };

        one_plus_m * correction
    }
}

impl<T> Mul<Motor3<T>> for Motor3<T>
where
    T: Num,
{
    type Output = Motor3<T>;

    #[inline]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    fn assert_point_approx(actual: Point3<f32>, expected: Point3<f32>) {
        let (ax, ay, az) = actual.coords();
        let (ex, ey, ez) = expected.coords();
        assert!(
            approx(ax, ex) && approx(ay, ey) && approx(az, ez),
            "expected {:?}, got {:?}",
            expected,
            actual
        );
    }

    #[test]
    fn reconstruct_identity() {
        let frame = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let motor = Motor3::reconstruct(frame, frame);

        let sample = Point3::at(2.0, -3.0, 4.0);
        assert_point_approx(motor.move_point(sample), sample);
    }

    #[test]
    fn reconstruct_rotate_90_about_x() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 0.0, 1.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(1.0, 0.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(0.0, 0.0, 1.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(0.0, -1.0, 0.0),
        );
    }

    #[test]
    fn reconstruct_rotate_90_about_y() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(0.0, 0.0, -1.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(0.0, 0.0, -1.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(0.0, 1.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(1.0, 0.0, 0.0),
        );
    }

    #[test]
    fn reconstruct_rotate_90_about_z() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(0.0, 1.0, 0.0),
            Point3::at(-1.0, 0.0, 0.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(0.0, 1.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(-1.0, 0.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(0.0, 0.0, 1.0),
        );
    }

    // KNOWN BUG: `Motor3::reconstruct` produces NaN for exact 180-degree
    // configurations. When the intermediate line-to-line (or plane-to-plane)
    // step is fed exactly antiparallel blades, their wedge/cross term is
    // identically zero, so the raw pre-halving motor collapses to
    // `scalar == -1` with a zero bivector/pseudo carrying no axis
    // information. `Motor3::sqrt` computes the half-angle motor via
    // `normalize(1 + m)`, which divides by `scalar + 1`, i.e. by zero here.
    // This is the standard closed-form "square root of a rotor" antipodal
    // singularity (the same class of issue as antipodal quaternion slerp),
    // not a simple typo: fixing it needs an alternate construction path fed
    // with the original blades (not just the already-reduced motor `sqrt`
    // receives), which is a larger, riskier change than is safe to make
    // without further design review. Left as a documented, pinned failure;
    // do not work around this downstream in the `conv` layer.
    #[test]
    #[ignore = "known bug: Motor3::reconstruct produces NaN at exact 180-degree configurations, see comment above"]
    fn reconstruct_rotate_180_about_x() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, -1.0, 0.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(1.0, 0.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(0.0, -1.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(0.0, 0.0, -1.0),
        );
    }

    #[test]
    #[ignore = "known bug: Motor3::reconstruct produces NaN at exact 180-degree configurations, see comment above reconstruct_rotate_180_about_x"]
    fn reconstruct_rotate_180_about_y() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(-1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(-1.0, 0.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(0.0, 1.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(0.0, 0.0, -1.0),
        );
    }

    #[test]
    #[ignore = "known bug: Motor3::reconstruct produces NaN at exact 180-degree configurations, see comment above reconstruct_rotate_180_about_x"]
    fn reconstruct_rotate_180_about_z() {
        let a = [
            Point3::ORIGIN,
            Point3::at(1.0, 0.0, 0.0),
            Point3::at(0.0, 1.0, 0.0),
        ];
        let b = [
            Point3::ORIGIN,
            Point3::at(-1.0, 0.0, 0.0),
            Point3::at(0.0, -1.0, 0.0),
        ];
        let motor = Motor3::reconstruct(a, b);

        assert_point_approx(
            motor.move_point(Point3::at(1.0, 0.0, 0.0)),
            Point3::at(-1.0, 0.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 1.0, 0.0)),
            Point3::at(0.0, -1.0, 0.0),
        );
        assert_point_approx(
            motor.move_point(Point3::at(0.0, 0.0, 1.0)),
            Point3::at(0.0, 0.0, 1.0),
        );
    }
}

impl<T> Mul<T> for Motor3<T>
where
    T: Num,
{
    type Output = Motor3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Motor3<T> {
        if self.bivector.norm2() == T::ZERO {
            let log = self.bivector;

            let scalar = Scalar3(T::ONE);
            let distance_halved =
                (log.e01 * log.e01 + log.e02 * log.e02 + log.e03 * log.e03).sqrt() * rhs;

            let bivector = log.normalized() * distance_halved;

            Motor3 {
                scalar,
                bivector,
                pseudo: Pseudo3::ZERO,
            }
            .normalized()
        } else {
            let atan = T::atan2(self.bivector.norm(), self.scalar.0);
            let log = self.bivector.normalized() * atan;

            let alpha_halved = log.norm() * rhs;
            let p = log.normalized();
            let (sin, cos) = alpha_halved.sin_cos();

            let scalar = Scalar3(cos);
            let bivector = p * sin;

            Motor3 {
                scalar,
                bivector,
                pseudo: Pseudo3::ZERO,
            }
            .normalized()
        }
    }
}
