#![deny(clippy::inconsistent_struct_constructor)]

mod bivector;
mod pseudo;
mod scalar;
mod trivector;
mod vector;

pub use self::{bivector::*, trivector::*, vector::*};
pub(crate) use self::{pseudo::*, scalar::*};

pub(super) use crate::pga::{regressive, regressive3, Dual};
