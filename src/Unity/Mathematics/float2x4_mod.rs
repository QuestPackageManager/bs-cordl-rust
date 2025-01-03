#[cfg(feature = "Unity+Mathematics+float2x4")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct float2x4 {
    pub c0: crate::Unity::Mathematics::float2,
    pub c1: crate::Unity::Mathematics::float2,
    pub c2: crate::Unity::Mathematics::float2,
    pub c3: crate::Unity::Mathematics::float2,
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::float2x4 =>
    "Unity.Mathematics"."float2x4"
);
#[cfg(feature = "Unity+Mathematics+float2x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float2x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl crate::Unity::Mathematics::float2x4 {
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_float2x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float2x4,
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
    pub fn ToString_Il2CppString_IFormatProvider1(
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
    pub fn _ctor_bool2x4_4(
        &mut self,
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2x4_10(
        &mut self,
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_2(
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
    pub fn _ctor_f32_f32_f32_f32_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m03, m10, m11, m12, m13),
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
    pub fn _ctor_float2_float2_float2_float2_0(
        &mut self,
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
        c2: crate::Unity::Mathematics::float2,
        c3: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2, c3),
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
    pub fn _ctor_int2x4_6(
        &mut self,
        v: crate::Unity::Mathematics::int2x4,
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
    pub fn _ctor_uint2x4_8(
        &mut self,
        v: crate::Unity::Mathematics::uint2x4,
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
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float2,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2x4_1(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2x4_3(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int2x4_2(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint2x4_4(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float2x4_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x4_f32_1(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x4_float2x4_0(
        lhs: crate::Unity::Mathematics::float2x4,
        rhs: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4>>
for crate::Unity::Mathematics::float2x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4>>
for crate::Unity::Mathematics::float2x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float2x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float2x4 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x4")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float2x4 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
