#[cfg(feature = "Unity+Mathematics+int4x3")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct int4x3 {
    pub c0: crate::Unity::Mathematics::int4,
    pub c1: crate::Unity::Mathematics::int4,
    pub c2: crate::Unity::Mathematics::int4,
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::int4x3 =>
    "Unity.Mathematics"."int4x3"
);
#[cfg(feature = "Unity+Mathematics+int4x3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::int4x3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl crate::Unity::Mathematics::int4x3 {
    pub fn Equals_Gc1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_int4x3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Gc_Gc1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool4x3_4(
        &mut self,
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double4x3_10(
        &mut self,
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_7(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x3_8(
        &mut self,
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
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
    pub fn _ctor_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        &mut self,
        m00: i32,
        m01: i32,
        m02: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m30: i32,
        m31: i32,
        m32: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int4_int4_int4_0(
        &mut self,
        c0: crate::Unity::Mathematics::int4,
        c1: crate::Unity::Mathematics::int4,
        c2: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_5(
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
    pub fn _ctor_uint4x3_6(
        &mut self,
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::int4>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::int4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4x3_1(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double4x3_7(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_4(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_6(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float4x3_5(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_uint4x3_3(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        x: crate::Unity::Mathematics::int4x3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LeftShift", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_OnesComplement(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_OnesComplement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        x: crate::Unity::Mathematics::int4x3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_RightShift", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_i32_int4x3_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int4x3_i32_1(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int4x3_int4x3_0(
        lhs: crate::Unity::Mathematics::int4x3,
        rhs: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::int4x3 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::int4x3 {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::int4x3>>
for crate::Unity::Mathematics::int4x3 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::int4x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int4x3")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::int4x3>>
for crate::Unity::Mathematics::int4x3 {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::int4x3> {
        todo!()
    }
}
