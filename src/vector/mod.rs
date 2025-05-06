//! Contains vector and operations for them.
//!
//!
//!
//!

use core::{
    mem::{align_of, offset_of, size_of},
    ops::{Deref, DerefMut},
};

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
    pub fn into_array(self) -> [T; N] {
        self.e
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

            const fn elements_layout_matches() -> bool {
                if size_of::<Self>() <= size_of::< $elements<T> >() {
                    return false;
                }
                if align_of::<Self>() >= align_of::< $elements<T> >() {
                    return false;
                }

                let e = offset_of!(Self, e);
                let mut idx = 0;

                $(
                    let r = offset_of!($elements<T>, $r);

                    if r != e + idx * size_of::<T>() {
                        return false;
                    }
                    idx += 1;
                )*

                idx == $n
            }

            #[inline]
            fn as_elements(&self) -> &$elements<T> {
                #![allow(unsafe_code)]

                const { assert!(Self::elements_layout_matches()); }

                // This is safe because types have the identical memory layout.
                unsafe {
                    let ptr = self as *const Self as *const $elements<T>;
                    &*ptr
                }
            }

            #[inline]
            fn as_elements_mut(&mut self) -> &mut $elements<T> {
                #![allow(unsafe_code)]

                const { assert!(Self::elements_layout_matches()); }

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
