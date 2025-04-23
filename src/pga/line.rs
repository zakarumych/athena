use crate::Scalar;

use super::elements::{BiVector3, Vector2};


/// Plane is fundamental object in 2d projective geometric algebra.
/// All other objects are produced by combining planes.
#[repr(C)]
pub struct Line2<T>(Vector2<T>);

impl<T> Line2<T>
where
    T: Scalar,
{
    /// Creates a new line from projective vector elements.
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
}

/// Line in 3D is an intersection of two planes.
#[repr(C)]
pub struct Line3<T>(BiVector3<T>);


impl<T> Line3<T>
where
    T: Scalar,
{
    /// Creates a new line from projective vector elements.
    pub const fn new(e01: T, e02: T, e03: T, e12: T, e31: T, e23: T) -> Self {
        Line3(BiVector3::new(e01, e02, e03, e12, e31, e23))
    }

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
}
