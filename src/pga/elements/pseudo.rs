use crate::Scalar;

use super::InnerProduct;


#[repr(transparent)]
pub(crate) struct Pseudo2<T> {
    pub e021: T,
}

impl<T> Pseudo2<T> {
    pub const fn new(e021: T) -> Self {
        Self { e021 }
    }
}

#[repr(transparent)]
pub(crate) struct Pseudo3<T> {
    pub e0123: T,
}

impl<T> Pseudo3<T> {
    pub const fn new(e0123: T) -> Self {
        Self { e0123 }
    }
}

impl<T> InnerProduct<T> for Pseudo2<T>
where
    T: Scalar,
{
    type Output = Pseudo2<T>;

    fn inner(&self, other: &T) -> Pseudo2<T> {
        Pseudo2::new(self.e021 * *other)
    }
}

impl<T> InnerProduct<T> for Pseudo3<T>
where
    T: Scalar,
{
    type Output = Pseudo3<T>;

    fn inner(&self, other: &T) -> Pseudo3<T> {
        Pseudo3::new(self.e0123 * *other)
    }
}

impl<T> InnerProduct<Pseudo2<T>> for Pseudo2<T>
where
    T: Scalar,
{
    type Output = T;

    fn inner(&self, _other: &Pseudo2<T>) -> T {
        T::ZERO
    }
}

impl<T> InnerProduct<Pseudo3<T>> for Pseudo3<T>
where
    T: Scalar,
{
    type Output = T;

    fn inner(&self, _other: &Pseudo3<T>) -> T {
        T::ZERO
    }
}
