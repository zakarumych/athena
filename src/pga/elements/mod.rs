use crate::Scalar;


mod bivector;
mod trivector;
mod vector;
mod pseudo;

pub(crate) use self::{vector::*, bivector::*, trivector::*, pseudo::*};

pub(crate) trait InnerProduct<Other> {
    type Output;
    fn inner(&self, other: &Other) -> Self::Output;
}

impl<T> InnerProduct<T> for T
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &T) -> T {
        *self * *other
    }
}

impl<T> InnerProduct<Vector2<T>> for T
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn inner(&self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::new(*self * other.e0, *self * other.e1, *self * other.e2)
    }
}

impl<T> InnerProduct<Vector3<T>> for T
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            *self * other.e0,
            *self * other.e1,
            *self * other.e2,
            *self * other.e3,
        )
    }
}

impl<T> InnerProduct<BiVector2<T>> for T
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn inner(&self, other: &BiVector2<T>) -> BiVector2<T> {
        BiVector2::new(*self * other.e12, *self * other.e01, *self * other.e02)
    }
}

impl<T> InnerProduct<XBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn inner(&self, other: &XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3::new(*self * other.e01, *self * other.e02, *self * other.e03)
    }
}

impl<T> InnerProduct<EBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = EBiVector3<T>;
    fn inner(&self, other: &EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3::new(*self * other.e12, *self * other.e31, *self * other.e23)
    }
}

impl<T> InnerProduct<BiVector3<T>> for T
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn inner(&self, other: &BiVector3<T>) -> BiVector3<T> {
        BiVector3::new(
            *self * other.e01,
            *self * other.e02,
            *self * other.e03,
            *self * other.e12,
            *self * other.e31,
            *self * other.e23,
        )
    }
}

impl<T> InnerProduct<TriVector3<T>> for T
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn inner(&self, other: &TriVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            *self * other.e021,
            *self * other.e013,
            *self * other.e032,
            *self * other.e123,
        )
    }
}

impl<T> InnerProduct<Pseudo2<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo2<T>;
    fn inner(&self, other: &Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2::new(*self * other.e021)
    }
}

impl<T> InnerProduct<Pseudo3<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn inner(&self, other: &Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3::new(*self * other.e0123)
    }
}

pub(crate) trait OuterProduct<Other> {
    type Output;
    fn outer(&self, other: &Other) -> Self::Output;
}

impl<T> OuterProduct<T> for T
where
    T: Scalar,
{
    type Output = T;
    fn outer(&self, other: &T) -> T {
        *self * *other
    }
}

impl<T> OuterProduct<Vector2<T>> for T
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn outer(&self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::new(*self * other.e0, *self * other.e1, *self * other.e2)
    }
}

impl<T> OuterProduct<Vector3<T>> for T
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn outer(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            *self * other.e0,
            *self * other.e1,
            *self * other.e2,
            *self * other.e3,
        )
    }
}

impl<T> OuterProduct<BiVector2<T>> for T
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn outer(&self, other: &BiVector2<T>) -> BiVector2<T> {
        BiVector2::new(*self * other.e12, *self * other.e01, *self * other.e02)
    }
}

impl<T> OuterProduct<XBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn outer(&self, other: &XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3::new(*self * other.e01, *self * other.e02, *self * other.e03)
    }
}

impl<T> OuterProduct<EBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = EBiVector3<T>;
    fn outer(&self, other: &EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3::new(*self * other.e12, *self * other.e31, *self * other.e23)
    }
}

impl<T> OuterProduct<BiVector3<T>> for T
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn outer(&self, other: &BiVector3<T>) -> BiVector3<T> {
        BiVector3::new(
            *self * other.e01,
            *self * other.e02,
            *self * other.e03,
            *self * other.e12,
            *self * other.e31,
            *self * other.e23,
        )
    }
}

impl<T> OuterProduct<TriVector3<T>> for T
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &TriVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            *self * other.e021,
            *self * other.e013,
            *self * other.e032,
            *self * other.e123,
        )
    }
}

impl<T> OuterProduct<Pseudo2<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo2<T>;
    fn outer(&self, other: &Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2::new(
            *self * other.e021,
        )
    }
}

impl<T> OuterProduct<Pseudo3<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3::new(
            *self * other.e0123,
        )
    }
}

pub(crate) trait GeometricProduct<Other> {
    type Output;
    fn geometric(&self, other: &Other) -> Self::Output;
}

impl<T> GeometricProduct<T> for T
where
    T: Scalar,
{
    type Output = T;
    fn geometric(&self, other: &T) -> T {
        *self * *other
    }
}

impl<T> GeometricProduct<Vector2<T>> for T
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn geometric(&self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::new(*self * other.e0, *self * other.e1, *self * other.e2)
    }
}

impl<T> GeometricProduct<Vector3<T>> for T
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn geometric(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            *self * other.e0,
            *self * other.e1,
            *self * other.e2,
            *self * other.e3,
        )
    }
}

impl<T> GeometricProduct<BiVector2<T>> for T
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn geometric(&self, other: &BiVector2<T>) -> BiVector2<T> {
        BiVector2::new(*self * other.e12, *self * other.e01, *self * other.e02)
    }
}

impl<T> GeometricProduct<XBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn geometric(&self, other: &XBiVector3<T>) -> XBiVector3<T> {
        XBiVector3::new(*self * other.e01, *self * other.e02, *self * other.e03)
    }
}

impl<T> GeometricProduct<EBiVector3<T>> for T
where
    T: Scalar,
{
    type Output = EBiVector3<T>;
    fn geometric(&self, other: &EBiVector3<T>) -> EBiVector3<T> {
        EBiVector3::new(*self * other.e12, *self * other.e31, *self * other.e23)
    }
}

impl<T> GeometricProduct<BiVector3<T>> for T
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn geometric(&self, other: &BiVector3<T>) -> BiVector3<T> {
        BiVector3::new(
            *self * other.e01,
            *self * other.e02,
            *self * other.e03,
            *self * other.e12,
            *self * other.e31,
            *self * other.e23,
        )
    }
}

impl<T> GeometricProduct<TriVector3<T>> for T
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn geometric(&self, other: &TriVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            *self * other.e021,
            *self * other.e013,
            *self * other.e032,
            *self * other.e123,
        )
    }
}

impl<T> GeometricProduct<Pseudo2<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo2<T>;
    fn geometric(&self, other: &Pseudo2<T>) -> Pseudo2<T> {
        Pseudo2::new(
            *self * other.e021,
        )
    }
}

impl<T> GeometricProduct<Pseudo3<T>> for T
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn geometric(&self, other: &Pseudo3<T>) -> Pseudo3<T> {
        Pseudo3::new(
            *self * other.e0123,
        )
    }
}
