#![deny(clippy::inconsistent_struct_constructor)]

mod bivector;
mod pseudo;
mod scalar;
mod vector;

pub(crate) use self::{bivector::*, pseudo::*, scalar::*, vector::*};

pub(super) use crate::pga::{regressive, Dual};
