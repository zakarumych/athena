//! Conversions between PGA geometric objects (see [`crate::pga`] types) and lina's
//! [`Vector`](crate::Vector)/[`Matrix`](crate::Matrix) types.

#[cfg(feature = "pga2")]
mod d2;

#[cfg(feature = "pga3")]
mod d3;
