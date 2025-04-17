use crate::Scalar;

/// Plane is fundamental object in 3d projective geometric algebra.
/// All other objects are produced by combining planes.
#[repr(C)]
pub struct Plane3<T> {
    q0: T,
    q1: T,
    q2: T,
    q3: T,
}

impl<T> Plane3<T>
where
    T: Scalar,
{
    /// Creates a new plane.
    pub const fn new(q0: T, q1: T, q2: T, q3: T) -> Self {
        Plane3 { q0, q1, q2, q3 }
    }

    /// Returns a plane at infinity.
    pub const fn infinity() -> Self {
        Plane3 {
            q0: T::ZERO,
            q1: T::ZERO,
            q2: T::ZERO,
            q3: T::ZERO,
        }
    }
}
