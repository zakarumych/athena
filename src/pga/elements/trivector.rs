use crate::Scalar;

use super::{ BiVector3, EBiVector3, InnerProduct, OuterProduct, Pseudo3, Vector3, XBiVector3};


#[repr(C)]
pub(crate) struct TriVector3<T> {
    pub e021: T,
    pub e013: T,
    pub e032: T,
    pub e123: T,
}

impl<T> TriVector3<T> {
    pub const fn new(e021: T, e013: T, e032: T, e123: T) -> Self {
        Self {
            e021,
            e013,
            e032,
            e123,
        }
    }
}


impl<T> InnerProduct<T> for TriVector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn inner(&self, other: &T) -> TriVector3<T> {
        TriVector3::new(
            self.e021 * *other,
            self.e013 * *other,
            self.e032 * *other,
            self.e123 * *other,
        )
    }
}

impl<T> InnerProduct<Vector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = BiVector3<T>;
    fn inner(&self, other: &Vector3<T>) -> BiVector3<T> {
        BiVector3::new(
            self.e013 * other.e3 - self.e021 * other.e2,
            self.e021 * other.e1 - self.e032 * other.e3,
            self.e032 * other.e2 - self.e013 * other.e1,
            self.e123 * other.e3,
            self.e123 * other.e2,
            self.e123 * other.e1,
        )
    }
}

impl<T> InnerProduct<XBiVector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, _other: &XBiVector3<T>) -> Vector3<T> {
        Vector3::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
}

impl<T> InnerProduct<EBiVector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &EBiVector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            -(self.e123 * other.e23),
            -(self.e123 * other.e31),
            -(self.e123 * other.e12),
        )
    }
}

impl<T> InnerProduct<BiVector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;
    fn inner(&self, other: &BiVector3<T>) -> Vector3<T> {
        Vector3::new(
            self.e021 * other.e12 + self.e013 * other.e31 + self.e032 * other.e23,
            -(self.e123 * other.e23),
            -(self.e123 * other.e31),
            -(self.e123 * other.e12),
        )
    }
}

impl<T> InnerProduct<TriVector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = T;
    fn inner(&self, other: &TriVector3<T>) -> T {
        -(self.e123 * other.e123)
    }
}

impl<T> OuterProduct<T> for TriVector3<T>
where
    T: Scalar,
{
    type Output = TriVector3<T>;
    fn outer(&self, other: &T) -> TriVector3<T> {
        let other = *other;
        TriVector3::new(
            self.e021 * other,
            self.e013 * other,
            self.e032 * other,
            self.e123 * other,
        )
    }
}

impl<T> OuterProduct<Vector3<T>> for TriVector3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;
    fn outer(&self, other: &Vector3<T>) -> Pseudo3<T> {
        Pseudo3::new(
            -(self.e021 * other.e3 + self.e013 * other.e2 + self.e032 * other.e1 + self.e123 * other.e0),
        )
    }
}
