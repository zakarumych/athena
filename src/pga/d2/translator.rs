use core::ops::Mul;

use crate::scalar::Num;

use super::{
    elements::{EBiVector2, Scalar2, XBiVector2},
    Line2, Motor2, Point2,
};

/// Translator is a 2D translation operator.
///
/// Unlike [`Motor2`], a translator never carries a rotation component.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translator2<T> {
    scalar: Scalar2<T>,
    bivector: XBiVector2<T>,
}

impl<T> Translator2<T> {
    /// Creates a new translator from the given scalar and bivector.
    #[inline]
    pub const fn new(scalar: Scalar2<T>, bivector: XBiVector2<T>) -> Self {
        Translator2 { scalar, bivector }
    }
}

impl<T> Translator2<T>
where
    T: Num,
{
    /// The identity translator.
    pub const IDENTITY: Self = Translator2 {
        scalar: Scalar2(T::ONE),
        bivector: XBiVector2::ZERO,
    };

    /// Returns the scalar part of this translator.
    #[inline]
    pub const fn scalar(&self) -> Scalar2<T> {
        self.scalar
    }

    /// Returns the bivector part of this translator.
    #[inline]
    pub const fn bivector(&self) -> XBiVector2<T> {
        self.bivector
    }

    /// Creates a new translator from the given offset.
    #[inline]
    pub fn from_offset(dx: T, dy: T) -> Self {
        Translator2 {
            scalar: Scalar2(T::ONE),
            bivector: XBiVector2::new(-dx * T::HALF, dy * T::HALF),
        }
    }

    /// Creates a new translator that moves `a` to `b`.
    ///
    /// Both points must be finite: an ideal point has no position to translate from/to.
    #[inline]
    pub fn from_point_to_point(a: Point2<T>, b: Point2<T>) -> Self {
        debug_assert!(!a.is_ideal(), "a must not be an ideal point");
        debug_assert!(!b.is_ideal(), "b must not be an ideal point");

        let (ax, ay) = a.coords();
        let (bx, by) = b.coords();
        Self::from_offset(bx - ax, by - ay)
    }

    /// Returns the offset of this translator.
    #[inline]
    pub fn offset(&self) -> (T, T) {
        (-self.bivector.e01 * T::TWO, self.bivector.e20 * T::TWO)
    }

    /// Returns the distance of this translator.
    #[inline]
    pub fn distance(&self) -> T {
        let (dx, dy) = self.offset();
        (dx * dx + dy * dy).sqrt()
    }

    /// Moves the given point by this translator.
    #[inline]
    pub fn move_point(&self, point: Point2<T>) -> Point2<T> {
        let p = point.bivector();
        // `self.bivector * p` isn't directly defined (its ideal part always
        // contributes zero), so decompose `p` down to its Euclidean part first.
        let e = EBiVector2::new(p.e12);
        let bv = self.scalar * p + self.bivector * e;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let a = bv * m_s_r;
        let b = bv * m_bv_r;

        Point2::from_bivector((a + b).normalized())
    }

    /// Moves the given line by this translator.
    #[inline]
    pub fn move_line(&self, line: Line2<T>) -> Line2<T> {
        let v0 = self.scalar * line.vector();
        let (v1, _zero1) = self.bivector * line.vector();
        let v = v0 + v1;

        let m_s_r = !self.scalar;
        let m_bv_r = !self.bivector;

        let a = v * m_s_r;
        let (b, _zero2) = v * m_bv_r;

        Line2::from_vector((a + b).normalized())
    }
}

impl<T> Mul<Translator2<T>> for Translator2<T>
where
    T: Num,
{
    type Output = Translator2<T>;

    #[inline]
    fn mul(self, rhs: Translator2<T>) -> Translator2<T> {
        Translator2 {
            scalar: self.scalar * rhs.scalar,
            bivector: self.bivector + rhs.bivector,
        }
    }
}

impl<T> From<Translator2<T>> for Motor2<T>
where
    T: Num,
{
    #[inline]
    fn from(translator: Translator2<T>) -> Self {
        Motor2::new(translator.scalar, translator.bivector.into())
    }
}
