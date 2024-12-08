#[cfg(feature = "Unity+Mathematics+RigidTransform")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RigidTransform {
    pub rot: crate::Unity::Mathematics::quaternion,
    pub pos: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::RigidTransform =>
    "Unity.Mathematics"."RigidTransform"
);
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::RigidTransform {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+RigidTransform")]
impl crate::Unity::Mathematics::RigidTransform {
    pub fn Equals_Object1(
        &mut self,
        x: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_RigidTransform0(
        &mut self,
        x: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (x),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider1(
        &mut self,
        format: *mut crate::System::String,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_float3x3_float3_1(
        &mut self,
        rotation: crate::Unity::Mathematics::float3x3,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rotation, translation),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_float4x4_2(
        &mut self,
        transform: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (transform),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_quaternion_float3_0(
        &mut self,
        rotation: crate::Unity::Mathematics::quaternion,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rotation, translation),
        )?;
        Ok(__cordl_ret)
    }
}
