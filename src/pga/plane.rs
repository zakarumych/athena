use crate::{Num, Point3};

use super::elements::Vector3;

/// Plane is fundamental object in 3d projective geometric algebra.
/// All other objects are produced by combining planes.
///
/// Plane3's dual is a Point3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plane3<T>(Vector3<T>);

impl<T> Plane3<T>
where
    T: Num,
{
    #[inline]
    pub(super) const fn vector(&self) -> Vector3<T> {
        self.0
    }

    #[inline]
    pub(super) const fn from_vector(vector: Vector3<T>) -> Self {
        Plane3(vector)
    }

    /// A vanishing plane.
    /// Also known as the plane at infinity.
    pub const INFINITY: Self = Plane3(Vector3 {
        e0: T::ONE,
        e1: T::ZERO,
        e2: T::ZERO,
        e3: T::ZERO,
    });

    /// An XY plane.
    pub const XY: Self = Plane3(Vector3 {
        e0: T::ZERO,
        e1: T::ZERO,
        e2: T::ZERO,
        e3: T::ONE,
    });

    /// A YZ plane.
    pub const YZ: Self = Plane3(Vector3 {
        e0: T::ZERO,
        e1: T::ONE,
        e2: T::ZERO,
        e3: T::ZERO,
    });

    /// A XZ plane.
    pub const XZ: Self = Plane3(Vector3 {
        e0: T::ZERO,
        e1: T::ZERO,
        e2: T::ONE,
        e3: T::ZERO,
    });

    /// Returns true if this is a plane at infinity.
    #[inline]
    pub fn is_ideal(&self) -> bool {
        self.0.e1 == T::ZERO && self.0.e2 == T::ZERO && self.0.e3 == T::ZERO
    }

    /// Returns the normal direction of the plane.
    pub fn normal(&self) -> Point3<T> {
        Point3::ideal(self.0.e1, self.0.e2, self.0.e3)
    }

    /// Creates a new plane from projective vector elements.
    #[inline]
    pub const fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        Plane3(Vector3 { e0, e1, e2, e3 })
    }

    /// Return the plane as parameters of a linear equation ax + by + cz + d = 0.
    #[inline]
    pub fn abcd(&self) -> (T, T, T, T) {
        (self.0.e1, self.0.e2, self.0.e3, self.0.e0)
    }

    /// Return the plane from parameters of a linear equation ax + by + cz + d = 0.
    #[inline]
    pub const fn from_abcd(a: T, b: T, c: T, d: T) -> Self {
        Plane3::new(d, a, b, c)
    }

    /// Normalizes the plane.
    #[inline]
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns a normalized plane.
    #[inline]
    pub fn normalized(&self) -> Self {
        Plane3(self.0.normalized())
    }
}
