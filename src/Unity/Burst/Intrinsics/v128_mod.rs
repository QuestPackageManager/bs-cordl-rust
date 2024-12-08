#[cfg(feature = "Unity+Burst+Intrinsics+v128")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct v128 {
    padding: [u8; 16usize],
}
#[cfg(feature = "Unity+Burst+Intrinsics+v128")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::v128 =>
    "Unity.Burst.Intrinsics"."v128"
);
#[cfg(feature = "Unity+Burst+Intrinsics+v128")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::v128 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+v128")]
impl crate::Unity::Burst::Intrinsics::v128 {
    pub fn _ctor_f32_12(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (f),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_f32_f32_f32_13(
        &mut self,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_f64_f64_15(
        &mut self,
        a: f64,
        b: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_i16_i16_i16_i16_i16_i16_i16_i16_5(
        &mut self,
        a: i16,
        b: i16,
        c: i16,
        d: i16,
        e: i16,
        f: i16,
        g: i16,
        h: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i32_9(
        &mut self,
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_i64_17(
        &mut self,
        a: i64,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_i8_3(
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_u16_u16_u16_u16_u16_u16_u16_u16_7(
        &mut self,
        a: u16,
        b: u16,
        c: u16,
        d: u16,
        e: u16,
        f: u16,
        g: u16,
        h: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_u32_u32_u32_u32_11(
        &mut self,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_u64_u64_19(
        &mut self,
        a: u64,
        b: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_u8_1(
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_v64_v64_20(
        &mut self,
        lo: crate::Unity::Burst::Intrinsics::v64,
        hi: crate::Unity::Burst::Intrinsics::v64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (lo, hi),
        )?;
        Ok(__cordl_ret)
    }
}
