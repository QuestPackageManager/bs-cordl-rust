#[cfg(feature = "Unity+Mathematics+quaternion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct quaternion {
    pub value: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::quaternion =>
    "Unity.Mathematics"."quaternion"
);
#[cfg(feature = "Unity+Mathematics+quaternion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::quaternion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl crate::Unity::Mathematics::quaternion {
    pub fn Equals_Il2CppObject1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_quaternion0(
        &mut self,
        x: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
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
    pub fn _ctor_f32_f32_f32_f32_0(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z, w),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3x3_2(
        &mut self,
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4_1(
        &mut self,
        value: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float4x4_3(
        &mut self,
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion>>
for crate::Unity::Mathematics::quaternion {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::quaternion> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::quaternion {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+quaternion")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::quaternion {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
