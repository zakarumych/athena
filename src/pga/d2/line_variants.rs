use crate::scalar::Num;

use super::{
    elements::{EVector2, Vector2, XVector2},
    Line2,
};

/// A line guaranteed non-ideal by construction.
///
/// Newtype over [`Vector2`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Axis2<T>(pub Vector2<T>);

impl<T> Axis2<T>
where
    T: Num,
{
    /// Creates a new axis from its components.
    ///
    /// # Panics
    /// In debug builds, panics if `e1` and `e2` are both zero (the line would be ideal).
    #[inline]
    pub fn new(e0: T, e1: T, e2: T) -> Self {
        debug_assert!(e1 != T::ZERO || e2 != T::ZERO, "axis must not be ideal");
        Axis2(Vector2::new(e0, e1, e2))
    }
}

impl<T> From<Axis2<T>> for Line2<T>
where
    T: Num,
{
    #[inline]
    fn from(a: Axis2<T>) -> Self {
        Line2::new(a.0.e0, a.0.e1, a.0.e2)
    }
}

impl<T> TryFrom<Line2<T>> for Axis2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line2<T>) -> Result<Self, ()> {
        if l.is_ideal() {
            return Err(());
        }
        let (e1, e2, e0) = l.abc();
        Ok(Axis2(Vector2::new(e0, e1, e2)))
    }
}

/// A line guaranteed ideal (the line at infinity) by construction.
///
/// Newtype over [`XVector2`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Loop2<T>(pub XVector2<T>);

impl<T> Loop2<T> {
    /// Creates a new ideal line from its offset component.
    #[inline]
    pub const fn new(e0: T) -> Self {
        Loop2(XVector2::new(e0))
    }
}

impl<T> From<Loop2<T>> for Line2<T>
where
    T: Num,
{
    #[inline]
    fn from(l: Loop2<T>) -> Self {
        Line2::new(l.0.e0, T::ZERO, T::ZERO)
    }
}

impl<T> TryFrom<Line2<T>> for Loop2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line2<T>) -> Result<Self, ()> {
        if !l.is_ideal() {
            return Err(());
        }
        let (_, _, e0) = l.abc();
        Ok(Loop2(XVector2::new(e0)))
    }
}

/// A line guaranteed to pass through the origin by construction.
///
/// Newtype over [`EVector2`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Radial2<T>(pub EVector2<T>);

impl<T> Radial2<T> {
    /// Creates a new through-origin line from its direction components.
    #[inline]
    pub const fn new(e1: T, e2: T) -> Self {
        Radial2(EVector2::new(e1, e2))
    }
}

impl<T> From<Radial2<T>> for Line2<T>
where
    T: Num,
{
    #[inline]
    fn from(r: Radial2<T>) -> Self {
        Line2::new(T::ZERO, r.0.e1, r.0.e2)
    }
}

impl<T> From<Radial2<T>> for Axis2<T>
where
    T: Num,
{
    #[inline]
    fn from(r: Radial2<T>) -> Self {
        Axis2::new(T::ZERO, r.0.e1, r.0.e2)
    }
}

impl<T> TryFrom<Line2<T>> for Radial2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line2<T>) -> Result<Self, ()> {
        let (e1, e2, e0) = l.abc();
        if e0 != T::ZERO {
            return Err(());
        }
        Ok(Radial2(EVector2::new(e1, e2)))
    }
}

impl<T> TryFrom<Axis2<T>> for Radial2<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(a: Axis2<T>) -> Result<Self, ()> {
        if a.0.e0 != T::ZERO {
            return Err(());
        }
        Ok(Radial2(EVector2::new(a.0.e1, a.0.e2)))
    }
}
