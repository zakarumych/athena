//! 2D projective geometric algebra (PGA)
//!

mod elements;
mod line;
mod motor;
mod point;

pub use self::{line::Line2, motor::Motor2, point::Point2};
