#[cfg(feature = "Unity+XR+Oculus+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Utils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Utils";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Utils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl crate::Unity::XR::Oculus::Utils {
    pub fn EnableDynamicFFR(enable: bool) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnableDynamicFFR", (enable))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFoveationLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFoveationLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemHeadsetType() -> quest_hook::libil2cpp::Result<
        crate::Unity::XR::Oculus::SystemHeadset,
    > {
        let __cordl_ret: crate::Unity::XR::Oculus::SystemHeadset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemHeadsetType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEyeTrackingPermissionGranted() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEyeTrackingPermissionGranted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PermissionGrantedCallback(
        permissionName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PermissionGrantedCallback", (permissionName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorScaleAndOffset(
        colorScale: crate::UnityEngine::Vector4,
        colorOffset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetColorScaleAndOffset", (colorScale, colorOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFoveationLevel(level: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFoveationLevel", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackedFoveatedRenderingEnabled() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackedFoveatedRenderingEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTrackedFoveatedRenderingSupported() -> quest_hook::libil2cpp::Result<
        bool,
    > {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTrackedFoveatedRenderingSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_foveatedRenderingLevel() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_foveatedRenderingLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDynamicFoveatedRendering() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useDynamicFoveatedRendering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeTrackedFoveatedRenderingEnabled(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeTrackedFoveatedRenderingEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_foveatedRenderingLevel(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_foveatedRenderingLevel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useDynamicFoveatedRendering(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_useDynamicFoveatedRendering", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
