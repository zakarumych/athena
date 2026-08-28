use crate::scalar::Num;

use super::{
    elements::{regressive, regressive3, TriVector3},
    Line3, Plane3,
};

/// Three dimensional point
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Point3<T>(TriVector3<T>);

impl<T> Point3<T>
where
    T: Num,
{
    #[inline]
    pub(super) const fn trivector(&self) -> TriVector3<T> {
        self.0
    }

    #[inline]
    pub(super) const fn from_trivector(trivector: TriVector3<T>) -> Self {
        Point3(trivector)
    }

    /// Origin point.
    pub const ORIGIN: Self = Point3(TriVector3 {
        e123: T::ONE,
        e032: T::ZERO,
        e013: T::ZERO,
        e021: T::ZERO,
    });

    /// Creates a new point at the given coordinates.
    #[inline]
    pub const fn at(x: T, y: T, z: T) -> Self {
        Point3(TriVector3 {
            e123: T::ONE,
            e032: x,
            e013: y,
            e021: z,
        })
    }

    /// Returns a new ideal point.
    /// Also known as the point at infinity.
    /// The point at infinity is a point located at infinity in the direction of the line
    ///
    /// Thus it can be used to represent a direction in 3D space.
    #[inline]
    pub const fn ideal(x: T, y: T, z: T) -> Self {
        Point3(TriVector3 {
            e123: T::ZERO,
            e032: x,
            e013: y,
            e021: z,
        })
    }

    /// Creates a new point from projective vector elements.
    #[inline]
    pub const fn new(e123: T, e032: T, e013: T, e021: T) -> Self {
        Point3(TriVector3 {
            e123,
            e032,
            e013,
            e021,
        })
    }

    /// Returns true if this is a point at infinity.
    #[inline]
    pub fn is_ideal(&self) -> bool {
        self.0.e123 == T::ZERO
    }

    /// Normalizes the point.
    ///
    /// Does not affect points at infinity.
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns same point, but normalized.
    pub fn normalized(&self) -> Self {
        Point3(self.0.normalized())
    }

    /// Returns the coordinates of the point.
    #[inline]
    pub const fn coords(&self) -> (T, T, T) {
        (self.0.e032, self.0.e013, self.0.e021)
    }

    /// Make this point act as a reflector.
    ///
    /// Reflects a point.
    #[inline]
    pub fn reflect_point(&self, point: Point3<T>) -> Point3<T> {
        let (s, bv) = self.trivector() * point.trivector();

        let a = s * !self.trivector();
        let b = bv * !self.trivector();

        Point3::from_trivector(a + b)
    }

    /// Make this point act as a reflector.
    ///
    /// Reflects a line.
    #[inline]
    pub fn reflect_line(&self, line: Line3<T>) -> Line3<T> {
        let (v, tv) = self.trivector() * line.bivector();

        let (a, _zero) = v * !self.trivector();
        let (_zero, b) = tv * !self.trivector();

        Line3::from_bivector(a + b)
    }

    /// Make this point act as a reflector.
    ///
    /// Reflects a plane.
    #[inline]
    pub fn reflect_plane(&self, plane: Plane3<T>) -> Plane3<T> {
        let (v, tv) = self.trivector() * plane.vector();

        let (a, _zero) = v * !self.trivector();
        let b = tv * !self.trivector();

        Plane3::from_vector(a + b)
    }

    /// Find the line through two points.
    #[inline]
    pub fn join(&self, other: Point3<T>) -> Line3<T> {
        Line3::from_bivector(regressive(self.trivector(), other.trivector()))
    }

    /// Find the plane through three points.
    #[inline]
    pub fn join3(&self, other: Point3<T>, another: Point3<T>) -> Plane3<T> {
        Plane3::from_vector(regressive3(
            self.trivector(),
            other.trivector(),
            another.trivector(),
        ))
    }

    /// Find orthogonal projection of this point to the line.
    #[inline]
    pub fn project_to(&self, line: Line3<T>) -> Point3<T> {
        let (_zero, tv) = !line.bivector() * (self.trivector() | line.bivector());
        Point3::from_trivector(tv)
    }
}
