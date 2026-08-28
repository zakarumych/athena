use crate::scalar::Num;

use super::{elements::XTriVector3, Point3};

/// A point guaranteed finite by construction.
///
/// Newtype over [`XTriVector3`]: reuses its arithmetic rather than duplicating its fields. The
/// point's weight (`e123`) is implicitly [`T::ONE`](Num::ONE). Axis correspondence: `e032` is x,
/// `e013` is y, `e021` is z (matches [`Point3::at`]'s layout).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position3<T>(pub XTriVector3<T>);

impl<T> Position3<T> {
    /// Creates a new position from its coordinates.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Position3(XTriVector3::new(z, y, x))
    }
}

impl<T> From<Position3<T>> for Point3<T>
where
    T: Num,
{
    #[inline]
    fn from(p: Position3<T>) -> Self {
        Point3::new(T::ONE, p.0.e032, p.0.e013, p.0.e021)
    }
}

impl<T> TryFrom<Point3<T>> for Position3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Point3<T>) -> Result<Self, ()> {
        if p.is_ideal() {
            return Err(());
        }
        let tv = p.normalized().trivector();
        Ok(Position3(XTriVector3::new(tv.e021, tv.e013, tv.e032)))
    }
}

/// A point guaranteed ideal (a direction) by construction.
///
/// Newtype over [`XTriVector3`]: reuses its arithmetic rather than duplicating its fields.
/// Axis correspondence: `e032` is x, `e013` is y, `e021` is z (matches [`Point3::ideal`]'s
/// layout).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Direction3<T>(pub XTriVector3<T>);

impl<T> Direction3<T> {
    /// Creates a new direction from its components.
    #[inline]
    pub const fn new(e021: T, e013: T, e032: T) -> Self {
        Direction3(XTriVector3::new(e021, e013, e032))
    }
}

impl<T> From<Direction3<T>> for Point3<T>
where
    T: Num,
{
    #[inline]
    fn from(d: Direction3<T>) -> Self {
        Point3::new(T::ZERO, d.0.e032, d.0.e013, d.0.e021)
    }
}

impl<T> TryFrom<Point3<T>> for Direction3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Point3<T>) -> Result<Self, ()> {
        if !p.is_ideal() {
            return Err(());
        }
        let tv = p.trivector();
        Ok(Direction3(XTriVector3::new(tv.e021, tv.e013, tv.e032)))
    }
}
