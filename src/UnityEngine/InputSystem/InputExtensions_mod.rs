#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputExtensions =>
    "UnityEngine.InputSystem"."InputExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
impl crate::UnityEngine::InputSystem::InputExtensions {}
#[cfg(feature = "UnityEngine+InputSystem+InputExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
