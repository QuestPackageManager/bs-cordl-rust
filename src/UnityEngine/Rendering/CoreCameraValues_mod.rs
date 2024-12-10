#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CoreCameraValues {
    pub filterMode: i32,
    pub cullingMask: u32,
    pub instanceID: i32,
}
#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CoreCameraValues =>
    "UnityEngine.Rendering"."CoreCameraValues"
);
#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CoreCameraValues {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
impl crate::UnityEngine::Rendering::CoreCameraValues {
    pub fn Equals_CoreCameraValues0(
        &mut self,
        other: crate::UnityEngine::Rendering::CoreCameraValues,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
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
}
#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CoreCameraValues>>
for crate::UnityEngine::Rendering::CoreCameraValues {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::CoreCameraValues> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CoreCameraValues")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CoreCameraValues>>
for crate::UnityEngine::Rendering::CoreCameraValues {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::CoreCameraValues,
    > {
        todo!()
    }
}
