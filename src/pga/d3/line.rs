use crate::scalar::Num;

use super::{
    elements::{regressive, BiVector3},
    Plane3, Point3,
};

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
