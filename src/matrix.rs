//! Matrices
//!

/// Column-major matrix type.
pub struct Matrix<T, const N: usize, const M: usize> {
    e: [[T; M]; N],
}
