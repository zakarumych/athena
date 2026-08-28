#![deny(clippy::inconsistent_struct_constructor)]

mod bivector;
mod pseudo;
mod scalar;
mod trivector;
mod vector;

pub(crate) use self::{bivector::*, pseudo::*, scalar::*, trivector::*, vector::*};

pub(super) use crate::pga::{regressive, regressive3, Dual};
