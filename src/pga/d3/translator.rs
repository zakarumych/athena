use core::ops::Mul;

use crate::scalar::Num;

use super::{
    elements::{EBiVector3, Pseudo3, Scalar3, XBiVector3},
    Line3, Motor3, Point3,
};

/// Translator is a 3D translation operator.
///
/// Unlike [`Motor3`], a translator never carries a rotation component.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translator3<T> {
    scalar: Scalar3<T>,
    bivector: XBiVector3<T>,
}

impl<T> Translator3<T> {
    /// Creates a new translator from the given scalar and bivector.
    #[inline]
    pub const fn new(scalar: Scalar3<T>, bivector: XBiVector3<T>) -> Self {
        Translator3 { scalar, bivector }
    }
}

impl<T> Translator3<T>
where
    T: Num,
{
    /// The identity translator.
    pub const IDENTITY: Self = Translator3 {
        scalar: Scalar3(T::ONE),
        bivector: XBiVector3::ZERO,
    };

    /// Returns the scalar part of this translator.
    #[inline]
    pub const fn scalar(&self) -> Scalar3<T> {
        self.scalar
    }

    /// Returns the bivector part of this translator.
    #[inline]
    pub const fn bivector(&self) -> XBiVector3<T> {
        self.bivector
    }

    /// Creates a new translator from the given offset.
    #[inline]
    pub fn from_offset(dx: T, dy: T, dz: T) -> Self {
        Translator3 {
            scalar: Scalar3(T::ONE),
            bivector: XBiVector3::new(-dx * T::HALF, -dy * T::HALF, -dz * T::HALF),
        }
    }

    /// Creates a new translator that moves `a` to `b`.
    ///
    /// Both points must be finite: an ideal point has no position to translate from/to.
    #[inline]
    pub fn from_point_to_point(a: Point3<T>, b: Point3<T>) -> Self {
        debug_assert!(!a.is_ideal(), "a must not be an ideal point");
        debug_assert!(!b.is_ideal(), "b must not be an ideal point");

        let (ax, ay, az) = a.coords();
        let (bx, by, bz) = b.coords();
        Self::from_offset(bx - ax, by - ay, bz - az)
    }

    /// Returns the offset of this translator.
    #[inline]
    pub fn offset(&self) -> (T, T, T) {
        (
            -self.bivector.e01 * T::TWO,
            -self.bivector.e02 * T::TWO,
            -self.bivector.e03 * T::TWO,
        )
    }

    /// Returns the distance of this translator.
    #[inline]
    pub fn distance(&self) -> T {
        let (dx, dy, dz) = self.offset();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Moves the given point by this translator.
    #[inline]
    pub fn move_point(&self, point: Point3<T>) -> Point3<T> {
        let p = point.trivector();
        let tv = self.scalar * p + self.bivector * p;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let a = tv * m_s_r;
        let b = tv * m_bv_r;

        Point3::from_trivector((a + b).normalized())
    }

    /// Moves the given line by this translator.
    #[inline]
    pub fn move_line(&self, line: Line3<T>) -> Line3<T> {
        let l = line.bivector();
        // `self.bivector * l` isn't directly defined (its ideal part always
        // contributes zero), so decompose `l` down to its Euclidean part first.
        let e = EBiVector3::new(l.e12, l.e31, l.e23);
        let (bv1, _zero0) = self.bivector * e;
        let bv = self.scalar * l + bv1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let a = bv * m_s_r;
        let (b, _zero1) = bv * m_bv_r;

        Line3::from_bivector((a + b).normalized())
    }
}

impl<T> Mul<Translator3<T>> for Translator3<T>
where
    T: Num,
{
    type Output = Translator3<T>;

    #[inline]
    fn mul(self, rhs: Translator3<T>) -> Translator3<T> {
        Translator3 {
            scalar: self.scalar * rhs.scalar,
            bivector: self.bivector + rhs.bivector,
        }
    }
}

impl<T> From<Translator3<T>> for Motor3<T>
where
    T: Num,
{
    #[inline]
    fn from(translator: Translator3<T>) -> Self {
        Motor3::new(translator.scalar, translator.bivector.into(), Pseudo3::ZERO)
    }
}
