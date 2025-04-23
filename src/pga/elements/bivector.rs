use crate::Scalar;

use super::{GeometricProduct, InnerProduct, OuterProduct, Pseudo2, Pseudo3, TriVector3, Vector2, Vector3};

#[repr(C)]
pub(crate) struct BiVector2<T> {
    pub e01: T,
    pub e02: T,
    pub e12: T,
}

impl<T> BiVector2<T> {
    pub const fn new(e01: T, e02: T, e12: T) -> Self {
        Self { e01, e02, e12 }
    }
}

#[repr(C)]
pub(crate) struct XBiVector3<T> {
    pub e01: T,
    pub e02: T,
    pub e03: T,
}

impl<T> XBiVector3<T> {
    pub const fn new(e01: T, e02: T, e03: T) -> Self {
        Self { e01, e02, e03 }
    }
}

#[repr(C)]
pub(crate) struct EBiVector3<T> {
    pub e12: T,
    pub e31: T,
    pub e23: T,
}

impl<T> EBiVector3<T> {
    pub const fn new(e12: T, e31: T, e23: T) -> Self {
        Self { e12, e31, e23 }
    }
}

#[repr(C)]
pub(crate) struct BiVector3<T> {
    pub e01: T,
    pub e02: T,
    pub e03: T,
    pub e12: T,
    pub e31: T,
    pub e23: T,
}

impl<T> BiVector3<T> {
    pub const fn new(e01: T, e02: T, e03: T, e12: T, e31: T, e23: T) -> Self {
        Self {
            e01,
            e02,
            e03,
            e12,
            e31,
            e23,
        }
    }
}

impl<T> InnerProduct<T> for BiVector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn inner(&self, other: &T) -> BiVector2<T> {
        BiVector2::new(self.e12 * *other, self.e01 * *other, self.e02 * *other)
    }
}

impl<T> InnerProduct<T> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn inner(&self, other: &T) -> XBiVector3<T> {
        XBiVector3::new(self.e01 * *other, self.e02 * *other, self.e03 * *other)
    }
}

impl<T> InnerProduct<T> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = EBiVector3<T>;
    fn inner(&self, other: &T) -> EBiVector3<T> {
        EBiVector3::new(self.e12 * *other, self.e31 * *other, self.e23 * *other)
    }
}

impl<T> InnerProduct<T> for BiVector3<T>
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn inner(&self, other: &T) -> BiVector3<T> {
        BiVector3::new(
            self.e01 * *other,
            self.e02 * *other,
            self.e03 * *other,
            self.e12 * *other,
            self.e31 * *other,
            self.e23 * *other,
        )
    }
}

impl<T> InnerProduct<Vector2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn inner(&self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::new(
            self.e01 * other.e1 + self.e02 * other.e2,
            self.e12 * other.e2,
            -(self.e12 * other.e1),
        )
    }
}

impl<T> InnerProduct<Vector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            T::ZERO,
            T::ZERO,
            T::ZERO,
        )
    }
}

impl<T> InnerProduct<Vector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            T::ZERO,
            self.e12 * other.e2 - self.e31 * other.e3,
            self.e23 * other.e3 - self.e12 * other.e1,
            self.e31 * other.e1 - self.e23 * other.e2,
        )
    }
}

impl<T> InnerProduct<Vector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            self.e12 * other.e2 - self.e31 * other.e3,
            self.e23 * other.e3 - self.e12 * other.e1,
            self.e31 * other.e1 - self.e23 * other.e2,
        )
    }
}

impl<T> InnerProduct<BiVector2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &BiVector2<T>) -> T {
        -(self.e12 * other.e12)
    }
}

impl<T> InnerProduct<XBiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, _other: &XBiVector3<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<XBiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, _other: &XBiVector3<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<XBiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, _other: &XBiVector3<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<EBiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, _other: &EBiVector3<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<EBiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &EBiVector3<T>) -> T {
        -(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23)
    }
}

impl<T> InnerProduct<EBiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &EBiVector3<T>) -> T {
        -(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23)
    }
}

impl<T> InnerProduct<BiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, _other: &BiVector3<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<BiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &BiVector3<T>) -> T {
        -(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23)
    }
}

impl<T> InnerProduct<BiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &BiVector3<T>) -> T {
        -(self.e12 * other.e12 + self.e31 * other.e31 + self.e23 * other.e23)
    }
}

impl<T> InnerProduct<TriVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, _other: &TriVector3<T>) -> Vector3<T> {
        Vector3::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T> InnerProduct<TriVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &TriVector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            -(self.e23 * other.e123),
            -(self.e31 * other.e123),
            -(self.e12 * other.e123),
        )
    }
}

impl<T> InnerProduct<TriVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &TriVector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e12 * other.e021 + self.e31 * other.e013 + self.e23 * other.e032,
            -(self.e23 * other.e123),
            -(self.e31 * other.e123),
            -(self.e12 * other.e123),
        )
    }
}

impl<T> InnerProduct<Pseudo2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = Vector2<T>;
    fn inner(&self, other: &Pseudo2<T>) -> Vector2<T> {
        Vector2::new(self.e12 * other.e021, T::ZERO, T::ZERO)
    }
}

impl<T> InnerProduct<Pseudo3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn inner(&self, _other: &Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3::new(T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T> InnerProduct<Pseudo3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn inner(&self, other: &Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3::new(
            -(self.e23 * other.e0123),
            -(self.e31 * other.e0123),
            -(self.e12 * other.e0123),
        )
    }
}

impl<T> InnerProduct<Pseudo3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = XBiVector3<T>;
    fn inner(&self, other: &Pseudo3<T>) -> XBiVector3<T> {
        XBiVector3::new(
            -(self.e23 * other.e0123),
            -(self.e31 * other.e0123),
            -(self.e12 * other.e0123),
        )
    }
}

impl<T> OuterProduct<T> for BiVector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn outer(&self, other: &T) -> BiVector2<T> {
        BiVector2::new(self.e01 * *other, self.e02 * *other, self.e12 * *other)
    }
}

impl<T> OuterProduct<Vector2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = Pseudo2<T>;
    fn outer(&self, other: &Vector2<T>) -> Pseudo2<T> {
        Pseudo2::new(self.e02 * other.e1 - self.e01 * other.e2)
    }
}

impl<T> OuterProduct<Vector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &Vector3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e02 * other.e1 - self.e01 * other.e2,
            self.e01 * other.e3 - self.e03 * other.e1,
            self.e03 * other.e2 - self.e02 * other.e3,
            T::ZERO,
        )
    }
}

impl<T> OuterProduct<Vector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &Vector3<T>) -> TriVector3<T> {
        TriVector3::new(
            -(self.e12 * other.e0),
            -(self.e31 * other.e0),
            -(self.e23 * other.e0),
            self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        )
    }
}

impl<T> OuterProduct<Vector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &Vector3<T>) -> TriVector3<T> {
        TriVector3::new(
            self.e02 * other.e1 - self.e01 * other.e2 - self.e12 * other.e0,
            self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            self.e03 * other.e2 - self.e02 * other.e3 - self.e23 * other.e0,
            self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        )
    }
}

impl<T> OuterProduct<XBiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &XBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(T::ZERO)
    }
}

impl<T> OuterProduct<EBiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &EBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e01 * other.e12 + self.e02 * other.e31 + self.e03 * other.e23)
    }
}

impl<T> OuterProduct<BiVector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &BiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e01 * other.e12 + self.e02 * other.e31 + self.e03 * other.e23)
    }
}

impl<T> OuterProduct<XBiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &XBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01)
    }
}

impl<T> OuterProduct<EBiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, _other: &EBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(T::ZERO)
    }
}

impl<T> OuterProduct<BiVector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &BiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01)
    }
}

impl<T> OuterProduct<XBiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &XBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e12 * other.e03 + self.e31 * other.e02 + self.e23 * other.e01)
    }
}

impl<T> OuterProduct<EBiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &EBiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(self.e01 * other.e12 + self.e02 * other.e31 + self.e03 * other.e23)
    }
}

impl<T> OuterProduct<BiVector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &BiVector3<T>) -> Pseudo3<T> {
        Pseudo3::new(
            self.e01 * other.e12
                + self.e02 * other.e31
                + self.e03 * other.e23
                + self.e12 * other.e03
                + self.e31 * other.e02
                + self.e23 * other.e01,
        )
    }
}



















impl<T> GeometricProduct<T> for BiVector2<T>
where
    T: Scalar,
{
    type Output = BiVector2<T>;
    fn geometric(&self, other: &T) -> BiVector2<T> {
        let other = *other;
        BiVector2::new(
            self.e01 * other,
            self.e02 * other,
            self.e12 * other,
        )
    }
}

impl<T> GeometricProduct<Vector2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = (Vector2<T>, Pseudo2<T>);
    fn geometric(&self, other: &Vector2<T>) -> (Vector2<T>, Pseudo2<T>) {
        let vec = Vector2::new(
            self.e01 * other.e1 + self.e02 * other.e2,
            self.e12 * other.e2,
            -(self.e12 * other.e1),
        );
        let pseudo = Pseudo2::new(self.e02 * other.e1 - self.e01 * other.e2);
        (vec, pseudo)
    }
}

impl<T> GeometricProduct<Vector3<T>> for XBiVector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            T::ZERO,
            T::ZERO,
            T::ZERO,
        );
        let triv = TriVector3::new(
            self.e02 * other.e1 - self.e01 * other.e2,
            self.e01 * other.e3 - self.e03 * other.e1,
            self.e03 * other.e2 - self.e02 * other.e3,
            T::ZERO,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<Vector3<T>> for EBiVector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            T::ZERO,
            self.e12 * other.e2 - self.e31 * other.e3,
            self.e23 * other.e3 - self.e12 * other.e1,
            self.e31 * other.e1 - self.e23 * other.e2,  
        );
        let triv = TriVector3::new(
            -(self.e12 * other.e0),
            -(self.e31 * other.e0),
            -(self.e23 * other.e0),
            self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<Vector3<T>> for BiVector3<T>
where
    T: Scalar,
{
    type Output = (Vector3<T>, TriVector3<T>);
    fn geometric(&self, other: &Vector3<T>) -> (Vector3<T>, TriVector3<T>) {
        let vec = Vector3::new(
            self.e01 * other.e1 + self.e02 * other.e2 + self.e03 * other.e3,
            self.e12 * other.e2 - self.e31 * other.e3,
            self.e23 * other.e3 - self.e12 * other.e1,
            self.e31 * other.e1 - self.e23 * other.e2,  
        );
        let triv = TriVector3::new(
            self.e02 * other.e1 - self.e01 * other.e2 - self.e12 * other.e0,
            self.e01 * other.e3 - self.e03 * other.e1 - self.e31 * other.e0,
            self.e03 * other.e2 - self.e02 * other.e3 - self.e23 * other.e0,
            self.e12 * other.e3 + self.e31 * other.e2 + self.e23 * other.e1,
        );
        (vec, triv)
    }
}

impl<T> GeometricProduct<BiVector2<T>> for BiVector2<T>
where
    T: Scalar,
{
    type Output = (BiVector2<T>, Pseudo2<T>);
    fn geometric(&self, other: &BiVector2<T>) -> (BiVector2<T>, Pseudo2<T>) {
        let bivec = BiVector2::new(
            
        );
        let pseudo = Pseudo2::new(self.e1 * other.e02 - self.e2 * other.e01);
        (vec, pseudo)
    }
}

impl<T> GeometricProduct<XBiVector3<T>> for BiVector3<T>
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

impl<T> GeometricProduct<EBiVector3<T>> for BiVector3<T>
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

impl<T> GeometricProduct<BiVector3<T>> for BiVector3<T>
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

impl<T> GeometricProduct<TriVector3<T>> for BiVector3<T>
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

impl<T> GeometricProduct<Pseudo2<T>> for BiVector2<T>
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

impl<T> GeometricProduct<Pseudo3<T>> for BiVector3<T>
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
