#[cfg(feature = "Unity+Mathematics+half3")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct half3 {
    pub x: crate::Unity::Mathematics::half,
    pub y: crate::Unity::Mathematics::half,
    pub z: crate::Unity::Mathematics::half,
}
#[cfg(feature = "Unity+Mathematics+half3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::half3 => "Unity.Mathematics"
    ."half3"
);
#[cfg(feature = "Unity+Mathematics+half3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::half3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+half3")]
impl crate::Unity::Mathematics::half3 {
    #[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
    pub type DebuggerProxy = crate::Unity::Mathematics::half3_DebuggerProxy;
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
    pub fn Equals_half3_0(
        &mut self,
        rhs: crate::Unity::Mathematics::half3,
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
    pub fn _ctor_double3_8(
        &mut self,
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_5(
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
    pub fn _ctor_f64_7(
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
    pub fn _ctor_float3_6(
        &mut self,
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half2_half2(
        &mut self,
        xy: crate::Unity::Mathematics::half2,
        z: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xy, z),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half3_3(
        &mut self,
        xyz: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (xyz),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half4(
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
    pub fn _ctor_half_half2_1(
        &mut self,
        x: crate::Unity::Mathematics::half,
        yz: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, yz),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_half_half_half0(
        &mut self,
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
        z: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xxzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_xzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_xzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yxzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_yzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_yzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zxzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zxzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zyzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zyzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzxz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzxz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzyz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzyz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzx(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzx",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzy",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zzzz(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_zzzz",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: crate::Unity::Mathematics::half,
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
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xyz(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xyz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xz(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_xzy(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_xzy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yx(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yxz(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yxz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yz(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yz",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_yzx(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_yzx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zx(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zxy(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zxy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zy(
        &mut self,
        value: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zy",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_zyx(
        &mut self,
        value: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_zyx",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct half3_DebuggerProxy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub x: crate::Unity::Mathematics::half,
    pub y: crate::Unity::Mathematics::half,
    pub z: crate::Unity::Mathematics::half,
}
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::half3_DebuggerProxy =>
    "Unity.Mathematics"."half3/DebuggerProxy"
);
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
impl std::ops::Deref for crate::Unity::Mathematics::half3_DebuggerProxy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
impl std::ops::DerefMut for crate::Unity::Mathematics::half3_DebuggerProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
impl crate::Unity::Mathematics::half3_DebuggerProxy {
    pub fn New(
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (v))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+half3+DebuggerProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Mathematics::half3_DebuggerProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
