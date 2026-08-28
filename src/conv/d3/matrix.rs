//! Conversions between 3D PGA rigid-motion operators (Motor3/Rotor3/Translator3) and lina's [`Matrix`](crate::Matrix) type.

use crate::{Matrix3, Matrix4, Motor3, Num, Point3, Rotor3, Translator3};

/// Converts a rotor to a 3x3 rotation matrix.
///
/// A rotor never translates, so the result never needs a homogeneous row.
impl<T> From<Rotor3<T>> for Matrix3<T>
where
    T: Num,
{
    #[inline]
    fn from(rotor: Rotor3<T>) -> Self {
        let (x0, y0, z0) = rotor
            .move_point(Point3::at(T::ONE, T::ZERO, T::ZERO))
            .coords();
        let (x1, y1, z1) = rotor
            .move_point(Point3::at(T::ZERO, T::ONE, T::ZERO))
            .coords();
        let (x2, y2, z2) = rotor
            .move_point(Point3::at(T::ZERO, T::ZERO, T::ONE))
            .coords();
        Matrix3::from_column_arrays([[x0, y0, z0], [x1, y1, z1], [x2, y2, z2]])
    }
}

/// Converts a 3x3 matrix to a rotor.
///
/// This assumes the input matrix is a valid rotation matrix; no
/// orthogonality check is performed.
impl<T> From<Matrix3<T>> for Rotor3<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix3<T>) -> Self {
        let a = matrix.arrays();
        let motor = Motor3::reconstruct(
            [
                Point3::ORIGIN,
                Point3::at(T::ONE, T::ZERO, T::ZERO),
                Point3::at(T::ZERO, T::ONE, T::ZERO),
            ],
            [
                Point3::ORIGIN,
                Point3::at(a[0][0], a[0][1], a[0][2]),
                Point3::at(a[1][0], a[1][1], a[1][2]),
            ],
        );
        Rotor3::from(motor)
    }
}

/// Converts a translator to a homogeneous 4x4 matrix.
impl<T> From<Translator3<T>> for Matrix4<T>
where
    T: Num,
{
    #[inline]
    fn from(translator: Translator3<T>) -> Self {
        let (dx, dy, dz) = translator.offset();
        Matrix4::from_column_arrays([
            [T::ONE, T::ZERO, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE, T::ZERO],
            [dx, dy, dz, T::ONE],
        ])
    }
}

/// Converts a homogeneous 4x4 matrix to a translator.
///
/// Only the translation column is read; the linear block is ignored.
impl<T> From<Matrix4<T>> for Translator3<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix4<T>) -> Self {
        let col3 = matrix.arrays()[3];
        Translator3::from_offset(col3[0], col3[1], col3[2])
    }
}

/// Converts a motor to a homogeneous 4x4 affine transform matrix.
///
/// This transforms the origin and the three unit basis points through the
/// motor to recover the linear and translation parts, costing several
/// sandwich products and normalizations. Fine for one-off or import-time
/// conversions, but not recommended in a per-frame hot loop.
impl<T> From<Motor3<T>> for Matrix4<T>
where
    T: Num,
{
    #[inline]
    fn from(motor: Motor3<T>) -> Self {
        let (ox, oy, oz) = motor.move_point(Point3::ORIGIN).coords();
        let (x0, y0, z0) = motor
            .move_point(Point3::at(T::ONE, T::ZERO, T::ZERO))
            .coords();
        let (x1, y1, z1) = motor
            .move_point(Point3::at(T::ZERO, T::ONE, T::ZERO))
            .coords();
        let (x2, y2, z2) = motor
            .move_point(Point3::at(T::ZERO, T::ZERO, T::ONE))
            .coords();
        Matrix4::from_column_arrays([
            [x0 - ox, y0 - oy, z0 - oz, T::ZERO],
            [x1 - ox, y1 - oy, z1 - oz, T::ZERO],
            [x2 - ox, y2 - oy, z2 - oz, T::ZERO],
            [ox, oy, oz, T::ONE],
        ])
    }
}

/// Converts a homogeneous 4x4 affine transform matrix to a motor.
///
/// This reconstructs the motor via [`Motor3::reconstruct`], which costs
/// several sandwich products and normalizations. Fine for one-off or
/// import-time conversions, but not recommended in a per-frame hot loop.
impl<T> From<Matrix4<T>> for Motor3<T>
where
    T: Num,
{
    #[inline]
    fn from(matrix: Matrix4<T>) -> Self {
        let a = matrix.arrays();
        let (tx, ty, tz) = (a[3][0], a[3][1], a[3][2]);
        let b0 = Point3::at(tx, ty, tz);
        let b1 = Point3::at(tx + a[0][0], ty + a[0][1], tz + a[0][2]);
        let b2 = Point3::at(tx + a[1][0], ty + a[1][1], tz + a[1][2]);
        Motor3::reconstruct(
            [
                Point3::ORIGIN,
                Point3::at(T::ONE, T::ZERO, T::ZERO),
                Point3::at(T::ZERO, T::ONE, T::ZERO),
            ],
            [b0, b1, b2],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Rotor3, Vector3, Vector4};

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
    fn rotor_matrix_round_trip() {
        // Deliberately avoids an exact 180-degree case: `Matrix3 -> Rotor3`
        // goes through `Motor3::reconstruct`, which has a known NaN bug at
        // exact 180-degree configurations (see the `#[ignore]`d tests next to
        // `Motor3::reconstruct` in `pga::d3::motor`). `5 * FRAC_PI_6` (150
        // degrees) exercises a steep angle close to that singularity without
        // hitting it.
        let cases: [(f32, [f32; 3]); 4] = [
            (core::f32::consts::FRAC_PI_4, [1.0, 0.0, 0.0]),
            (core::f32::consts::FRAC_PI_2, [0.0, 1.0, 0.0]),
            (-core::f32::consts::FRAC_PI_6, [0.0, 0.0, 1.0]),
            (5.0 * core::f32::consts::FRAC_PI_6, [0.0, 1.0, 0.0]),
        ];

        for &(angle, axis) in &cases {
            let rotor = Rotor3::from_axis_angle(Point3::at(axis[0], axis[1], axis[2]), angle);
            let matrix: Matrix3<f32> = rotor.into();
            let rotor2: Rotor3<f32> = matrix.into();

            let samples = [
                Point3::at(1.0f32, 0.0, 0.0),
                Point3::at(0.0, 2.0, 3.0),
                Point3::at(-1.5, 0.5, 2.0),
            ];

            for &sample in &samples {
                let expected = rotor.move_point(sample);
                let actual = rotor2.move_point(sample);
                assert_point_approx(actual, expected);

                let v: Vector3<f32> = sample.into();
                let mv = matrix * v;
                let (ex, ey, ez) = expected.coords();
                assert!(approx(mv.x, ex));
                assert!(approx(mv.y, ey));
                assert!(approx(mv.z, ez));
            }
        }
    }

    #[test]
    fn translator_matrix_round_trip() {
        let translator = Translator3::from_offset(3.0f32, -4.0, 1.5);
        let matrix: Matrix4<f32> = translator.into();
        let translator2: Translator3<f32> = matrix.into();

        let (dx, dy, dz) = translator2.offset();
        assert!(approx(dx, 3.0));
        assert!(approx(dy, -4.0));
        assert!(approx(dz, 1.5));
    }

    #[test]
    fn motor_matrix_round_trip() {
        let rotor = Rotor3::from_axis_angle(
            Point3::at(0.0f32, 0.0, 1.0),
            core::f32::consts::FRAC_PI_2,
        );
        let motor: Motor3<f32> = rotor.into();
        let translator = Translator3::from_offset(3.0f32, 4.0, -2.0);
        let translator_motor: Motor3<f32> = translator.into();
        let motor = translator_motor * motor;

        let matrix: Matrix4<f32> = motor.into();
        std::eprintln!("matrix = {:?}", matrix);
        std::eprintln!("motor O = {:?}", motor.move_point(Point3::ORIGIN));
        std::eprintln!("motor ex = {:?}", motor.move_point(Point3::at(1.0, 0.0, 0.0)));
        std::eprintln!("motor ey = {:?}", motor.move_point(Point3::at(0.0, 1.0, 0.0)));
        let motor2: Motor3<f32> = matrix.into();
        std::eprintln!("motor2 O = {:?}", motor2.move_point(Point3::ORIGIN));
        std::eprintln!("motor2 ex = {:?}", motor2.move_point(Point3::at(1.0, 0.0, 0.0)));
        std::eprintln!("motor2 ey = {:?}", motor2.move_point(Point3::at(0.0, 1.0, 0.0)));

        let samples = [
            Point3::at(1.0f32, 0.0, 0.0),
            Point3::at(2.0, 3.0, -1.0),
            Point3::at(-1.5, 0.5, 4.0),
        ];

        for &sample in &samples {
            let expected = motor.move_point(sample);
            let actual = motor2.move_point(sample);
            assert_point_approx(actual, expected);

            let (sx, sy, sz) = sample.coords();
            let hv = Vector4::new(sx, sy, sz, 1.0);
            let mv = matrix * hv;
            let (ex, ey, ez) = expected.coords();
            assert!(approx(mv.x, ex));
            assert!(approx(mv.y, ey));
            assert!(approx(mv.z, ez));
        }
    }
}
