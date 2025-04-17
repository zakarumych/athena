//! Contains the geometric algebra.
//!
//!
//!

mod line;
mod motor;
mod plane;
mod point;
mod reflector;
mod rotor;
mod screw;

/// Projective 2d vector.
#[repr(C)]
struct Vector2<T> {
    q0: T,
    q1: T,
    q2: T,
}

/// Projective 2d bi-vector.
#[repr(C)]
struct BiVector2<T> {
    q01: T,
    q02: T,
    q12: T,
}

/// Projective 3d vector.
#[repr(C)]
struct Vector3<T> {
    q0: T,
    q1: T,
    q2: T,
    q3: T,
}

/// Projective 3d bi-vector.
#[repr(C)]
struct BiVector3<T> {
    q01: T,
    q02: T,
    q03: T,
    q12: T,
    q13: T,
    q23: T,
}

/// Projective 3d tri-vector.
#[repr(C)]
struct TriVector3<T> {
    q012: T,
    q013: T,
    q023: T,
    q123: T,
}

trait WedgeProduct<Other> {
    type Output;
    fn wedge(&self, other: &Other) -> Self::Output;
}
