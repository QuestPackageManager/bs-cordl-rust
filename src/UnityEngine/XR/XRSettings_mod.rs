#[cfg(feature = "UnityEngine+XR+XRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRSettings => "UnityEngine.XR"
    ."XRSettings"
);
#[cfg(feature = "UnityEngine+XR+XRSettings")]
impl std::ops::Deref for crate::UnityEngine::XR::XRSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
impl std::ops::DerefMut for crate::UnityEngine::XR::XRSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
impl crate::UnityEngine::XR::XRSettings {
    #[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
    pub type StereoRenderingMode = crate::UnityEngine::XR::XRSettings_StereoRenderingMode;
    pub fn get_deviceEyeTextureDimension() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::TextureDimension,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::TextureDimension = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_deviceEyeTextureDimension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTextureHeight() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTextureHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTextureResolutionScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTextureResolutionScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyeTextureWidth() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_eyeTextureWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDeviceActive() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_isDeviceActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_loadedDeviceName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_loadedDeviceName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_occlusionMaskScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_occlusionMaskScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderViewportScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderViewportScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderViewportScaleInternal() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderViewportScaleInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showDeviceView() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_showDeviceView", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stereoRenderingMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::XR::XRSettings_StereoRenderingMode,
    > {
        let __cordl_ret: crate::UnityEngine::XR::XRSettings_StereoRenderingMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_stereoRenderingMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useOcclusionMesh() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_useOcclusionMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyeTextureResolutionScale(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_eyeTextureResolutionScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderViewportScale(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_renderViewportScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderViewportScaleInternal(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_renderViewportScaleInternal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_showDeviceView(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_showDeviceView", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::XRSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XRSettings_StereoRenderingMode {
    MultiPass = 0i32,
    SinglePass = 1i32,
    SinglePassInstanced = 2i32,
    SinglePassMultiview = 3i32,
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRSettings_StereoRenderingMode
    => "UnityEngine.XR"."XRSettings/StereoRenderingMode"
);
