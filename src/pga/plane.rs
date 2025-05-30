use crate::Num;

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
    pub(super) const fn vector(&self) -> Vector3<T> {
        self.0
    }

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

    /// Creates a new plane from projective vector elements.
    pub const fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        Plane3(Vector3 { e0, e1, e2, e3 })
    }

    /// Returns norm of the line.
    pub fn norm(&self) -> T {
        self.0.norm()
    }

    /// Returns squared norm of the line.
    pub fn norm2(&self) -> T {
        self.0.norm2()
    }

    /// Normalizes the line.
    pub fn normalize(&mut self) {
        self.0.normalize();
    }

    /// Returns a normalized line.
    pub fn normalized(&self) -> Self {
        Plane3(self.0.normalized())
    }
}
