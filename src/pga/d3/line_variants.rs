use crate::scalar::Num;

use super::{
    elements::{BiVector3, EBiVector3, XBiVector3},
    Line3,
};

/// A line's ideal (moment) bivector, guaranteed to pass through the origin.
///
/// Newtype over [`EBiVector3`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Radial3<T>(pub EBiVector3<T>);

impl<T> Radial3<T> {
    /// Creates a new through-origin line from its direction components.
    #[inline]
    pub const fn new(e12: T, e31: T, e23: T) -> Self {
        Radial3(EBiVector3::new(e12, e31, e23))
    }
}

/// A line guaranteed ideal (lies entirely at infinity) by construction.
///
/// Newtype over [`XBiVector3`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Loop3<T>(pub XBiVector3<T>);

impl<T> Loop3<T> {
    /// Creates a new ideal line from its moment components.
    #[inline]
    pub const fn new(e01: T, e02: T, e03: T) -> Self {
        Loop3(XBiVector3::new(e01, e02, e03))
    }
}

/// A line guaranteed non-ideal by construction.
///
/// Newtype over [`BiVector3`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Axis3<T>(pub BiVector3<T>);

impl<T> Axis3<T>
where
    T: Num,
{
    /// Creates a new axis from its moment and direction components.
    ///
    /// # Panics
    /// In debug builds, panics if the direction part is zero (the line would be ideal).
    #[inline]
    pub fn new(e01: T, e02: T, e03: T, e12: T, e31: T, e23: T) -> Self {
        debug_assert!(
            e12 != T::ZERO || e31 != T::ZERO || e23 != T::ZERO,
            "axis must not be ideal"
        );
        Axis3(BiVector3::new(e01, e02, e03, e12, e31, e23))
    }
}

impl<T> From<Axis3<T>> for Line3<T>
where
    T: Num,
{
    #[inline]
    fn from(a: Axis3<T>) -> Self {
        Line3::new(a.0.e01, a.0.e02, a.0.e03, a.0.e12, a.0.e31, a.0.e23)
    }
}

impl<T> TryFrom<Line3<T>> for Axis3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line3<T>) -> Result<Self, ()> {
        let bv = l.bivector();
        if bv.e12 == T::ZERO && bv.e31 == T::ZERO && bv.e23 == T::ZERO {
            return Err(());
        }
        Ok(Axis3(bv))
    }
}

impl<T> From<Loop3<T>> for Line3<T>
where
    T: Num,
{
    #[inline]
    fn from(l: Loop3<T>) -> Self {
        Line3::new(l.0.e01, l.0.e02, l.0.e03, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T> TryFrom<Line3<T>> for Loop3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line3<T>) -> Result<Self, ()> {
        let bv = l.bivector();
        if bv.e12 != T::ZERO || bv.e31 != T::ZERO || bv.e23 != T::ZERO {
            return Err(());
        }
        Ok(Loop3(XBiVector3::new(bv.e01, bv.e02, bv.e03)))
    }
}

impl<T> From<Radial3<T>> for Line3<T>
where
    T: Num,
{
    #[inline]
    fn from(r: Radial3<T>) -> Self {
        Line3::new(T::ZERO, T::ZERO, T::ZERO, r.0.e12, r.0.e31, r.0.e23)
    }
}

impl<T> From<Radial3<T>> for Axis3<T>
where
    T: Num,
{
    #[inline]
    fn from(r: Radial3<T>) -> Self {
        Axis3::new(T::ZERO, T::ZERO, T::ZERO, r.0.e12, r.0.e31, r.0.e23)
    }
}

impl<T> TryFrom<Line3<T>> for Radial3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(l: Line3<T>) -> Result<Self, ()> {
        let bv = l.bivector();
        if bv.e01 != T::ZERO || bv.e02 != T::ZERO || bv.e03 != T::ZERO {
            return Err(());
        }
        Ok(Radial3(EBiVector3::new(bv.e12, bv.e31, bv.e23)))
    }
}

impl<T> TryFrom<Axis3<T>> for Radial3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(a: Axis3<T>) -> Result<Self, ()> {
        if a.0.e01 != T::ZERO || a.0.e02 != T::ZERO || a.0.e03 != T::ZERO {
            return Err(());
        }
        Ok(Radial3(EBiVector3::new(a.0.e12, a.0.e31, a.0.e23)))
    }
}
