use crate::Num;

use super::{
    elements::{regressive, BiVector3, Vector2},
    Plane3, Point2, Point3,
};

/// Line is fundamental object in 2d projective geometric algebra.
/// All other objects are produced by combining lines.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Line2<T>(Vector2<T>);

impl<T> Line2<T>
where
    T: Num,
{
    #[inline]
    pub(super) const fn vector(&self) -> Vector2<T> {
        self.0
    }

    #[inline]
    pub(super) const fn from_vector(vector: Vector2<T>) -> Self {
        Line2(vector)
    }

    /// Creates a new line from projective vector elements.
    #[inline]
    pub const fn new(e0: T, e1: T, e2: T) -> Self {
        Line2(Vector2 { e0, e1, e2 })
    }

    /// A vanishing line.
    /// Also known as the line at infinity.
    ///
    /// In 2D there's exactly one line at infinity.
    pub const INFINITY: Self = Line2(Vector2 {
        e0: T::ONE,
        e1: T::ZERO,
        e2: T::ZERO,
    });

    /// Returns true if this is a line at infinity.
    #[inline]
    pub fn is_ideal(&self) -> bool {
        self.0.e1 == T::ZERO && self.0.e2 == T::ZERO
    }

    /// Returns tangent of the line.
    #[inline]
    pub fn tangent(&self) -> T {
        -self.0.e1 / self.0.e2
    }

    /// Returns y coordinate where it intersects the y axis.
    #[inline]
    pub fn y0(&self) -> T {
        -self.0.e0 / self.0.e2
    }

    /// Returns x coordinate where it intersects the x axis.
    #[inline]
    pub fn x0(&self) -> T {
        -self.0.e0 / self.0.e1
    }

    /// Return the line as parameters of a linear equation ax + by + c = 0.
    #[inline]
    pub fn abc(&self) -> (T, T, T) {
        (self.0.e1, self.0.e2, self.0.e0)
    }

    /// Return the line from parameters of a linear equation ax + by + c = 0.
    #[inline]
    pub const fn from_abc(a: T, b: T, c: T) -> Self {
        Line2::new(c, a, b)
    }

    /// Normalizes the line.
    #[inline]
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns a normalized line.
    #[inline]
    pub fn normalized(&self) -> Self {
        Line2(self.0.normalized())
    }

    /// Make this line act as a reflector.
    ///
    /// Reflects a point.
    #[inline]
    pub fn reflect_point(&self, point: Point2<T>) -> Point2<T> {
        let (v, p) = self.vector() * point.bivector();

        let (_zero, a) = v * !self.vector();
        let b = p * !self.vector();

        Point2::from_bivector(a + b)
    }

    /// Make this line act as a reflector.
    ///
    /// Reflects a line.
    #[inline]
    pub fn reflect_line(&self, line: Line2<T>) -> Line2<T> {
        let (s, bv) = self.vector() * line.vector();

        let a = self.vector() * s;
        let (b, _zero) = bv * self.vector();

        Line2::from_vector(a + b)
    }

    /// Find the intersection point of two lines.
    #[inline]
    pub fn meet(&self, other: Line2<T>) -> Point2<T> {
        Point2::from_bivector(self.vector() ^ other.vector())
    }

    /// Find parallel line through the given point.
    #[inline]
    pub fn project_to(&self, point: Point2<T>) -> Line2<T> {
        let v = (self.vector() | point.bivector()) | !point.bivector();
        Line2::from_vector(v)
    }
}

/// Line in 3D is an intersection of two planes.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Line3<T>(BiVector3<T>);

impl<T> Line3<T>
where
    T: Num,
{
    /// A vanishing horizontal line.
    ///
    /// This assumes Y is up and Z is forward.
    pub const HORIZON: Self = Line3(BiVector3::new(
        T::ZERO,
        T::ZERO,
        T::ZERO,
        T::ZERO,
        T::ONE,
        T::ZERO,
    ));

    #[inline]
    pub(super) const fn bivector(&self) -> BiVector3<T> {
        self.0
    }

    #[inline]
    pub(super) const fn from_bivector(bivector: BiVector3<T>) -> Self {
        Line3(bivector)
    }

    /// Creates a new line from projective vector elements.
    #[inline]
    pub const fn new(e01: T, e02: T, e03: T, e12: T, e31: T, e23: T) -> Self {
        Line3(BiVector3::new(e01, e02, e03, e12, e31, e23))
    }

    /// Returns squared norm of the line.
    #[inline]
    pub fn norm2(&self) -> T {
        self.0.norm2()
    }

    /// Normalizes the line.
    #[inline]
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns a normalized line.
    #[inline]
    pub fn normalized(&self) -> Self {
        Line3(self.0.normalized())
    }

    /// Find the plane that contains this line and the given point.
    #[inline]
    pub fn join(&self, other: Point3<T>) -> Plane3<T> {
        let r = regressive(self.bivector(), other.trivector());
        Plane3::from_vector(r)
    }
}
