#![deny(clippy::inconsistent_struct_constructor)]

mod bivector;
mod pseudo;
mod scalar;
mod vector;

pub use self::{bivector::*, vector::*};
pub(crate) use self::{pseudo::*, scalar::*};

pub(super) use crate::pga::{regressive, Dual};
