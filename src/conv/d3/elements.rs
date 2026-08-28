//! Conversions between 3D PGA geometric objects (points, lines, planes) and lina's [`Vector`](crate::Vector) type.

use crate::{Line3, Num, Plane3, Point3, Vector3, Vector4};

/// Converts a point to its affine coordinates.
///
/// An ideal point (a point at infinity) has a zero homogeneous weight, so
/// [`Point3::coords`] divides by zero for it, producing infinite or NaN
/// components. This is existing behavior of [`Point3::coords`], not
/// something introduced by this conversion.
impl<T> From<Point3<T>> for Vector3<T>
where
    T: Num,
{
    #[inline]
    fn from(point: Point3<T>) -> Self {
        let (x, y, z) = point.coords();
        Vector3::new(x, y, z)
    }
}

impl<T> From<Vector3<T>> for Point3<T>
where
    T: Num,
{
    #[inline]
    fn from(vector: Vector3<T>) -> Self {
        Point3::at(vector.x, vector.y, vector.z)
    }
}

/// Converts a plane to its implicit equation coefficients `(a, b, c, d)` for `ax + by + cz + d = 0`.
impl<T> From<Plane3<T>> for Vector4<T>
where
    T: Num,
{
    #[inline]
    fn from(plane: Plane3<T>) -> Self {
        let (a, b, c, d) = plane.abcd();
        Vector4::new(a, b, c, d)
    }
}

impl<T> From<Vector4<T>> for Plane3<T>
where
    T: Num,
{
    #[inline]
    fn from(vector: Vector4<T>) -> Self {
        Plane3::from_abcd(vector.x, vector.y, vector.z, vector.w)
    }
}

/// Converts a line to its `(moment, direction)` pair.
///
/// The moment is the line's ideal part (`e01`, `e02`, `e03`), the direction
/// is its Euclidean part (`e12`, `e31`, `e23`). Both components are
/// `Vector3`, so a swapped argument order will not be caught at compile
/// time: pay attention to the `(moment, direction)` ordering documented
/// here and on the inverse conversion below.
impl<T> From<Line3<T>> for (Vector3<T>, Vector3<T>)
where
    T: Num,
{
    /// Returns `(moment, direction)`: the moment is the line's ideal part
    /// (e01, e02, e03), the direction is its Euclidean part (e12, e31, e23).
    #[inline]
    fn from(line: Line3<T>) -> Self {
        let bv = line.bivector();
        (
            Vector3::new(bv.e01, bv.e02, bv.e03),
            Vector3::new(bv.e12, bv.e31, bv.e23),
        )
    }
}

impl<T> From<(Vector3<T>, Vector3<T>)> for Line3<T>
where
    T: Num,
{
    /// Builds a line from `(moment, direction)`, the inverse of the
    /// `Line3 -> (Vector3, Vector3)` conversion.
    #[inline]
    fn from((moment, direction): (Vector3<T>, Vector3<T>)) -> Self {
        Line3::new(
            moment.x,
            moment.y,
            moment.z,
            direction.x,
            direction.y,
            direction.z,
        )
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
        let p = Point3::at(3.0f32, -2.5, 7.0);
        let v: Vector3<f32> = p.into();
        assert!(approx(v.x, 3.0));
        assert!(approx(v.y, -2.5));
        assert!(approx(v.z, 7.0));

        let p2: Point3<f32> = v.into();
        let (x, y, z) = p2.coords();
        assert!(approx(x, 3.0));
        assert!(approx(y, -2.5));
        assert!(approx(z, 7.0));
    }

    #[test]
    fn plane_vector_round_trip() {
        let plane = Plane3::from_abcd(2.0f32, -1.0, 0.5, 5.0);
        let v: Vector4<f32> = plane.into();
        assert!(approx(v.x, 2.0));
        assert!(approx(v.y, -1.0));
        assert!(approx(v.z, 0.5));
        assert!(approx(v.w, 5.0));

        let plane2: Plane3<f32> = v.into();
        let (a, b, c, d) = plane2.abcd();
        assert!(approx(a, 2.0));
        assert!(approx(b, -1.0));
        assert!(approx(c, 0.5));
        assert!(approx(d, 5.0));
    }

    #[test]
    fn line_vector_pair_round_trip() {
        // Distinct, recognizable values so a moment/direction swap would be caught.
        let line = Line3::new(1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0);

        let (moment, direction): (Vector3<f32>, Vector3<f32>) = line.into();
        assert!(approx(moment.x, 1.0));
        assert!(approx(moment.y, 2.0));
        assert!(approx(moment.z, 3.0));
        assert!(approx(direction.x, 4.0));
        assert!(approx(direction.y, 5.0));
        assert!(approx(direction.z, 6.0));

        let line2: Line3<f32> = (moment, direction).into();
        assert_eq!(line, line2);
    }
}
