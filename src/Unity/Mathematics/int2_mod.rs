#[cfg(feature = "Unity+Mathematics+int2")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct int2 {
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "Unity+Mathematics+int2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::int2 => "Unity.Mathematics"
    ."int2"
);
#[cfg(feature = "Unity+Mathematics+int2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::int2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl crate::Unity::Mathematics::int2 {
    #[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::int2_DebuggerProxy;
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
    pub fn Equals_int2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::int2,
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
    pub fn _ctor_bool2_4(
        &mut self,
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2_10(
        &mut self,
        v: crate::Unity::Mathematics::double2,
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
    pub fn _ctor_float2_8(
        &mut self,
        v: crate::Unity::Mathematics::float2,
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
    pub fn _ctor_i32_i32_0(
        &mut self,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int2_1(
        &mut self,
        xy: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy),
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
    pub fn _ctor_uint2_6(
        &mut self,
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2_1(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2_7(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_4(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_6(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_float2_5(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_uint2_3(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LeftShift", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_OnesComplement(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_OnesComplement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_RightShift", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_i32_int2_2(
        lhs: i32,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int2_i32_1(
        lhs: crate::Unity::Mathematics::int2,
        rhs: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_int2_int2_0(
        lhs: crate::Unity::Mathematics::int2,
        rhs: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xy(
        &mut self,
        value: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yx(
        &mut self,
        value: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::int2>>
for crate::Unity::Mathematics::int2 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::int2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::int2>>
for crate::Unity::Mathematics::int2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::int2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::int2 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::int2 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct int2_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::int2_DebuggerProxy =>
    "Unity.Mathematics"."int2/DebuggerProxy"
);
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::int2_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::int2_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl crate::Unity::Mathematics::int2_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+int2+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::int2_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
