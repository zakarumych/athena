//! Contains N-dimensional vector type and operations for it.
//!
//! Some methods are implemented for specific dimensions (1D, 2D, 3D, and 4D) to provide convenient access to the elements of the vector.
//!

use core::{
    mem::{align_of, offset_of, size_of},
    ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

use crate::Num;

pub use self::elements::{X, XY, XYZ, XYZW};

mod elements;

/// A vector in N-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Vector<T, const N: usize> {
    e: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    /// Create a new vector from an array of elements.
    #[inline(always)]
    pub const fn from_array(e: [T; N]) -> Self {
        Vector { e }
    }

    /// Interpret array reference as a vector reference.
    #[inline(always)]
    pub const fn from_array_ref(e: &[T; N]) -> &Self {
        #![allow(unsafe_code)]

        unsafe {
            // This is safe because the memory layout of the vector and the array are identical.
            let ptr = e as *const [T; N] as *const Self;
            &*ptr
        }
    }

    /// Interpret mutable array reference as a vector reference.
    #[inline(always)]
    pub const fn from_array_mut(e: &mut [T; N]) -> &mut Self {
        #![allow(unsafe_code)]

        unsafe {
            // This is safe because the memory layout of the vector and the array are identical.
            let ptr = e as *mut [T; N] as *mut Self;
            &mut *ptr
        }
    }

    /// Extracts the elements of the vector as an array.
    #[inline(always)]
    pub const fn array(&self) -> &[T; N] {
        &self.e
    }

    /// Extracts the elements of the vector as an array.
    #[inline(always)]
    pub const fn array_mut(&mut self) -> &mut [T; N] {
        &mut self.e
    }

    /// Extracts the elements of the vector as an array.
    #[inline(always)]
    pub fn into_array(self) -> [T; N] {
        self.e
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num,
{
    /// Create a new vector with all elements set to zero.
    pub const ZERO: Self = Vector { e: [T::ZERO; N] };

    /// Returns the dot product of this vector and `rhs`.
    #[inline]
    pub fn dot(&self, rhs: &Self) -> T {
        let mut result = T::ZERO;

        for n in 0..N {
            result += self.e[n] * rhs.e[n];
        }

        result
    }
}

impl<T> Vector<T, 3>
where
    T: Num,
{
    /// Returns the cross product of this vector and `rhs`.
    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vector::from_array([
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        ])
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline]
    fn add(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut result = Vector::ZERO;

        for n in 0..N {
            result.e[n] = self.e[n] + rhs.e[n];
        }

        result
    }
}

impl<T, const N: usize> Add<&Vector<T, N>> for &Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn add(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        *self + *rhs
    }
}

impl<T, const N: usize> Add<&Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn add(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        self + *rhs
    }
}

impl<T, const N: usize> Add<Vector<T, N>> for &Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn add(self, rhs: Vector<T, N>) -> Vector<T, N> {
        *self + rhs
    }
}

impl<T, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    #[inline]
    fn add_assign(&mut self, rhs: Vector<T, N>) {
        for n in 0..N {
            self.e[n] += rhs.e[n];
        }
    }
}

impl<T, const N: usize> AddAssign<&Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Vector<T, N>) {
        *self += *rhs;
    }
}

impl<T, const N: usize> Sub<Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline]
    fn sub(self, rhs: Vector<T, N>) -> Vector<T, N> {
        let mut result = Vector::ZERO;

        for n in 0..N {
            result.e[n] = self.e[n] - rhs.e[n];
        }

        result
    }
}

impl<T, const N: usize> Sub<&Vector<T, N>> for &Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn sub(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        *self - *rhs
    }
}

impl<T, const N: usize> Sub<&Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn sub(self, rhs: &Vector<T, N>) -> Vector<T, N> {
        self - *rhs
    }
}

impl<T, const N: usize> Sub<Vector<T, N>> for &Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn sub(self, rhs: Vector<T, N>) -> Vector<T, N> {
        *self - rhs
    }
}

impl<T, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Vector<T, N>) {
        for n in 0..N {
            self.e[n] -= rhs.e[n];
        }
    }
}

impl<T, const N: usize> SubAssign<&Vector<T, N>> for Vector<T, N>
where
    T: Num,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Vector<T, N>) {
        *self -= *rhs;
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline]
    fn mul(self, rhs: T) -> Vector<T, N> {
        let mut result = Vector::ZERO;

        for n in 0..N {
            result.e[n] = self.e[n] * rhs;
        }

        result
    }
}

impl<T, const N: usize> Mul<T> for &Vector<T, N>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: T) -> Vector<T, N> {
        *self * rhs
    }
}

impl<T, const N: usize> MulAssign<T> for Vector<T, N>
where
    T: Num,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        for n in 0..N {
            self.e[n] *= rhs;
        }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &T {
        assert!(index < N, "Index out of bounds: {}", index);
        &self.e[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < N, "Index out of bounds: {}", index);
        &mut self.e[index]
    }
}

// Helper macro to implement methods for specific dimensions.
macro_rules! impl_for_n {
    // Literal dimensions number and identifiers for each dimension.
    ($ty:ident $n:literal $alias:ident $elements:ident [$($r:ident)*] $(where $($clause:tt)+)?) => {
        #[doc = concat!("A ", stringify!($n), "-dimensional vector")]
        pub type $alias<T = f32> = $ty<T, $n>;

        impl<T> $ty<T, $n> $(where $($clause)+)? {
            #[doc = concat!("Create a new vector in ", stringify!($n), "-dimensional space")]
            #[inline(always)]
            pub const fn new($($r: T),*) -> Self {
                $ty::from_array([$($r,)*])
            }

            const fn elements_layout_matches() {
                if size_of::<Self>() < size_of::< $elements<T> >() {
                    panic!(concat!("Size of ", stringify!($ty)," is less than or equal to size of ", stringify!($elements)));
                }
                if align_of::<Self>() > align_of::< $elements<T> >() {
                    panic!(concat!("Alignment of ", stringify!($ty)," is greater than or equal to alignment of ", stringify!($elements)));
                }

                let e = offset_of!(Self, e);
                let mut idx = 0;

                $(
                    let r = offset_of!($elements<T>, $r);

                    if r != e + idx * size_of::<T>() {
                        panic!(concat!("Offset of ", stringify!($r), " in ", stringify!($elements), " does not match offset of ", stringify!($ty), " at index ", stringify!($n)));
                    }
                    idx += 1;
                )*

                if idx != $n {
                    panic!(concat!("Number of elements in ", stringify!($elements), " does not match number of elements in ", stringify!($ty)));
                }
            }

            #[inline]
            fn as_elements(&self) -> &$elements<T> {
                #![allow(unsafe_code)]

                const { Self::elements_layout_matches(); }

                // This is safe because types have the identical memory layout.
                unsafe {
                    let ptr = self as *const Self as *const $elements<T>;
                    &*ptr
                }
            }

            #[inline]
            fn as_elements_mut(&mut self) -> &mut $elements<T> {
                #![allow(unsafe_code)]

                const { Self::elements_layout_matches(); }

                // This is safe because types have the identical memory layout.
                unsafe {
                    let ptr = self as *mut Self as *mut $elements<T>;
                    &mut *ptr
                }
            }
        }

        impl<T> Deref for $ty<T, $n> $(where $($clause)+)? {
            type Target = $elements<T>;

            #[inline(always)]
            fn deref(&self) -> &$elements<T> {
                self.as_elements()
            }
        }

        impl<T> DerefMut for $ty<T, $n> $(where $($clause)+)? {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut $elements<T> {
                self.as_elements_mut()
            }
        }
    };
}

impl_for_n!(Vector 1 Vector1 X [x]);
impl_for_n!(Vector 2 Vector2 XY [x y]);
impl_for_n!(Vector 3 Vector3 XYZ [x y z]);
impl_for_n!(Vector 4 Vector4 XYZW [x y z w]);

#[cfg(test)]
mod tests {
    use super::{Vector2, Vector3};

    #[test]
    fn vector_arithmetic() {
        let lhs = Vector2::new(2.0f32, -3.0);
        let rhs = Vector2::new(4.0f32, 5.0);

        assert_eq!(lhs + rhs, Vector2::new(6.0, 2.0));
        assert_eq!(lhs - rhs, Vector2::new(-2.0, -8.0));
        assert_eq!(lhs * 2.0, Vector2::new(4.0, -6.0));

        let mut value = lhs;
        value += &rhs;
        value -= rhs;
        value *= 3.0;
        assert_eq!(value, Vector2::new(6.0, -9.0));
    }

    #[test]
    fn dot_and_cross_products() {
        let lhs = Vector3::new(1.0f32, 2.0, 3.0);
        let rhs = Vector3::new(4.0f32, 5.0, 6.0);

        assert_eq!(lhs.dot(&rhs), 32.0);
        assert_eq!(lhs.cross(&rhs), Vector3::new(-3.0, 6.0, -3.0));
    }
}
