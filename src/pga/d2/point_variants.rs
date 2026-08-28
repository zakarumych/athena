use crate::scalar::Num;

use super::{elements::XBiVector2, Point2};

/// A point guaranteed finite by construction.
///
/// Newtype over [`XBiVector2`]: reuses its arithmetic rather than duplicating its fields. The
/// point's weight (`e12`) is implicitly [`T::ONE`](Num::ONE). Axis correspondence: `e01` is the
/// y component, `e20` is the x component (matches [`Point2::at`]'s layout).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position2<T>(pub XBiVector2<T>);

impl<T> Position2<T> {
    /// Creates a new position from its coordinates.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Position2(XBiVector2::new(y, x))
    }
}

impl<T> From<Position2<T>> for Point2<T>
where
    T: Num,
{
    #[inline]
    fn from(p: Position2<T>) -> Self {
        Point2::new(p.0.e01, p.0.e20, T::ONE)
    }
}

impl<T> TryFrom<Point2<T>> for Position2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Point2<T>) -> Result<Self, ()> {
        if p.is_ideal() {
            return Err(());
        }
        let bv = p.normalized().bivector();
        Ok(Position2(XBiVector2::new(bv.e01, bv.e20)))
    }
}

/// A point guaranteed ideal (a direction) by construction.
///
/// Newtype over [`XBiVector2`]: reuses its arithmetic rather than duplicating its fields.
/// Axis correspondence: `e01` is the y component, `e20` is the x component (matches
/// [`Point2::ideal`]'s layout).
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Direction2<T>(pub XBiVector2<T>);

impl<T> Direction2<T> {
    /// Creates a new direction from its components.
    #[inline]
    pub const fn new(e01: T, e20: T) -> Self {
        Direction2(XBiVector2::new(e01, e20))
    }
}

impl<T> From<Direction2<T>> for Point2<T>
where
    T: Num,
{
    #[inline]
    fn from(d: Direction2<T>) -> Self {
        Point2::new(d.0.e01, d.0.e20, T::ZERO)
    }
}

impl<T> TryFrom<Point2<T>> for Direction2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Point2<T>) -> Result<Self, ()> {
        if !p.is_ideal() {
            return Err(());
        }
        let bv = p.bivector();
        Ok(Direction2(XBiVector2::new(bv.e01, bv.e20)))
    }
}
