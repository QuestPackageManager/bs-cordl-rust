#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseCaptureController {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseCaptureController
    => "UnityEngine.UIElements"."MouseCaptureController"
);
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseCaptureController {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseCaptureController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl crate::UnityEngine::UIElements::MouseCaptureController {}
#[cfg(feature = "UnityEngine+UIElements+MouseCaptureController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseCaptureController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}