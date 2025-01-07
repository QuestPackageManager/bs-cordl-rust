#[cfg(feature = "UnityEngine+XR+XRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::XRSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "XRSettings";
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XRSettings_StereoRenderingMode {
    #[default]
    MultiPass = 0i32,
    SinglePass = 1i32,
    SinglePassInstanced = 2i32,
    SinglePassMultiview = 3i32,
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::XR::XRSettings_StereoRenderingMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "StereoRenderingMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::XRSettings_StereoRenderingMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::XRSettings_StereoRenderingMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::XRSettings_StereoRenderingMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+XR+XRSettings+StereoRenderingMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::XR::XRSettings_StereoRenderingMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
