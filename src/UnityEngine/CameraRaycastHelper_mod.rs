#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CameraRaycastHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CameraRaycastHelper =>
    "UnityEngine"."CameraRaycastHelper"
);
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl std::ops::Deref for crate::UnityEngine::CameraRaycastHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl std::ops::DerefMut for crate::UnityEngine::CameraRaycastHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl crate::UnityEngine::CameraRaycastHelper {}
#[cfg(feature = "UnityEngine+CameraRaycastHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::CameraRaycastHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
