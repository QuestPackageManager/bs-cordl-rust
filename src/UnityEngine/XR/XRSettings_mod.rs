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
#[cfg(feature = "UnityEngine+XR+XRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSettings {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+XR+XRSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::XRSettings => "UnityEngine.XR"
    ."XRSettings"
);
#[cfg(feature = "UnityEngine+XR+XRSettings")]
impl std::ops::Deref for crate::UnityEngine::XR::XRSettings {
    type Target = crate::System::Object;
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
