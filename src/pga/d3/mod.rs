//! 3D projective geometric algebra (PGA)

mod elements;
mod line;
mod line_variants;
mod motor;
mod plane;
mod plane_variants;
mod point;
mod point_variants;
mod rotor;
mod translator;

pub use self::{
    line::Line3,
    line_variants::{Axis3, Loop3, Radial3},
    motor::Motor3,
    plane::Plane3,
    plane_variants::{Face3, Horizon3},
    point::Point3,
    point_variants::{Direction3, Position3},
    rotor::Rotor3,
    translator::Translator3,
};
