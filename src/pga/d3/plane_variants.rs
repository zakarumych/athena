use crate::scalar::Num;

use super::{
    elements::{Vector3, XVector3},
    Plane3,
};

/// A plane guaranteed non-ideal by construction.
///
/// Newtype over [`Vector3`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Face3<T>(pub Vector3<T>);

impl<T> Face3<T>
where
    T: Num,
{
    /// Creates a new face from its components.
    ///
    /// # Panics
    /// In debug builds, panics if `e1`, `e2`, `e3` are all zero (the plane would be ideal).
    #[inline]
    pub fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        debug_assert!(
            e1 != T::ZERO || e2 != T::ZERO || e3 != T::ZERO,
            "face must not be ideal"
        );
        Face3(Vector3::new(e0, e1, e2, e3))
    }
}

impl<T> From<Face3<T>> for Plane3<T>
where
    T: Num,
{
    #[inline]
    fn from(f: Face3<T>) -> Self {
        Plane3::new(f.0.e0, f.0.e1, f.0.e2, f.0.e3)
    }
}

impl<T> TryFrom<Plane3<T>> for Face3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Plane3<T>) -> Result<Self, ()> {
        if p.is_ideal() {
            return Err(());
        }
        let (e1, e2, e3, e0) = p.abcd();
        Ok(Face3(Vector3::new(e0, e1, e2, e3)))
    }
}

/// A plane guaranteed ideal (the plane at infinity) by construction.
///
/// Newtype over [`XVector3`]: reuses its arithmetic rather than duplicating its fields.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Horizon3<T>(pub XVector3<T>);

impl<T> Horizon3<T> {
    /// Creates a new ideal plane from its offset component.
    #[inline]
    pub const fn new(e0: T) -> Self {
        Horizon3(XVector3::new(e0))
    }
}

impl<T> From<Horizon3<T>> for Plane3<T>
where
    T: Num,
{
    #[inline]
    fn from(h: Horizon3<T>) -> Self {
        Plane3::new(h.0.e0, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T> TryFrom<Plane3<T>> for Horizon3<T>
where
    T: Num,
{
    type Error = ();

    #[inline]
    fn try_from(p: Plane3<T>) -> Result<Self, ()> {
        if !p.is_ideal() {
            return Err(());
        }
        let (_, _, _, e0) = p.abcd();
        Ok(Horizon3(XVector3::new(e0)))
    }
}
