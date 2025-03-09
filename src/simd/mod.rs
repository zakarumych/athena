//! SIMD support for several architectures.
//!
//! And fallback implementations for unsupported architectures.
//!

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
mod aarch64;

#[cfg(not(any(all(target_arch = "aarch64", target_feature = "neon"))))]
mod fallback;

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub use self::aarch64::*;

pub trait Simd<const N: usize>: Sized {
    type Array;

    fn from_array(elements: [Self; N]) -> Self::Array;
}

macro_rules! impl_simd {
    ($($ty:ident | $($n:literal as $simd:ident),+ $(,)?)+) => {
        $(
            $(
                impl Simd<$n> for $ty {
                    type Array = $simd;

                    #[inline(always)]
                    fn from_array(elements: [Self; $n]) -> Self::Array {
                        $simd::from_array(elements)
                    }
                }
            )+
        )+
    };
}

impl_simd!(
    u8 | 1 as U8x1,
    2 as U8x2,
    4 as U8x4,
    8 as U8x8,
    16 as U8x16,
    u16 | 1 as U16x1,
    2 as U16x2,
    4 as U16x4,
    8 as U16x8,
    16 as U16x16,
    u32 | 1 as U32x1,
    2 as U32x2,
    4 as U32x4,
    8 as U32x8,
    16 as U32x16,
    u64 | 1 as U64x1,
    2 as U64x2,
    4 as U64x4,
    8 as U64x8,
    16 as U64x16,
    i8 | 1 as I8x1,
    2 as I8x2,
    4 as I8x4,
    8 as I8x8,
    16 as I8x16,
    i16 | 1 as I16x1,
    2 as I16x2,
    4 as I16x4,
    8 as I16x8,
    16 as I16x16,
    i32 | 1 as I32x1,
    2 as I32x2,
    4 as I32x4,
    8 as I32x8,
    16 as I32x16,
    i64 | 1 as I64x1,
    2 as I64x2,
    4 as I64x4,
    8 as I64x8,
    16 as I64x16,
    f32 | 1 as F32x1,
    2 as F32x2,
    4 as F32x4,
    8 as F32x8,
    16 as F32x16,
    f64 | 1 as F64x1,
    2 as F64x2,
    4 as F64x4,
    8 as F64x8,
    16 as F64x16,
);
