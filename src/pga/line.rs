use crate::Scalar;

use super::{BiVector3, Vector2};

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
    pub const INFINITY: Self = Line2(Vector2 {
        e0: T::ONE,
        e1: T::ZERO,
        e2: T::ZERO,
    });
}

/// Line in 3D is an intersection of two planes.
#[repr(C)]
pub struct Line3<T>(BiVector3<T>);
