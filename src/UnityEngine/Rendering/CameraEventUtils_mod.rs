#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct CameraEventUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CameraEventUtils =>
    "UnityEngine.Rendering"."CameraEventUtils"
);
#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CameraEventUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CameraEventUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
impl crate::UnityEngine::Rendering::CameraEventUtils {}
#[cfg(feature = "UnityEngine+Rendering+CameraEventUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::CameraEventUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}