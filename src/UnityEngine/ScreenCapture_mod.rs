#[cfg(feature = "UnityEngine+ScreenCapture")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenCapture {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ScreenCapture")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ScreenCapture => "UnityEngine"
    ."ScreenCapture"
);
#[cfg(feature = "UnityEngine+ScreenCapture")]
impl std::ops::Deref for crate::UnityEngine::ScreenCapture {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScreenCapture")]
impl std::ops::DerefMut for crate::UnityEngine::ScreenCapture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ScreenCapture")]
impl crate::UnityEngine::ScreenCapture {
    #[cfg(feature = "UnityEngine+ScreenCapture+StereoScreenCaptureMode")]
    pub type StereoScreenCaptureMode = crate::UnityEngine::ScreenCapture_StereoScreenCaptureMode;
}
#[cfg(feature = "UnityEngine+ScreenCapture")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ScreenCapture {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ScreenCapture+StereoScreenCaptureMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCapture_StereoScreenCaptureMode {
    BothEyes = 3i32,
    LeftEye = 1i32,
    RightEye = 2i32,
}
#[cfg(feature = "UnityEngine+ScreenCapture+StereoScreenCaptureMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ScreenCapture_StereoScreenCaptureMode => "UnityEngine"
    ."ScreenCapture/StereoScreenCaptureMode"
);
