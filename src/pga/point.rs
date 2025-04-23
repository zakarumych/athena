use crate::Scalar;

use super::elements::{BiVector2, TriVector3};

/// Two dimensional point
#[repr(transparent)]
pub struct Point2<T>(BiVector2<T>);

impl<T> Point2<T>
where
    T: Scalar,
{
    /// Origin point.
    pub const ORIGIN: Self = Point2(BiVector2 {
        e12: T::ONE,
        e01: T::ZERO,
        e02: T::ZERO,
    });

    /// Creates a new point at the given coordinates.
    pub const fn at(x: T, y: T) -> Self {
        Point2(BiVector2 {
            e12: T::ONE,
            e01: x,
            e02: y,
        })
    }

    /// Returns a new ideal point.
    /// Also known as the point at infinity.
    /// The point at infinity is a point located at infinity in the direction of the line
    ///
    /// Thus it can be used to represent a direction in 2D space.
    pub const fn ideal(e01: T, e02: T) -> Self {
        Point2(BiVector2 {
            e12: T::ZERO,
            e01: e01,
            e02: e02,
        })
    }

    /// Creates a new point from projective vector elements.
    pub const fn new(e12: T, e01: T, e02: T) -> Self {
        Point2(BiVector2 { e12, e01, e02 })
    }

    /// Normalizes the point.
    pub fn normalized(self) -> Self {
        if self.0.e12 == T::ZERO || self.0.e12 == T::ONE {
            self
        } else {
            Point2(BiVector2 {
                e12: T::ONE,
                e01: self.0.e01 / self.0.e12,
                e02: self.0.e02 / self.0.e12,
            })
        }
    }

    pub fn normailze(&mut self) {
        if self.0.e12 == T::ZERO || self.0.e12 == T::ONE {
            return;
        } else {
            self.0.e01 /= self.0.e12;
            self.0.e02 /= self.0.e12;
            self.0.e12 = T::ONE;
        }
    }
}

#[repr(transparent)]
pub struct Point3<T>(TriVector3<T>);

impl<T> Point3<T>
where
    T: Scalar,
{
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
    pub const fn ideal(e032: T, e013: T, e021: T) -> Self {
        Point3(TriVector3 {
            e123: T::ZERO,
            e032,
            e013,
            e021,
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
}
