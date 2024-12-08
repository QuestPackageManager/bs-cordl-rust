#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct InputArrayExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::InputArrayExtensions =>
    "UnityEngine.InputSystem.Utilities"."InputArrayExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+InputArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::InputArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
