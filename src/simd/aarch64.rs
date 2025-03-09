#![allow(unsafe_code)]

use core::arch::aarch64::*;

macro_rules! impls {
    ($($ty:ident = $prim:ident * $n:literal {
        $($simd:ident)?
        $([$simd_m:ident; $m:literal])?
    })+) => {
        $(
            #[repr(transparent)]
            pub struct $ty(
                $($simd)?
                $([$simd_m; $m])?
            );
        )+
    };
}

impls! {
    U8x1 = u8 * 1 { u8 }
    U8x2 = u8 * 2 { u16 }
    U8x4 = u8 * 4 { u32 }
    U8x8 = u8 * 8 { uint8x8_t }
    U8x16 = u8 * 16 { uint8x16_t }

    U16x1 = u16 * 1 { u16 }
    U16x2 = u16 * 2 { u32 }
    U16x4 = u16 * 4 { uint16x4_t }
    U16x8 = u16 * 8 { uint16x8_t }
    U16x16 = u16 * 16 { [uint16x8_t; 2] }

    U32x1 = u32 * 1 { u32 }
    U32x2 = u32 * 2 { uint32x2_t }
    U32x4 = u32 * 4 { uint32x4_t }
    U32x8 = u32 * 8 { [uint32x4_t; 2] }
    U32x16 = u32 * 16 { [uint32x4_t; 4] }

    U64x1 = u64 * 1 { uint64x1_t }
    U64x2 = u64 * 2 { uint64x2_t }
    U64x4 = u64 * 4 { [uint64x2_t; 2] }
    U64x8 = u64 * 8 { [uint64x2_t; 4] }
    U64x16 = u64 * 16 { [uint64x2_t; 8] }

    I8x1 = i8 * 1 { i8 }
    I8x2 = i8 * 2 { int8x8_t }
    I8x4 = i8 * 4 { int8x8_t }
    I8x8 = i8 * 8 { int8x8_t }
    I8x16 = i8 * 16 { int8x16_t }

    I16x1 = i16 * 1 { i16 }
    I16x2 = i16 * 2 { int16x4_t }
    I16x4 = i16 * 4 { int16x4_t }
    I16x8 = i16 * 8 { int16x8_t }
    I16x16 = i16 * 16 { [int16x8_t; 2] }

    I32x1 = i32 * 1 { i32 }
    I32x2 = i32 * 2 { int32x2_t }
    I32x4 = i32 * 4 { int32x4_t }
    I32x8 = i32 * 8 { [int32x4_t; 2] }
    I32x16 = i32 * 16 { [int32x4_t; 4] }

    I64x1 = i64 * 1 { int64x1_t }
    I64x2 = i64 * 2 { int64x2_t }
    I64x4 = i64 * 4 { [int64x2_t; 2] }
    I64x8 = i64 * 8 { [int64x2_t; 4] }
    I64x16 = i64 * 16 { [int64x2_t; 8] }

    F32x1 = f32 * 1 { f32 }
    F32x2 = f32 * 2 { float32x2_t }
    F32x4 = f32 * 4 { float32x4_t }
    F32x8 = f32 * 8 { [float32x4_t; 2] }
    F32x16 = f32 * 16 { [float32x4_t; 4] }

    F64x1 = f64 * 1 { float64x1_t }
    F64x2 = f64 * 2 { float64x2_t }
    F64x4 = f64 * 4 { [float64x2_t; 2] }
    F64x8 = f64 * 8 { [float64x2_t; 4] }
    F64x16 = f64 * 16 { [float64x2_t; 8] }
}

impl U8x1 {
    #[inline(always)]
    pub fn from_array(elements: [u8; 1]) -> Self {
        U8x1(elements[0])
    }
}

impl U8x2 {
    #[inline(always)]
    pub fn from_array(elements: [u8; 2]) -> Self {
        U8x2(u16::from_ne_bytes(elements))
    }
}

impl U8x4 {
    #[inline(always)]
    pub fn from_array(elements: [u8; 4]) -> Self {
        U8x4(u32::from_ne_bytes(elements))
    }
}

impl U8x8 {
    #[inline(always)]
    pub fn from_array(elements: [u8; 8]) -> Self {
        U8x8(unsafe { vld1_u8(elements.as_ptr()) })
    }
}

impl U8x16 {
    #[inline(always)]
    pub fn from_array(elements: [u8; 16]) -> Self {
        U8x16(unsafe { vld1q_u8(elements.as_ptr()) })
    }
}

impl U16x1 {
    #[inline(always)]
    pub fn from_array(elements: [u16; 1]) -> Self {
        Self(elements[0])
    }
}

impl U16x2 {
    #[inline(always)]
    pub fn from_array(elements: [u16; 2]) -> Self {
        U16x2(unsafe { core::mem::transmute(elements) })
    }
}

impl U16x4 {
    #[inline(always)]
    pub fn from_array(elements: [u16; 4]) -> Self {
        U16x4(unsafe { vld1_u16(elements.as_ptr()) })
    }
}

impl U16x8 {
    #[inline(always)]
    pub fn from_array(elements: [u16; 8]) -> Self {
        U16x8(unsafe { vld1q_u16(elements.as_ptr()) })
    }
}

impl U16x16 {
    #[inline]
    pub fn from_array(elements: [u16; 16]) -> Self {
        let ptr = elements.as_ptr();
        U16x16([unsafe { vld1q_u16(ptr) }, unsafe { vld1q_u16(ptr.add(8)) }])
    }
}

impl U32x1 {
    #[inline(always)]
    pub fn from_array(elements: [u32; 1]) -> Self {
        U32x1(elements[0])
    }
}

impl U32x2 {
    #[inline(always)]
    pub fn from_array(elements: [u32; 2]) -> Self {
        U32x2(unsafe { vld1_u32(elements.as_ptr()) })
    }
}

impl U32x4 {
    #[inline(always)]
    pub fn from_array(elements: [u32; 4]) -> Self {
        U32x4(unsafe { vld1q_u32(elements.as_ptr()) })
    }
}

impl U32x8 {
    #[inline]
    pub fn from_array(elements: [u32; 8]) -> Self {
        let ptr = elements.as_ptr();
        U32x8([unsafe { vld1q_u32(ptr) }, unsafe { vld1q_u32(ptr.add(4)) }])
    }
}

impl U32x16 {
    #[inline]
    pub fn from_array(elements: [u32; 16]) -> Self {
        let ptr = elements.as_ptr();
        U32x16([
            unsafe { vld1q_u32(ptr) },
            unsafe { vld1q_u32(ptr.add(4)) },
            unsafe { vld1q_u32(ptr.add(8)) },
            unsafe { vld1q_u32(ptr.add(12)) },
        ])
    }
}

impl U64x1 {
    #[inline(always)]
    pub fn from_array(elements: [u64; 1]) -> Self {
        U64x1(unsafe { vld1_u64(elements.as_ptr()) })
    }
}

impl U64x2 {
    #[inline(always)]
    pub fn from_array(elements: [u64; 2]) -> Self {
        U64x2(unsafe { vld1q_u64(elements.as_ptr()) })
    }
}

impl U64x4 {
    #[inline]
    pub fn from_array(elements: [u64; 4]) -> Self {
        let ptr = elements.as_ptr();
        U64x4([unsafe { vld1q_u64(ptr) }, unsafe { vld1q_u64(ptr.add(2)) }])
    }
}

impl U64x8 {
    #[inline]
    pub fn from_array(elements: [u64; 8]) -> Self {
        let ptr = elements.as_ptr();
        U64x8([
            unsafe { vld1q_u64(ptr) },
            unsafe { vld1q_u64(ptr.add(2)) },
            unsafe { vld1q_u64(ptr.add(4)) },
            unsafe { vld1q_u64(ptr.add(6)) },
        ])
    }
}

impl U64x16 {
    #[inline]
    pub fn from_array(elements: [u64; 16]) -> Self {
        let ptr = elements.as_ptr();
        U64x16([
            unsafe { vld1q_u64(ptr) },
            unsafe { vld1q_u64(ptr.add(2)) },
            unsafe { vld1q_u64(ptr.add(4)) },
            unsafe { vld1q_u64(ptr.add(6)) },
            unsafe { vld1q_u64(ptr.add(8)) },
            unsafe { vld1q_u64(ptr.add(10)) },
            unsafe { vld1q_u64(ptr.add(12)) },
            unsafe { vld1q_u64(ptr.add(14)) },
        ])
    }
}

impl I8x1 {
    #[inline(always)]
    pub fn from_array(elements: [i8; 1]) -> Self {
        I8x1(elements[0])
    }
}

impl I8x2 {
    #[inline(always)]
    pub fn from_array(elements: [i8; 2]) -> Self {
        let array = [elements[0], elements[1], 0, 0, 0, 0, 0, 0];
        I8x2(unsafe { vld1_s8(array.as_ptr()) })
    }
}

impl I8x4 {
    #[inline(always)]
    pub fn from_array(elements: [i8; 4]) -> Self {
        let array = [
            elements[0],
            elements[1],
            elements[2],
            elements[3],
            0,
            0,
            0,
            0,
        ];
        I8x4(unsafe { vld1_s8(array.as_ptr()) })
    }
}

impl I8x8 {
    #[inline(always)]
    pub fn from_array(elements: [i8; 8]) -> Self {
        I8x8(unsafe { vld1_s8(elements.as_ptr()) })
    }
}

impl I8x16 {
    #[inline(always)]
    pub fn from_array(elements: [i8; 16]) -> Self {
        I8x16(unsafe { vld1q_s8(elements.as_ptr()) })
    }
}

impl I16x1 {
    #[inline(always)]
    pub fn from_array(elements: [i16; 1]) -> Self {
        Self(elements[0])
    }
}

impl I16x2 {
    #[inline(always)]
    pub fn from_array(elements: [i16; 2]) -> Self {
        let array = [elements[0], elements[1], 0, 0];
        I16x2(unsafe { vld1_s16(array.as_ptr()) })
    }
}

impl I16x4 {
    #[inline(always)]
    pub fn from_array(elements: [i16; 4]) -> Self {
        I16x4(unsafe { vld1_s16(elements.as_ptr()) })
    }
}

impl I16x8 {
    #[inline(always)]
    pub fn from_array(elements: [i16; 8]) -> Self {
        I16x8(unsafe { vld1q_s16(elements.as_ptr()) })
    }
}

impl I16x16 {
    #[inline]
    pub fn from_array(elements: [i16; 16]) -> Self {
        let ptr = elements.as_ptr();
        I16x16([unsafe { vld1q_s16(ptr) }, unsafe { vld1q_s16(ptr.add(8)) }])
    }
}

impl I32x1 {
    #[inline(always)]
    pub fn from_array(elements: [i32; 1]) -> Self {
        I32x1(elements[0])
    }
}

impl I32x2 {
    #[inline(always)]
    pub fn from_array(elements: [i32; 2]) -> Self {
        I32x2(unsafe { vld1_s32(elements.as_ptr()) })
    }
}

impl I32x4 {
    #[inline(always)]
    pub fn from_array(elements: [i32; 4]) -> Self {
        I32x4(unsafe { vld1q_s32(elements.as_ptr()) })
    }
}

impl I32x8 {
    #[inline]
    pub fn from_array(elements: [i32; 8]) -> Self {
        let ptr = elements.as_ptr();
        I32x8([unsafe { vld1q_s32(ptr) }, unsafe { vld1q_s32(ptr.add(4)) }])
    }
}

impl I32x16 {
    #[inline]
    pub fn from_array(elements: [i32; 16]) -> Self {
        let ptr = elements.as_ptr();
        I32x16([
            unsafe { vld1q_s32(ptr) },
            unsafe { vld1q_s32(ptr.add(4)) },
            unsafe { vld1q_s32(ptr.add(8)) },
            unsafe { vld1q_s32(ptr.add(12)) },
        ])
    }
}

impl I64x1 {
    #[inline(always)]
    pub fn from_array(elements: [i64; 1]) -> Self {
        I64x1(unsafe { vld1_s64(elements.as_ptr()) })
    }
}

impl I64x2 {
    #[inline(always)]
    pub fn from_array(elements: [i64; 2]) -> Self {
        I64x2(unsafe { vld1q_s64(elements.as_ptr()) })
    }
}

impl I64x4 {
    #[inline]
    pub fn from_array(elements: [i64; 4]) -> Self {
        let ptr = elements.as_ptr();
        I64x4([unsafe { vld1q_s64(ptr) }, unsafe { vld1q_s64(ptr.add(2)) }])
    }
}

impl I64x8 {
    #[inline]
    pub fn from_array(elements: [i64; 8]) -> Self {
        let ptr = elements.as_ptr();
        I64x8([
            unsafe { vld1q_s64(ptr) },
            unsafe { vld1q_s64(ptr.add(2)) },
            unsafe { vld1q_s64(ptr.add(4)) },
            unsafe { vld1q_s64(ptr.add(6)) },
        ])
    }
}

impl I64x16 {
    #[inline]
    pub fn from_array(elements: [i64; 16]) -> Self {
        let ptr = elements.as_ptr();
        I64x16([
            unsafe { vld1q_s64(ptr) },
            unsafe { vld1q_s64(ptr.add(2)) },
            unsafe { vld1q_s64(ptr.add(4)) },
            unsafe { vld1q_s64(ptr.add(6)) },
            unsafe { vld1q_s64(ptr.add(8)) },
            unsafe { vld1q_s64(ptr.add(10)) },
            unsafe { vld1q_s64(ptr.add(12)) },
            unsafe { vld1q_s64(ptr.add(14)) },
        ])
    }
}

impl F32x1 {
    #[inline(always)]
    pub fn from_array(elements: [f32; 1]) -> Self {
        F32x1(elements[0])
    }
}

impl F32x2 {
    #[inline(always)]
    pub fn from_array(elements: [f32; 2]) -> Self {
        F32x2(unsafe { vld1_f32(elements.as_ptr()) })
    }
}

impl F32x4 {
    #[inline(always)]
    pub fn from_array(elements: [f32; 4]) -> Self {
        F32x4(unsafe { vld1q_f32(elements.as_ptr()) })
    }
}

impl F32x8 {
    #[inline]
    pub fn from_array(elements: [f32; 8]) -> Self {
        let ptr = elements.as_ptr();
        F32x8([unsafe { vld1q_f32(ptr) }, unsafe { vld1q_f32(ptr.add(4)) }])
    }
}

impl F32x16 {
    #[inline]
    pub fn from_array(elements: [f32; 16]) -> Self {
        let ptr = elements.as_ptr();
        F32x16([
            unsafe { vld1q_f32(ptr) },
            unsafe { vld1q_f32(ptr.add(4)) },
            unsafe { vld1q_f32(ptr.add(8)) },
            unsafe { vld1q_f32(ptr.add(12)) },
        ])
    }
}

impl F64x1 {
    #[inline(always)]
    pub fn from_array(elements: [f64; 1]) -> Self {
        F64x1(unsafe { vld1_f64(elements.as_ptr()) })
    }
}

impl F64x2 {
    #[inline(always)]
    pub fn from_array(elements: [f64; 2]) -> Self {
        F64x2(unsafe { vld1q_f64(elements.as_ptr()) })
    }
}

impl F64x4 {
    #[inline]
    pub fn from_array(elements: [f64; 4]) -> Self {
        let ptr = elements.as_ptr();
        F64x4([unsafe { vld1q_f64(ptr) }, unsafe { vld1q_f64(ptr.add(2)) }])
    }
}

impl F64x8 {
    #[inline]
    pub fn from_array(elements: [f64; 8]) -> Self {
        let ptr = elements.as_ptr();
        F64x8([
            unsafe { vld1q_f64(ptr) },
            unsafe { vld1q_f64(ptr.add(2)) },
            unsafe { vld1q_f64(ptr.add(4)) },
            unsafe { vld1q_f64(ptr.add(6)) },
        ])
    }
}

impl F64x16 {
    #[inline]
    pub fn from_array(elements: [f64; 16]) -> Self {
        let ptr = elements.as_ptr();
        F64x16([
            unsafe { vld1q_f64(ptr) },
            unsafe { vld1q_f64(ptr.add(2)) },
            unsafe { vld1q_f64(ptr.add(4)) },
            unsafe { vld1q_f64(ptr.add(6)) },
            unsafe { vld1q_f64(ptr.add(8)) },
            unsafe { vld1q_f64(ptr.add(10)) },
            unsafe { vld1q_f64(ptr.add(12)) },
            unsafe { vld1q_f64(ptr.add(14)) },
        ])
    }
}
