#[cfg(feature = "Unity+Burst+Intrinsics+v256")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct v256 {
    padding: quest_hook::libil2cpp::ValueTypePadding<32usize>,
}
#[cfg(feature = "Unity+Burst+Intrinsics+v256")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::v256 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "v256";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Burst::Intrinsics::v256 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Burst::Intrinsics::v256 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Burst::Intrinsics::v256 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Burst::Intrinsics::v256 {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+v256")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::v256 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+v256")]
impl crate::Unity::Burst::Intrinsics::v256 {
    pub fn _ctor_f32_12(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_13(
        &mut self,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        g: f32,
        h: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_14(
        &mut self,
        f: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_f64_f64_f64_15(
        &mut self,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i16_4(
        &mut self,
        v: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_i16_5(
        &mut self,
        a: i16,
        b: i16,
        c: i16,
        d: i16,
        e: i16,
        f: i16,
        g: i16,
        h: i16,
        i: i16,
        j: i16,
        k: i16,
        l: i16,
        m: i16,
        n: i16,
        o: i16,
        p: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_8(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_i32_9(
        &mut self,
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_16(
        &mut self,
        f: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_i64_i64_i64_17(
        &mut self,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i8_2(
        &mut self,
        b: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_3(
        &mut self,
        a: i8,
        b: i8,
        c: i8,
        d: i8,
        e: i8,
        f: i8,
        g: i8,
        h: i8,
        i: i8,
        j: i8,
        k: i8,
        l: i8,
        m: i8,
        n: i8,
        o: i8,
        p: i8,
        q: i8,
        r: i8,
        s: i8,
        t: i8,
        u: i8,
        v: i8,
        w: i8,
        x: i8,
        y: i8,
        z: i8,
        A: i8,
        B: i8,
        C: i8,
        D: i8,
        E: i8,
        F: i8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                i,
                j,
                k,
                l,
                m,
                n,
                o,
                p,
                q,
                r,
                s,
                t,
                u,
                v,
                w,
                x,
                y,
                z,
                A,
                B,
                C,
                D,
                E,
                F,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u16_6(
        &mut self,
        v: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_u16_7(
        &mut self,
        a: u16,
        b: u16,
        c: u16,
        d: u16,
        e: u16,
        f: u16,
        g: u16,
        h: u16,
        i: u16,
        j: u16,
        k: u16,
        l: u16,
        m: u16,
        n: u16,
        o: u16,
        p: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_10(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_u32_u32_u32_u32_u32_u32_u32_11(
        &mut self,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        g: u32,
        h: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_18(
        &mut self,
        f: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_u64_u64_u64_19(
        &mut self,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_0(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_1(
        &mut self,
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
        j: u8,
        k: u8,
        l: u8,
        m: u8,
        n: u8,
        o: u8,
        p: u8,
        q: u8,
        r: u8,
        s: u8,
        t: u8,
        u: u8,
        v: u8,
        w: u8,
        x: u8,
        y: u8,
        z: u8,
        A: u8,
        B: u8,
        C: u8,
        D: u8,
        E: u8,
        F: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                i,
                j,
                k,
                l,
                m,
                n,
                o,
                p,
                q,
                r,
                s,
                t,
                u,
                v,
                w,
                x,
                y,
                z,
                A,
                B,
                C,
                D,
                E,
                F,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_v128_v128_20(
        &mut self,
        lo: crate::Unity::Burst::Intrinsics::v128,
        hi: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (lo, hi),
        )?;
        Ok(__cordl_ret.into())
    }
}
