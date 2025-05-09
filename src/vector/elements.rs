use super::Vector;

/// Type to implement x and y elements access.
#[repr(C)]
pub struct X<T> {
    /// Vector element 'x'
    pub x: T,
}

/// Type to implement x and y elements access.
#[repr(C)]
pub struct XY<T> {
    /// Vector element 'x'
    pub x: T,
    /// Vector element 'y'
    pub y: T,
}

/// Type to implement x, y and z elements access.
#[repr(C)]
pub struct XYZ<T> {
    /// Vector element 'x'
    pub x: T,
    /// Vector element 'y'
    pub y: T,
    /// Vector element 'z'
    pub z: T,
}

/// Type to implement x, y, z and w elements access.
#[repr(C)]
pub struct XYZW<T> {
    /// Vector element 'x'
    pub x: T,
    /// Vector element 'y'
    pub y: T,
    /// Vector element 'z'
    pub z: T,
    /// Vector element 'w'
    pub w: T,
}

macro_rules! impl_swizzle {
    (for impl $n:literal $($swizzle:ident => $($e:ident)+),+ $(,)?) => {};
    (for $head:ident $(,$tail:ident)* impl $n:literal $($swizzle:ident => $($e:ident)+),+ $(,)?) => {
        impl<T> $head<T> where T: Copy
        {
            $(
                #[doc = concat!("Returns the ", stringify!($swizzle), " swizzle.")]
                ///
                // /// Check swizzle.
                // ///
                // /// ```
                // #[doc = concat!("assert_eq!(", stringify!($n), ", \"", stringify!($swizzle), "\".len());")]
                // #[doc = concat!("assert_eq!(\"", $(stringify!($e),)+ "\", \"", stringify!($swizzle), "\");")]
                // /// ```
                pub fn $swizzle(&self) -> Vector<T, $n> {
                    Vector { e: [$(self.$e,)+] }
                }
            )+
        }

        impl_swizzle!(for $($tail),* impl $n $($swizzle => $($e)+),+);
    }
}

impl_swizzle!(for X, XY, XYZ, XYZW impl 1 x => x);
impl_swizzle!(for X, XY, XYZ, XYZW impl 2 xx => x x);
impl_swizzle!(for X, XY, XYZ, XYZW impl 3 xxx => x x x);
impl_swizzle!(for X, XY, XYZ, XYZW impl 4 xxxx => x x x x);

impl_swizzle!(for XY, XYZ, XYZW impl 1 y => y);

impl_swizzle!(for XY, XYZ, XYZW impl 2
               xy => x y,
    yx => y x, yy => y y,
);

impl_swizzle!(for XY, XYZ, XYZW impl 3
                  xxy => x x y, xyx => x y x, xyy => x y y,
    yxx => y x x, yxy => y x y, yyx => y y x, yyy => y y y,
);

impl_swizzle!(for XY, XYZ, XYZW impl 4
                     xxxy => x x x y, xxyx => x x y x, xxyy => x x y y, xyxx => x y x x, xyxy => x y x y, xyyx => x y y x, xyyy => x y y y,
    yxxx => y x x x, yxxy => y x x y, yxyx => y x y x, yxyy => y x y y, yyxx => y y x x, yyxy => y y x y, yyyx => y y y x, yyyy => y y y y,
);

impl_swizzle!(for XYZ, XYZW impl 1 z => z);
impl_swizzle!(for XYZ, XYZW impl 2
                          xz => x z,
                          yz => y z,
    zx => z x, zy => z y, zz => z z,
);

impl_swizzle!(for XYZ, XYZW impl 3
                                xxz => x x z,                             xyz => x y z, xzx => x z x, xzy => x z y, xzz => x z z,
                                yxz => y x z,                             yyz => y y z, yzx => y z x, yzy => y z y, yzz => y z z,
    zxx => z x x, zxy => z x y, zxz => z x z, zyx => z y x, zyy => z y y, zyz => z y z, zzx => z z x, zzy => z z y, zzz => z z z,
);

impl_swizzle!(for XYZ, XYZW impl 4
                                      xxxz => x x x z,                                   xxyz => x x y z, xxzx => x x z x, xxzy => x x z y, xxzz => x x z z,
                                      xyxz => x y x z,                                   xyyz => x y y z, xyzx => x y z x, xyzy => x y z y, xyzz => x y z z,
    xzxx => x z x x, xzxy => x z x y, xzxz => x z x z, xzyx => x z y x, xzyy => x z y y, xzyz => x z y z, xzzx => x z z x, xzzy => x z z y, xzzz => x z z z,
                                      yxxz => y x x z,                                   yxyz => y x y z, yxzx => y x z x, yxzy => y x z y, yxzz => y x z z,
                                      yyxz => y y x z,                                   yyyz => y y y z, yyzx => y y z x, yyzy => y y z y, yyzz => y y z z,
    yzxx => y z x x, yzxy => y z x y, yzxz => y z x z, yzyx => y z y x, yzyy => y z y y, yzyz => y z y z, yzzx => y z z x, yzzy => y z z y, yzzz => y z z z,
    zxxx => z x x x, zxxy => z x x y, zxxz => z x x z, zxyx => z x y x, zxyy => z x y y, zxyz => z x y z, zxzx => z x z x, zxzy => z x z y, zxzz => z x z z,
    zyxx => z y x x, zyxy => z y x y, zyxz => z y x z, zyyx => z y y x, zyyy => z y y y, zyyz => z y y z, zyzx => z y z x, zyzy => z y z y, zyzz => z y z z,
    zzxx => z z x x, zzxy => z z x y, zzxz => z z x z, zzyx => z z y x, zzyy => z z y y, zzyz => z z y z, zzzx => z z z x, zzzy => z z z y, zzzz => z z z z,
);

impl_swizzle!(for XYZW impl 1 w => w);
impl_swizzle!(for XYZW impl 2 xw => x w, yw => y w, zw => z w, wx => w x, wy => w y, wz => w z, ww => w w);
impl_swizzle!(for XYZW impl 3
                                              xxw => x x w,                                           xyw => x y w,
                                              xzw => x z w, xwx => x w x, xwy => x w y, xwz => x w z, xww => x w w,
                                              yxw => y x w,                                           yyw => y y w,
                                              yzw => y z w, ywx => y w x, ywy => y w y, ywz => y w z, yww => y w w,
                                              zxw => z x w,                                           zyw => z y w,
                                              zzw => z z w, zwx => z w x, zwy => z w y, zwz => z w z, zww => z w w,
    wxx => w x x, wxy => w x y, wxz => w x z, wxw => w x w, wyx => w y x, wyy => w y y, wyz => w y z, wyw => w y w,
    wzx => w z x, wzy => w z y, wzz => w z z, wzw => w z w, wwx => w w x, wwy => w w y, wwz => w w z, www => w w w,
);

impl_swizzle!(for XYZW impl 4
                                                       xxxw => x x x w,                                                    xxyw => x x y w,
                                                       xxzw => x x z w, xxwx => x x w x, xxwy => x x w y, xxwz => x x w z, xxww => x x w w,
                                                       xyxw => x y x w,                                                    xyyw => x y y w,
                                                       xyzw => x y z w, xywx => x y w x, xywy => x y w y, xywz => x y w z, xyww => x y w w,
                                                       xzxw => x z x w,                                                    xzyw => x z y w,
                                                       xzzw => x z z w, xzwx => x z w x, xzwy => x z w y, xzwz => x z w z, xzww => x z w w,
    xwxx => x w x x, xwxy => x w x y, xwxz => x w x z, xwxw => x w x w, xwyx => x w y x, xwyy => x w y y, xwyz => x w y z, xwyw => x w y w,
    xwzx => x w z x, xwzy => x w z y, xwzz => x w z z, xwzw => x w z w, xwwx => x w w x, xwwy => x w w y, xwwz => x w w z, xwww => x w w w,

                                                       yxxw => y x x w,                                                    yxyw => y x y w,
                                                       yxzw => y x z w, yxwx => y x w x, yxwy => y x w y, yxwz => y x w z, yxww => y x w w,
                                                       yyxw => y y x w,                                                    yyyw => y y y w,
                                                       yyzw => y y z w, yywx => y y w x, yywy => y y w y, yywz => y y w z, yyww => y y w w,
                                                       yzxw => y z x w,                                                    yzyw => y z y w,
                                                       yzzw => y z z w, yzwx => y z w x, yzwy => y z w y, yzwz => y z w z, yzww => y z w w,
    ywxx => y w x x, ywxy => y w x y, ywxz => y w x z, ywxw => y w x w, ywyx => y w y x, ywyy => y w y y, ywyz => y w y z, ywyw => y w y w,
    ywzx => y w z x, ywzy => y w z y, ywzz => y w z z, ywzw => y w z w, ywwx => y w w x, ywwy => y w w y, ywwz => y w w z, ywww => y w w w,

                                                       zxxw => z x x w,                                                    zxyw => z x y w,
                                                       zxzw => z x z w, zxwx => z x w x, zxwy => z x w y, zxwz => z x w z, zxww => z x w w,
                                                       zyxw => z y x w,                                                    zyyw => z y y w,
                                                       zyzw => z y z w, zywx => z y w x, zywy => z y w y, zywz => z y w z, zyww => z y w w,
                                                       zzxw => z z x w,                                                    zzyw => z z y w,
                                                       zzzw => z z z w, zzwx => z z w x, zzwy => z z w y, zzwz => z z w z, zzww => z z w w,
    zwxx => z w x x, zwxy => z w x y, zwxz => z w x z, zwxw => z w x w, zwyx => z w y x, zwyy => z w y y, zwyz => z w y z, zwyw => z w y w,
    zwzx => z w z x, zwzy => z w z y, zwzz => z w z z, zwzw => z w z w, zwwx => z w w x, zwwy => z w w y, zwwz => z w w z, zwww => z w w w,

    wxxx => w x x x, wxxy => w x x y, wxxz => w x x z, wxxw => w x x w, wxyx => w x y x, wxyy => w x y y, wxyz => w x y z, wxyw => w x y w,
    wxzx => w x z x, wxzy => w x z y, wxzz => w x z z, wxzw => w x z w, wxwx => w x w x, wxwy => w x w y, wxwz => w x w z, wxww => w x w w,
    wyxx => w y x x, wyxy => w y x y, wyxz => w y x z, wyxw => w y x w, wyyx => w y y x, wyyy => w y y y, wyyz => w y y z, wyyw => w y y w,
    wyzx => w y z x, wyzy => w y z y, wyzz => w y z z, wyzw => w y z w, wywx => w y w x, wywy => w y w y, wywz => w y w z, wyww => w y w w,
    wzxx => w z x x, wzxy => w z x y, wzxz => w z x z, wzxw => w z x w, wzyx => w z y x, wzyy => w z y y, wzyz => w z y z, wzyw => w z y w,
    wzzx => w z z x, wzzy => w z z y, wzzz => w z z z, wzzw => w z z w, wzwx => w z w x, wzwy => w z w y, wzwz => w z w z, wzww => w z w w,
    wwxx => w w x x, wwxy => w w x y, wwxz => w w x z, wwxw => w w x w, wwyx => w w y x, wwyy => w w y y, wwyz => w w y z, wwyw => w w y w,
    wwzx => w w z x, wwzy => w w z y, wwzz => w w z z, wwzw => w w z w, wwwx => w w w x, wwwy => w w w y, wwwz => w w w z, wwww => w w w w,
);
