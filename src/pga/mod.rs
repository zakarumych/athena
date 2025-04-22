//! Contains the geometric algebra.
//!
//!
//!

use core::ops::Mul;

use crate::Scalar;

mod line;
mod motor;
mod plane;
mod point;
mod reflector;
mod rotor;
mod screw;

#[repr(C)]
struct Vector2<T> {
    e0: T,
    e1: T,
    e2: T,
}

#[repr(C)]
struct Vector3<T> {
    e0: T,
    e1: T,
    e2: T,
    e3: T,
}

#[repr(C)]
struct BiVector2<T> {
    e12: T,
    e01: T,
    e02: T,
}

#[repr(C)]
struct EBiVector3<T> {
    e23: T,
    e31: T,
    e12: T,
}

#[repr(C)]
struct XBiVector3<T> {
    e01: T,
    e02: T,
    e03: T,
}

#[repr(C)]
struct BiVector3<T> {
    e: EBiVector3<T>,
    x: XBiVector3<T>,
}

#[repr(C)]
struct TriVector3<T> {
    e123: T,
    e032: T,
    e013: T,
    e021: T,
}

#[repr(transparent)]
struct Pseudo2<T> {
    e012: T,
}

#[repr(transparent)]
struct Pseudo3<T> {
    e0123: T,
}

trait InnerProduct<Other> {
    type Output;
    fn inner(&self, other: &Other) -> Self::Output;
}

trait OuterProduct<Other> {
    type Output;
    fn outer(&self, other: &Other) -> Self::Output;
}

trait VProduct<Other> {
    type Output;
    fn v(&self, other: &Other) -> Self::Output;
}

trait GeometricProduct<Other> {
    type Output;
    fn geometric(&self, other: &Other) -> Self::Output;
}
