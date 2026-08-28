//! Conversions between 2D PGA objects and lina's [`Vector`](crate::Vector)/[`Matrix`](crate::Matrix) types.

use crate::{Line2, Matrix2, Matrix3, Motor2, Num, Point2, Rotor2, Translator2, Vector2, Vector3};

/// Converts a point to its affine coordinates.
///
/// An ideal point (a point at infinity) has a zero homogeneous weight, so
/// [`Point2::coords`] divides by zero for it, producing infinite or NaN
/// components. This is existing behavior of [`Point2::coords`], not
/// something introduced by this conversion.
impl<T> From<Point2<T>> for Vector2<T>
where
    T: Num,
{
    #[inline]
    fn from(point: Point2<T>) -> Self {
        let (x, y) = point.coords();
        Vector2::new(x, y)
    }
}

impl<T> From<Vector2<T>> for Point2<T>
where
    T: Num,
{
    #[inline]
    fn from(vector: Vector2<T>) -> Self {
        Point2::at(vector.x, vector.y)
    }
}

/// Converts a line to its implicit equation coefficients `(a, b, c)` for `ax + by + c = 0`.
impl<T> From<Line2<T>> for Vector3<T>
where
    T: Num,
{
    #[inline]
    fn from(line: Line2<T>) -> Self {
        let (a, b, c) = line.abc();
        Vector3::new(a, b, c)
    }
}

impl<T> From<Vector3<T>> for Line2<T>
where
    T: Num,
{
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        Line2::from_abc(vector.x, vector.y, vector.z)
    }
}

/// Converts a rotor to a 2x2 rotation matrix.
///
/// A rotor never translates, so the result never needs a homogeneous row.
impl<T> From<Rotor2<T>> for Matrix2<T>
where
    T: Num,
{
    #[inline]
    fn from(rotor: Rotor2<T>) -> Self {
        let (x0, y0) = rotor.move_point(Point2::at(T::ONE, T::ZERO)).coords();
        let (x1, y1) = rotor.move_point(Point2::at(T::ZERO, T::ONE)).coords();
        Matrix2::from_column_arrays([[x0, y0], [x1, y1]])
    }
}

/// Converts a 2x2 matrix to a rotor.
///
/// This assumes the input matrix is a valid rotation matrix; no
/// orthogonality check is performed.
impl<T> From<Matrix2<T>> for Rotor2<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix2<T>) -> Self {
        let col0 = matrix.arrays()[0];
        let angle = T::atan2(col0[1], col0[0]);
        Rotor2::from_angle(angle)
    }
}

/// Converts a translator to a homogeneous 3x3 matrix.
impl<T> From<Translator2<T>> for Matrix3<T>
where
    T: Num,
{
    #[inline]
    fn from(translator: Translator2<T>) -> Self {
        let (dx, dy) = translator.offset();
        Matrix3::from_column_arrays([
            [T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO],
            [dx, dy, T::ONE],
        ])
    }
}

/// Converts a homogeneous 3x3 matrix to a translator.
///
/// Only the translation column is read; the linear block is ignored.
impl<T> From<Matrix3<T>> for Translator2<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix3<T>) -> Self {
        let col2 = matrix.arrays()[2];
        Translator2::from_offset(col2[0], col2[1])
    }
}

/// Converts a motor to a homogeneous 3x3 affine transform matrix.
///
/// This transforms the origin and the two unit basis points through the
/// motor to recover the linear and translation parts, costing several
/// sandwich products and normalizations. Fine for one-off or import-time
/// conversions, but not recommended in a per-frame hot loop.
impl<T> From<Motor2<T>> for Matrix3<T>
where
    T: Num,
{
    #[inline]
    fn from(motor: Motor2<T>) -> Self {
        let (ox, oy) = motor.move_point(Point2::ORIGIN).coords();
        let (x0, y0) = motor.move_point(Point2::at(T::ONE, T::ZERO)).coords();
        let (x1, y1) = motor.move_point(Point2::at(T::ZERO, T::ONE)).coords();
        Matrix3::from_column_arrays([
            [x0 - ox, y0 - oy, T::ZERO],
            [x1 - ox, y1 - oy, T::ZERO],
            [ox, oy, T::ONE],
        ])
    }
}

/// Converts a homogeneous 3x3 affine transform matrix to a motor.
///
/// This reconstructs the motor via [`Motor2::reconstruct`], which costs
/// several sandwich products and normalizations. Fine for one-off or
/// import-time conversions, but not recommended in a per-frame hot loop.
impl<T> From<Matrix3<T>> for Motor2<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix3<T>) -> Self {
        let col0 = matrix.arrays()[0];
        let col2 = matrix.arrays()[2];
        let b0 = Point2::at(col2[0], col2[1]);
        let b1 = Point2::at(col2[0] + col0[0], col2[1] + col0[1]);
        Motor2::reconstruct([Point2::ORIGIN, Point2::at(T::ONE, T::ZERO)], [b0, b1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn point_vector_round_trip() {
        let p = Point2::at(3.0f32, -2.5);
        let v: Vector2<f32> = p.into();
        assert!(approx(v.x, 3.0));
        assert!(approx(v.y, -2.5));

        let p2: Point2<f32> = v.into();
        let (x, y) = p2.coords();
        assert!(approx(x, 3.0));
        assert!(approx(y, -2.5));
    }

    #[test]
    fn line_vector_round_trip() {
        let line = Line2::from_abc(2.0f32, -1.0, 5.0);
        let v: Vector3<f32> = line.into();
        assert!(approx(v.x, 2.0));
        assert!(approx(v.y, -1.0));
        assert!(approx(v.z, 5.0));

        let line2: Line2<f32> = v.into();
        let (a, b, c) = line2.abc();
        assert!(approx(a, 2.0));
        assert!(approx(b, -1.0));
        assert!(approx(c, 5.0));
    }

    #[test]
    fn rotor_matrix_round_trip() {
        let angles: [f32; 5] = [
            0.0,
            core::f32::consts::FRAC_PI_4,
            core::f32::consts::FRAC_PI_2,
            core::f32::consts::PI,
            -core::f32::consts::FRAC_PI_6,
        ];

        for &angle in &angles {
            let rotor = Rotor2::from_angle(angle);
            let matrix: Matrix2<f32> = rotor.into();
            let rotor2: Rotor2<f32> = matrix.into();

            let samples = [Point2::at(1.0f32, 0.0), Point2::at(2.0, 3.0), Point2::at(-1.5, 0.5)];

            for &sample in &samples {
                let expected = rotor.move_point(sample);
                let (ex, ey) = expected.coords();

                let actual = rotor2.move_point(sample);
                let (ax, ay) = actual.coords();
                assert!(approx(ax, ex), "rotor round trip mismatch at angle {angle}");
                assert!(approx(ay, ey), "rotor round trip mismatch at angle {angle}");

                let v: Vector2<f32> = sample.into();
                let mv = matrix * v;
                assert!(approx(mv.x, ex), "matrix apply mismatch at angle {angle}");
                assert!(approx(mv.y, ey), "matrix apply mismatch at angle {angle}");
            }
        }
    }

    #[test]
    fn translator_matrix_round_trip() {
        let translator = Translator2::from_offset(3.0f32, -4.0);
        let matrix: Matrix3<f32> = translator.into();
        let translator2: Translator2<f32> = matrix.into();

        let (dx, dy) = translator2.offset();
        assert!(approx(dx, 3.0));
        assert!(approx(dy, -4.0));
    }

    #[test]
    fn motor_matrix_round_trip() {
        let rotor = Rotor2::from_angle(core::f32::consts::FRAC_PI_2);
        let motor: Motor2<f32> = rotor.into();
        let translator = Translator2::from_offset(3.0f32, 4.0);
        let translator_motor: Motor2<f32> = translator.into();
        let motor = translator_motor * motor;

        let matrix: Matrix3<f32> = motor.into();
        let motor2: Motor2<f32> = matrix.into();

        let samples = [Point2::at(1.0f32, 0.0), Point2::at(2.0, 3.0), Point2::at(-1.5, 0.5)];

        for &sample in &samples {
            let expected = motor.move_point(sample);
            let (ex, ey) = expected.coords();

            let actual = motor2.move_point(sample);
            let (ax, ay) = actual.coords();
            assert!(approx(ax, ex));
            assert!(approx(ay, ey));

            let (sx, sy) = sample.coords();
            let hv = Vector3::new(sx, sy, 1.0);
            let mv = matrix * hv;
            assert!(approx(mv.x, ex));
            assert!(approx(mv.y, ey));
        }
    }
}
