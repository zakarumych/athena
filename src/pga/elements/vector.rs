use crate::Scalar;

use super::{
    pseudo, BiVector2, BiVector3, EBiVector3, GeometricProduct, InnerProduct, OuterProduct, Pseudo2, Pseudo3, TriVector3, XBiVector3
};

#[repr(C)]
pub(crate) struct Vector2<T> {
    pub e0: T,
    pub e1: T,
    pub e2: T,
}

impl<T> Vector2<T> {
    pub const fn new(e0: T, e1: T, e2: T) -> Self {
        Self { e0, e1, e2 }
    }
}

#[repr(C)]
pub(crate) struct Vector3<T> {
    pub e0: T,
    pub e1: T,
    pub e2: T,
    pub e3: T,
}

impl<T> Vector3<T> {
    pub const fn new(e0: T, e1: T, e2: T, e3: T) -> Self {
        Self { e0, e1, e2, e3 }
    }
}

impl<T> InnerProduct<T> for Vector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn inner(&self, other: &T) -> Vector2<T> {
        Vector2::new(self.e0 * *other, self.e1 * *other, self.e2 * *other)
    }
}

impl<T> InnerProduct<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &T) -> Vector3<T> {
        Vector3::new(
            self.e0 * *other,
            self.e1 * *other,
            self.e2 * *other,
            self.e3 * *other,
        )
    }
}

impl<T> InnerProduct<Vector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &Vector2<T>) -> T {
        self.e1 * other.e1 - self.e2 * other.e2
    }
}

impl<T> InnerProduct<Vector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &Vector3<T>) -> T {
        self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3
    }
}

impl<T> InnerProduct<BiVector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn inner(&self, other: &BiVector2<T>) -> Vector2<T> {
        Vector2::new(
            -(self.e1 * other.e01 + self.e2 * other.e02),
            -(self.e2 * other.e12),
            self.e1 * other.e12,
        )
    }
}

impl<T> InnerProduct<XBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &XBiVector3<T>) -> Vector3<T> {
        Vector3::new(
            -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            T::ZERO,
            T::ZERO,
            T::ZERO,
        )
    }
}

impl<T> InnerProduct<EBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &EBiVector3<T>) -> Vector3<T> {
        Vector3::new(
            T::ZERO,
            self.e3 * other.e31 - self.e2 * other.e12,
            self.e1 * other.e12 - self.e3 * other.e23,
            self.e2 * other.e23 - self.e1 * other.e31,
        )
    }
}

impl<T> InnerProduct<BiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &BiVector3<T>) -> Vector3<T> {
        Vector3::new(
            -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            self.e3 * other.e31 - self.e2 * other.e12,
            self.e1 * other.e12 - self.e3 * other.e23,
            self.e2 * other.e23 - self.e1 * other.e31,
        )
    }
}

impl<T> InnerProduct<TriVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn inner(&self, other: &TriVector3<T>) -> BiVector3<T> {
        BiVector3::new(
            self.e3 * other.e013 - self.e2 * other.e021,
            self.e1 * other.e021 - self.e3 * other.e032,
            self.e2 * other.e032 - self.e1 * other.e013,
            self.e1 * other.e123,
            self.e2 * other.e123,
            self.e3 * other.e123,
        )
    }
}

impl<T> InnerProduct<Pseudo2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn inner(&self, other: &Pseudo2<T>) -> BiVector2<T> {
        BiVector2::new(-(self.e2 * other.e021), self.e1 * other.e021, T::ZERO)
    }
}

impl<T> InnerProduct<Pseudo3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn inner(&self, other: &Pseudo3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e3 * other.e0123,
            self.e2 * other.e0123,
            self.e1 * other.e0123,
            T::ZERO,
        )
    }
}

impl<T> OuterProduct<T> for Vector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn outer(&self, other: &T) -> Vector2<T> {
        let other = *other;
        Vector2::new(self.e0 * other, self.e1 * other, self.e2 * other)
    }
}

impl<T> OuterProduct<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn outer(&self, other: &T) -> Vector3<T> {
        let other = *other;
        Vector3::new(
            self.e0 * other,
            self.e1 * other,
            self.e2 * other,
            self.e3 * other,
        )
    }
}

impl<T> OuterProduct<Vector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn outer(&self, other: &Vector2<T>) -> BiVector2<T> {
        BiVector2::new(
            self.e0 * other.e1 - self.e1 * other.e0,
            self.e0 * other.e2 - self.e2 * other.e0,
            self.e1 * other.e2 - self.e2 * other.e1,
        )
    }
}

impl<T> OuterProduct<Vector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn outer(&self, other: &Vector3<T>) -> BiVector3<T> {
        BiVector3::new(
            self.e0 * other.e1 - self.e1 * other.e0,
            self.e0 * other.e2 - self.e2 * other.e0,
            self.e0 * other.e3 - self.e3 * other.e0,
            self.e1 * other.e2 - self.e2 * other.e1,
            self.e3 * other.e1 - self.e1 * other.e3,
            self.e2 * other.e3 - self.e3 * other.e2,
        )
    }
}

impl<T> OuterProduct<BiVector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = Pseudo2<T>;
    fn outer(&self, other: &BiVector2<T>) -> Pseudo2<T> {
        Pseudo2::new(self.e1 * other.e02 - self.e2 * other.e01)
    }
}

impl<T> OuterProduct<XBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &XBiVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e1 * other.e02 - self.e2 * other.e01,
            self.e3 * other.e01 - self.e1 * other.e03,
            self.e2 * other.e03 - self.e3 * other.e02,
            T::ZERO,
        )
    }
}

impl<T> OuterProduct<EBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &EBiVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            -(self.e0 * other.e12),
            -(self.e0 * other.e31),
            -(self.e0 * other.e23),
            self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        )
    }
}

impl<T> OuterProduct<BiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &BiVector3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e1 * other.e02 - self.e2 * other.e01 - self.e0 * other.e12,
            self.e3 * other.e01 - self.e1 * other.e03 - self.e0 * other.e31,
            self.e2 * other.e03 - self.e3 * other.e02 - self.e0 * other.e23,
            self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        )
    }
}

impl<T> OuterProduct<TriVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &TriVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(
            self.e0 * other.e123
                - self.e1 * other.e032
                - self.e2 * other.e013
                - self.e3 * other.e021,
        )
    }
}

impl<T> GeometricProduct<T> for Vector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn geometric(&self, other: &T) -> Vector2<T> {
        let other = *other;
        Vector2::new(self.e0 * other, self.e1 * other, self.e2 * other)
    }
}

impl<T> GeometricProduct<Vector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = (T, BiVector2<T>);
    fn geometric(&self, other: &Vector2<T>) -> (T, BiVector2<T>) {
        let scalar = self.e1 * other.e1 + self.e2 * other.e2;

        let bivec = BiVector2::new(
            self.e0 * other.e1 - self.e1 * other.e0,
            self.e0 * other.e2 - self.e2 * other.e0,
            self.e1 * other.e2 - self.e2 * other.e1,
        );

        (scalar, bivec)
    }
}

impl<T> GeometricProduct<Vector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = (T, BiVector3<T>);
    fn geometric(&self, other: &Vector3<T>) -> (T, BiVector3<T>) {
        let scalar = self.e1 * other.e1 + self.e2 * other.e2 + self.e3 * other.e3;

        let bivec = BiVector3::new(
            self.e0 * other.e1 - self.e1 * other.e0,
            self.e0 * other.e2 - self.e2 * other.e0,
            self.e0 * other.e3 - self.e3 * other.e0,
            self.e1 * other.e2 - self.e2 * other.e1,
            self.e3 * other.e1 - self.e1 * other.e3,
            self.e2 * other.e3 - self.e3 * other.e2,
        );

        (scalar, bivec)
    }
}

impl<T> GeometricProduct<BiVector2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = (Vector2<T>, Pseudo2<T>);
    fn geometric(&self, other: &BiVector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        let vec = Vector2::new(
            -(self.e1 * other.e01 + self.e2 * other.e02),
            -(self.e2 * other.e12),
            self.e1 * other.e12,
        );
        let pseudo = Pseudo2::new(self.e1 * other.e02 - self.e2 * other.e01);
        (vec, pseudo)
    }
}

impl<T> GeometricProduct<XBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &XBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            T::ZERO,
            T::ZERO,
            T::ZERO,
        );
        let triv = TriVector3::new(
            self.e1 * other.e02 - self.e2 * other.e01,
            self.e3 * other.e01 - self.e1 * other.e03,
            self.e2 * other.e03 - self.e3 * other.e02,
            T::ZERO,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<EBiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &EBiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            T::ZERO,
            self.e3 * other.e31 - self.e2 * other.e12,
            self.e1 * other.e12 - self.e3 * other.e23,
            self.e2 * other.e23 - self.e1 * other.e31,
        );
        let triv = TriVector3::new(
            -(self.e0 * other.e12),
            -(self.e0 * other.e31),
            -(self.e0 * other.e23),
            self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<BiVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &BiVector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            -(self.e1 * other.e01 + self.e2 * other.e02 + self.e3 * other.e03),
            self.e3 * other.e31 - self.e2 * other.e12,
            self.e1 * other.e12 - self.e3 * other.e23,
            self.e2 * other.e23 - self.e1 * other.e31,
        );
        let triv = TriVector3::new(
            self.e1 * other.e02 - self.e2 * other.e01 - self.e0 * other.e12,
            self.e3 * other.e01 - self.e1 * other.e03 - self.e0 * other.e31,
            self.e2 * other.e03 - self.e3 * other.e02 - self.e0 * other.e23,
            self.e1 * other.e23 + self.e2 * other.e31 + self.e3 * other.e12,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<TriVector3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = (BiVector3<T>, Pseudo3<T>);
    fn geometric(&self, other: &TriVector3<T>) -> (BiVector3<T>, Pseudo3<T>) {
        let bivec = BiVector3::new(
            self.e3 * other.e013 - self.e2 * other.e021,
            self.e1 * other.e021 - self.e3 * other.e032,
            self.e2 * other.e032 - self.e1 * other.e013,
            self.e3 * other.e123,
            self.e2 * other.e123,
            self.e1 * other.e123,
        );

        let pseudo = Pseudo3::new(
            self.e0 * other.e123
                + self.e1 * other.e032
                + self.e2 * other.e013
                + self.e3 * other.e021,
        );

        (bivec, pseudo)
    }
}

impl<T> GeometricProduct<Pseudo2<T>> for Vector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn geometric(&self, other: &Pseudo2<T>) -> BiVector2<T> {
        BiVector2::new(
            -(self.e2 * other.e021),
            self.e1 * other.e021,
            T::ZERO,
        )
    }
}

impl<T> GeometricProduct<Pseudo3<T>> for Vector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn geometric(&self, other: &Pseudo3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e3 * other.e0123,
            self.e2 * other.e0123,
            self.e1 * other.e0123,
            T::ZERO,
        )
    }
}
