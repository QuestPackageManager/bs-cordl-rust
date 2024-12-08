#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct XRSupport {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::XRSupport =>
    "UnityEngine.InputSystem.XR"."XRSupport"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::XRSupport {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::XRSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
impl crate::UnityEngine::InputSystem::XR::XRSupport {}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::XRSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
