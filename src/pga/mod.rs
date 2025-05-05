//! Contains the geometric algebra.
//!
//!
//!

mod elements;
mod line;
mod motor;
mod plane;
mod point;
mod reflector;
mod rotor;
mod screw;

pub use self::{
    line::{Line2, Line3},
    motor::Motor2,
    plane::Plane3,
    point::{Point2, Point3},
};
