#[cfg(feature = "Unity+Mathematics+double4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct double4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[cfg(feature = "Unity+Mathematics+double4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::double4 =>
    "Unity.Mathematics"."double4"
);
#[cfg(feature = "Unity+Mathematics+double4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::double4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+double4")]
impl crate::Unity::Mathematics::double4 {
    #[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::double4_DebuggerProxy;
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
    pub fn Equals_double4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::double4,
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
    pub fn _ctor__cordl_bool9(
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
    pub fn _ctor_bool4_10(
        &mut self,
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2_double2_5(
        &mut self,
        xy: crate::Unity::Mathematics::double2,
        zw: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy, zw),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2_f64_f64_4(
        &mut self,
        xy: crate::Unity::Mathematics::double2,
        z: f64,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy, z, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double3_f64_6(
        &mut self,
        xyz: crate::Unity::Mathematics::double3,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xyz, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double4_7(
        &mut self,
        xyzw: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xyzw),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_17(
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
    pub fn _ctor_f64_8(
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
    pub fn _ctor_f64_double2_f64_2(
        &mut self,
        x: f64,
        yz: crate::Unity::Mathematics::double2,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, yz, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_double3_3(
        &mut self,
        x: f64,
        yzw: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, yzw),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_f64_double2_1(
        &mut self,
        x: f64,
        y: f64,
        zw: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, zw),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_f64_f64_f64_0(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_18(
        &mut self,
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half15(
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
    pub fn _ctor_half4_16(
        &mut self,
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_11(
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
    pub fn _ctor_int4_12(
        &mut self,
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_13(
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
    pub fn _ctor_uint4_14(
        &mut self,
        v: crate::Unity::Mathematics::uint4,
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
    pub fn get_ww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_www(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_www",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wwzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wwzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wxzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wywx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wywx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wywy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wywy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wywz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wywz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xwzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xwzz",
            (),
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
    pub fn get_xxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxwz",
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
    pub fn get_xxxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxw",
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
    pub fn get_xxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxz",
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
    pub fn get_xxyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyw",
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
    pub fn get_xxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzz",
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
    pub fn get_xyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xywx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xywx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xywy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xywy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xywz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xywz",
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
    pub fn get_xyxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxw",
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
    pub fn get_xyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxz",
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
    pub fn get_xyyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyw",
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
    pub fn get_xyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ywzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ywzz",
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
    pub fn get_yxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxwz",
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
    pub fn get_yxxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxw",
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
    pub fn get_yxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxz",
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
    pub fn get_yxyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyw",
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
    pub fn get_yxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzz",
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
    pub fn get_yyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yywx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yywx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yywy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yywy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yywz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yywz",
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
    pub fn get_yyxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxw",
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
    pub fn get_yyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxz",
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
    pub fn get_yyyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyw",
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
    pub fn get_yyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zwzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zwzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zywx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zywx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zywy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zywy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zywz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zywz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzww(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzww",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzwx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzwx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzwy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzwy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzwz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzwz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzw(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool4_1(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f64_0(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_float4_8(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half4_6(
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half5(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int4_2(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint4_4(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double4_double4_0(
        lhs: crate::Unity::Mathematics::double4,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_double4_f64_1(
        lhs: crate::Unity::Mathematics::double4,
        rhs: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f64_double4_2(
        lhs: f64,
        rhs: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
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
    pub fn set_wx(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wxy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wxy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wxyz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wxyz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wxz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wxz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wxzy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wxzy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wy(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wyx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wyx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wyxz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wyxz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wyz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wyz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wyzx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wyzx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wz(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wzx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wzx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wzxy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wzxy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wzy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wzy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wzyx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wzyx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xw(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xwy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xwy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xwyz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xwyz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xwz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xwz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xwzy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xwzy",
            (value),
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
    pub fn set_xyw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xyw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xywz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xywz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xyz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xyz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xyzw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xyzw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xz(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xzw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xzw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xzwy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xzwy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xzy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xzy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xzyw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xzyw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yw(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ywx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ywx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ywxz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ywxz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ywz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ywz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ywzx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ywzx",
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
    pub fn set_yxw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yxw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yxwz(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yxwz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yxz(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yxz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yxzw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yxzw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yz(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yzw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yzw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yzwx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yzwx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yzx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yzx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yzxw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yzxw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zw(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zwx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zwx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zwxy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zwxy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zwy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zwy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zwyx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zwyx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zx(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zxw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zxw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zxwy(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zxwy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zxy(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zxy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zxyw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zxyw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zy(
        &mut self,
        value: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zyw(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zyw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zywx(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zywx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zyx(
        &mut self,
        value: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zyx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zyxw(
        &mut self,
        value: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zyxw",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+double4")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::double4 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::Unity::Mathematics::double4 {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4>>
for crate::Unity::Mathematics::double4 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4>>
for crate::Unity::Mathematics::double4 {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::double4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct double4_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::double4_DebuggerProxy =>
    "Unity.Mathematics"."double4/DebuggerProxy"
);
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::double4_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::double4_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
impl crate::Unity::Mathematics::double4_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+double4+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::double4_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
