#[cfg(feature = "Unity+Mathematics+double2")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct double2 {
    pub x: f64,
    pub y: f64,
}
#[cfg(feature = "Unity+Mathematics+double2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::double2 =>
    "Unity.Mathematics"."double2"
);
#[cfg(feature = "Unity+Mathematics+double2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::double2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+double2")]
impl crate::Unity::Mathematics::double2 {
    #[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::double2_DebuggerProxy;
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
    pub fn Equals_double2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::double2,
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
    pub fn _ctor_double2_1(
        &mut self,
        xy: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_11(
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
    pub fn _ctor_f64_f64_0(
        &mut self,
        x: f64,
        y: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float2_12(
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
    pub fn _ctor_half2_10(
        &mut self,
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half9(
        &mut self,
        v: crate::Unity::Mathematics::half,
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
    pub fn _ctor_int2_6(
        &mut self,
        v: crate::Unity::Mathematics::int2,
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
    pub fn _ctor_uint2_8(
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
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2_1(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f64_0(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float2_8(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half2_6(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half5(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int2_2(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint2_4(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double2_double2_0(
        lhs: crate::Unity::Mathematics::double2,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double2_f64_1(
        lhs: crate::Unity::Mathematics::double2,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f64_double2_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: f64,
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
        value: crate::Unity::Mathematics::double2,
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
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+double2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::double2>>
for crate::Unity::Mathematics::double2 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::double2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::double2>>
for crate::Unity::Mathematics::double2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::double2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double2")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::double2 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double2")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::double2 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct double2_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: f64,
    pub y: f64,
}
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::double2_DebuggerProxy =>
    "Unity.Mathematics"."double2/DebuggerProxy"
);
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::double2_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::double2_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
impl crate::Unity::Mathematics::double2_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+double2+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::double2_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
