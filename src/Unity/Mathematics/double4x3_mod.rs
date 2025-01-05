#[cfg(feature = "Unity+Mathematics+double4x3")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct double4x3 {
    pub c0: crate::Unity::Mathematics::double4,
    pub c1: crate::Unity::Mathematics::double4,
    pub c2: crate::Unity::Mathematics::double4,
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::double4x3 =>
    "Unity.Mathematics"."double4x3"
);
#[cfg(feature = "Unity+Mathematics+double4x3")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::double4x3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
impl crate::Unity::Mathematics::double4x3 {
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
    pub fn Equals_double4x3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::double4x3,
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
    pub fn _ctor_double4_double4_double4_0(
        &mut self,
        c0: crate::Unity::Mathematics::double4,
        c1: crate::Unity::Mathematics::double4,
        c2: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_9(
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
    pub fn _ctor_f64_2(
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
    pub fn _ctor_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_1(
        &mut self,
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m30: f64,
        m31: f64,
        m32: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x3_10(
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
    pub fn _ctor_i32_5(
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
    pub fn _ctor_int4x3_6(
        &mut self,
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_7(
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
    pub fn _ctor_uint4x3_8(
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
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double4>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::double4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4x3_1(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_5(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f64_0(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4x3_6(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int4x3_2(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint4x3_4(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double4x3_double4x3_0(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double4x3_f64_1(
        lhs: crate::Unity::Mathematics::double4x3,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f64_double4x3_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::double4x3 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::double4x3 {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4x3>>
for crate::Unity::Mathematics::double4x3 {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4x3> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4x3")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4x3>>
for crate::Unity::Mathematics::double4x3 {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4x3> {
        todo!()
    }
}
