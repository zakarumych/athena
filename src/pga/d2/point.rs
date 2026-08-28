use crate::scalar::Num;

use super::{
    elements::{regressive, BiVector2},
    Line2,
};

/// Two dimensional point
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Point2<T>(BiVector2<T>);

impl<T> Point2<T>
where
    T: Num,
{
    #[inline]
    pub(super) const fn bivector(&self) -> BiVector2<T> {
        self.0
    }

    #[inline]
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
    #[inline]
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
    ///
    /// Both coordinates must not be zero at the same time.
    #[inline]
    pub const fn ideal(x: T, y: T) -> Self {
        Point2(BiVector2 {
            e01: y,
            e20: x,
            e12: T::ZERO,
        })
    }

    /// Creates a new point from projective vector elements.
    #[inline]
    pub const fn new(e01: T, e20: T, e12: T) -> Self {
        Point2(BiVector2 { e01, e20, e12 })
    }

    /// Returns true if this is a point at infinity.
    #[inline]
    pub fn is_ideal(&self) -> bool {
        self.0.e12 == T::ZERO
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
    #[inline]
    pub fn coords(&self) -> (T, T) {
        (self.0.e20 / self.0.e12, self.0.e01 / self.0.e12)
    }

    /// Make this point act as a reflector.
    ///
    /// Reflects a point.
    #[inline]
    pub fn reflect_point(&self, point: Point2<T>) -> Point2<T> {
        let (s, bv) = self.bivector() * point.bivector();

        let a = s * !self.bivector();
        let (_zero, b) = bv * !self.bivector();

        Point2::from_bivector(a + b)
    }

    /// Make this point act as a reflector.
    ///
    /// Reflects a line.
    #[inline]
    pub fn reflect_line(&self, line: Line2<T>) -> Line2<T> {
        let (v, p) = self.bivector() * line.vector();

        let (a, _zero) = v * !self.bivector();
        let b = p * !self.bivector();

        Line2::from_vector(a + b)
    }

    /// Find the line through two points.
    #[inline]
    pub fn join(&self, other: Point2<T>) -> Line2<T> {
        Line2::from_vector(regressive(self.bivector(), other.bivector()))
    }

    /// Find orthogonal projection of this point to the line.
    #[inline]
    pub fn project_to(&self, line: Line2<T>) -> Point2<T> {
        let (_zero, bv) = !line.vector() * (self.bivector() | line.vector());
        Point2::from_bivector(bv)
    }
}
