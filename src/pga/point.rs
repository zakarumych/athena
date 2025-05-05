use core::ops::Mul;

use crate::Num;

use super::{
    elements::{regressive, BiVector2, Dual, TriVector3},
    Line2, Motor2,
};

/// Two dimensional point
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Point2<T>(BiVector2<T>);

impl<T> Point2<T>
where
    T: Num,
{
    pub(super) const fn bivector(&self) -> BiVector2<T> {
        self.0
    }

    pub(super) const fn from_bivector(bivector: BiVector2<T>) -> Self {
        Point2(bivector)
    }

    /// Origin point.
    pub const ORIGIN: Self = Point2(BiVector2 {
        e01: T::ZERO,
        e20: T::ZERO,
        e12: T::ONE,
    });

    /// Creates a new point at the given coordinates.
    pub const fn at(x: T, y: T) -> Self {
        Point2(BiVector2 {
            e01: y,
            e20: x,
            e12: T::ONE,
        })
    }

    /// Returns a new ideal point.
    /// Also known as the point at infinity.
    /// The point at infinity is a point located at infinity in the direction of the line
    ///
    /// Thus it can be used to represent a direction in 2D space.
    pub const fn ideal(x: T, y: T) -> Self {
        Point2(BiVector2 {
            e01: y,
            e20: x,
            e12: T::ZERO,
        })
    }

    /// Creates a new point from projective vector elements.
    pub const fn new(e01: T, e20: T, e12: T) -> Self {
        Point2(BiVector2 { e01, e20, e12 })
    }

    /// Returns true if this is a point at infinity.
    pub fn is_ideal(&self) -> bool {
        self.0.e12 == T::ZERO
    }

    /// Returns norm of the point.
    pub fn norm(&self) -> T {
        self.0.norm()
    }

    /// Normalizes the point.
    ///
    /// Does not affect points at infinity.
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns same point, but normalized.
    pub fn normalized(&self) -> Self {
        Point2(self.0.normalized())
    }

    /// Returns the coordinates of the point.
    pub const fn coords(&self) -> (T, T) {
        (self.0.e20, self.0.e01)
    }

    /// Make this line act as a reflector.
    ///
    /// Reflects a point.
    pub fn reflect_point(&self, point: Point2<T>) -> Point2<T> {
        let (s, bv) = self.bivector() * point.bivector();

        let a = (!self.bivector()) * s;
        let (_zero, b) = bv * !self.bivector();

        Point2::from_bivector(a + b)
    }

    /// Make this line act as a reflector.
    ///
    /// Reflects a line.
    pub fn reflect_line(&self, line: Line2<T>) -> Line2<T> {
        let (v, p) = self.bivector() * line.vector();

        let (a, _zero) = v * !self.bivector();
        let b = p * !self.bivector();

        Line2::from_vector(a + b)
    }

    /// Find the line through two points.
    pub fn join(&self, other: Point2<T>) -> Line2<T> {
        Line2::from_vector(regressive(self.bivector(), other.bivector()))
    }

    /// Find orthogonal projection of this point to the line.
    pub fn project_to(&self, line: Line2<T>) -> Point2<T> {
        let (_zero, bv) = !line.vector() * (self.bivector() | line.vector());
        Point2::from_bivector(bv)
    }
}

/// Three dimensional point
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Point3<T>(TriVector3<T>);

impl<T> Point3<T>
where
    T: Num,
{
    pub(super) const fn trivector(&self) -> TriVector3<T> {
        self.0
    }

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
    pub const fn ideal(x: T, y: T, z: T) -> Self {
        Point3(TriVector3 {
            e123: T::ZERO,
            e032: x,
            e013: y,
            e021: z,
        })
    }

    /// Creates a new point from projective vector elements.
    pub const fn new(e123: T, e032: T, e013: T, e021: T) -> Self {
        Point3(TriVector3 {
            e123,
            e032,
            e013,
            e021,
        })
    }

    /// Returns true if this is a point at infinity.
    pub fn is_ideal(&self) -> bool {
        self.0.e123 == T::ZERO
    }

    /// Normalizes the point.
    ///
    /// Does not affect points at infinity.
    pub fn normalize(&mut self) {
        let norm = self.0.e123;
        if norm != T::ZERO {
            self.0.e032 /= norm;
            self.0.e013 /= norm;
            self.0.e021 /= norm;
            self.0.e123 = T::ONE;
        }
    }

    /// Returns same point, but normalized.
    pub fn normalized(&self) -> Self {
        let mut point = *self;
        point.normalize();
        point
    }

    /// Returns the coordinates of the point.
    pub const fn coords(&self) -> (T, T, T) {
        (self.0.e032, self.0.e013, self.0.e021)
    }
}
