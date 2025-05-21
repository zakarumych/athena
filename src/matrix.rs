//! Matrices
//!

use core::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Mul, MulAssign},
};

use crate::{Num, Vector};

/// Column-major matrix type.
/// `N` is the number of columns = row size.
/// `M` is the number of rows = column size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Matrix<T, const N: usize, const M: usize = N> {
    e: [[T; M]; N],
}

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    /// Constructs a new matrix from column arrays
    pub const fn from_column_arrays(e: [[T; M]; N]) -> Self {
        Matrix { e }
    }

    /// Constructs a new matrix from row arrays
    pub fn from_row_arrays(e: [[T; N]; M]) -> Self {
        #![allow(unsafe_code)]
        let e = ManuallyDrop::new(e);

        let mut elements = [const { [const { MaybeUninit::uninit() }; M] }; N];

        for n in 0..N {
            for m in 0..M {
                elements[n][m].write(unsafe { core::ptr::read(&e[m][n]) });
            }
        }

        Matrix {
            e: elements.map(|col| col.map(|v| unsafe { v.assume_init() })),
        }
    }

    /// Transposes the matrix.
    pub fn transpose(self) -> Matrix<T, M, N> {
        #![allow(unsafe_code)]
        let me = ManuallyDrop::new(self);

        let mut elements = [const { [const { MaybeUninit::uninit() }; N] }; M];

        for n in 0..N {
            for m in 0..M {
                elements[m][n].write(unsafe { core::ptr::read(&me.e[n][m]) });
            }
        }

        Matrix {
            e: elements.map(|col| col.map(|v| unsafe { v.assume_init() })),
        }
    }

    /// Returns a row vector.
    #[inline]
    pub fn row(&self, m: usize) -> Option<Vector<T, N>>
    where
        T: Copy,
    {
        if m >= M {
            return None;
        }

        let mut n = 0;
        let row = [(); N].map(|()| {
            let e = self.e[n][m];
            n += 1;
            e
        });

        Some(Vector::from_array(row))
    }

    /// Returns a column vector.
    #[inline(always)]
    pub const fn column(&self, n: usize) -> Option<Vector<T, M>>
    where
        T: Copy,
    {
        if n >= N {
            return None;
        }

        Some(Vector::from_array(self.e[n]))
    }

    /// Returns reference to a column vector.
    #[inline(always)]
    pub const fn column_ref(&self, n: usize) -> Option<&Vector<T, M>> {
        if n >= N {
            return None;
        }

        Some(Vector::from_array_ref(&self.e[n]))
    }

    /// Returns mutable reference to a column vector.
    #[inline(always)]
    pub const fn column_mut(&mut self, n: usize) -> Option<&mut Vector<T, M>> {
        if n >= N {
            return None;
        }

        Some(Vector::from_array_mut(&mut self.e[n]))
    }

    /// Returns reference to the matrix elements as arrays.
    pub const fn arrays(&self) -> &[[T; M]; N] {
        &self.e
    }

    /// Returns mutable reference to the matrix elements as arrays.
    pub const fn arrays_mut(&mut self) -> &mut [[T; M]; N] {
        &mut self.e
    }
}

impl<T, const M: usize> Matrix<T, 1, M> {
    /// Create a new column vector.
    #[inline(always)]
    pub fn from_column(e: Vector<T, M>) -> Self {
        Matrix {
            e: [e.into_array()],
        }
    }

    /// Extracts the column vector from the matrix.
    #[inline(always)]
    pub fn into_column(self) -> Vector<T, M> {
        let [e] = self.e;
        Vector::from_array(e)
    }
}

impl<T, const N: usize> Matrix<T, N, 1> {
    /// Create a new row vector.
    #[inline(always)]
    pub fn from_row(e: Vector<T, N>) -> Self {
        Matrix {
            e: e.into_array().map(|v| [v]),
        }
    }

    /// Extracts the row vector from the matrix.
    #[inline(always)]
    pub fn into_row(self) -> Vector<T, N> {
        Vector::from_array(self.e.map(|[v]| v))
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<&Matrix<T, K, M>> for &Matrix<T, N, K>
where
    T: Num,
{
    type Output = Matrix<T, N, M>;

    #[inline]
    fn mul(self, rhs: &Matrix<T, K, M>) -> Self::Output {
        let mut result = Matrix::<T, N, M> {
            e: [[T::ZERO; M]; N],
        };

        for n in 0..N {
            for m in 0..M {
                for k in 0..K {
                    result.e[n][m] += self.e[n][k] * rhs.e[k][m];
                }
            }
        }

        result
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<Matrix<T, K, M>> for &Matrix<T, N, K>
where
    T: Num,
{
    type Output = Matrix<T, N, M>;

    #[inline(always)]
    fn mul(self, rhs: Matrix<T, K, M>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<&Matrix<T, K, M>> for Matrix<T, N, K>
where
    T: Num,
{
    type Output = Matrix<T, N, M>;

    #[inline(always)]
    fn mul(self, rhs: &Matrix<T, K, M>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<T, const N: usize, const M: usize, const K: usize> Mul<Matrix<T, K, M>> for Matrix<T, N, K>
where
    T: Num,
{
    type Output = Matrix<T, N, M>;

    #[inline(always)]
    fn mul(self, rhs: Matrix<T, K, M>) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl<T, const N: usize> MulAssign<&Matrix<T, N>> for Matrix<T, N>
where
    T: Num,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &Matrix<T, N>) {
        *self = self.mul(rhs);
    }
}

impl<T, const N: usize> MulAssign<Matrix<T, N>> for Matrix<T, N>
where
    T: Num,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Matrix<T, N>) {
        *self = self.mul(rhs);
    }
}

impl<T, const N: usize, const M: usize> Mul<&Vector<T, N>> for &Matrix<T, N, M>
where
    T: Num,
{
    type Output = Vector<T, M>;

    #[inline]
    fn mul(self, rhs: &Vector<T, N>) -> Vector<T, M> {
        let mut result = Vector::<T, M>::ZERO;

        for n in 0..N {
            for m in 0..M {
                result[m] += self.e[n][m] * rhs[n];
            }
        }

        result
    }
}

impl<T, const N: usize, const M: usize> Mul<Vector<T, N>> for &Matrix<T, N, M>
where
    T: Num,
{
    type Output = Vector<T, M>;

    #[inline(always)]
    fn mul(self, rhs: Vector<T, N>) -> Vector<T, M> {
        self.mul(&rhs)
    }
}

impl<T, const N: usize, const M: usize> Mul<&Vector<T, N>> for Matrix<T, N, M>
where
    T: Num,
{
    type Output = Vector<T, M>;

    #[inline(always)]
    fn mul(self, rhs: &Vector<T, N>) -> Vector<T, M> {
        (&self).mul(rhs)
    }
}

impl<T, const N: usize, const M: usize> Mul<Vector<T, N>> for Matrix<T, N, M>
where
    T: Num,
{
    type Output = Vector<T, M>;

    #[inline(always)]
    fn mul(self, rhs: Vector<T, N>) -> Vector<T, M> {
        (&self).mul(&rhs)
    }
}

impl<T, const N: usize, const M: usize> Mul<&Matrix<T, N, M>> for &Vector<T, M>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: &Matrix<T, N, M>) -> Vector<T, N> {
        let mut result = Vector::<T, N>::ZERO;

        for n in 0..N {
            for m in 0..M {
                result[n] += rhs.e[n][m] * self[m];
            }
        }

        result
    }
}

impl<T, const N: usize, const M: usize> Mul<Matrix<T, N, M>> for &Vector<T, M>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: Matrix<T, N, M>) -> Vector<T, N> {
        self.mul(&rhs)
    }
}

impl<T, const N: usize, const M: usize> Mul<&Matrix<T, N, M>> for Vector<T, M>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: &Matrix<T, N, M>) -> Vector<T, N> {
        (&self).mul(rhs)
    }
}

impl<T, const N: usize, const M: usize> Mul<Matrix<T, N, M>> for Vector<T, M>
where
    T: Num,
{
    type Output = Vector<T, N>;

    #[inline(always)]
    fn mul(self, rhs: Matrix<T, N, M>) -> Vector<T, N> {
        (&self).mul(&rhs)
    }
}

/// Matrix with 1 column and 1 row.
pub type Matrix1x1<T> = Matrix<T, 1, 1>;

/// Matrix with 1 column and 2 rows.
pub type Matrix1x2<T> = Matrix<T, 1, 2>;

/// Matrix with 1 column and 3 rows.
pub type Matrix1x3<T> = Matrix<T, 1, 3>;

/// Matrix with 1 column and 4 rows.
pub type Matrix1x4<T> = Matrix<T, 1, 4>;

/// Matrix with 2 columns and 1 row.
pub type Matrix2x1<T> = Matrix<T, 2, 1>;

/// Matrix with 2 columns and 2 rows.
pub type Matrix2x2<T> = Matrix<T, 2, 2>;

/// Matrix with 2 columns and 3 rows.
pub type Matrix2x3<T> = Matrix<T, 2, 3>;

/// Matrix with 2 columns and 4 rows.
pub type Matrix2x4<T> = Matrix<T, 2, 4>;

/// Matrix with 3 columns and 1 row.
pub type Matrix3x1<T> = Matrix<T, 3, 1>;

/// Matrix with 3 columns and 2 rows.
pub type Matrix3x2<T> = Matrix<T, 3, 2>;

/// Matrix with 3 columns and 3 rows.
pub type Matrix3x3<T> = Matrix<T, 3, 3>;

/// Matrix with 3 columns and 4 rows.
pub type Matrix3x4<T> = Matrix<T, 3, 4>;

/// Matrix with 4 columns and 1 row.
pub type Matrix4x1<T> = Matrix<T, 4, 1>;

/// Matrix with 4 columns and 2 rows.
pub type Matrix4x2<T> = Matrix<T, 4, 2>;

/// Matrix with 4 columns and 3 rows.
pub type Matrix4x3<T> = Matrix<T, 4, 3>;

/// Matrix with 4 columns and 4 rows.
pub type Matrix4x4<T> = Matrix<T, 4, 4>;

/// Matrix with 1 column and 1 row.
pub type Matrix1<T> = Matrix<T, 1>;

/// Matrix with 2 columns and 2 rows.
pub type Matrix2<T> = Matrix<T, 2>;

/// Matrix with 3 columns and 3 rows.
pub type Matrix3<T> = Matrix<T, 3>;

/// Matrix with 4 columns and 4 rows.
pub type Matrix4<T> = Matrix<T, 4>;
