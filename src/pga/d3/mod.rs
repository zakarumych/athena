//! 3D projective geometric algebra (PGA)

mod elements;
mod line;
mod motor;
mod plane;
mod point;

pub use self::{line::Line3, motor::Motor3, plane::Plane3, point::Point3};
