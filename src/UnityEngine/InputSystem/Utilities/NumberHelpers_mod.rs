#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NumberHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::NumberHelpers =>
    "UnityEngine.InputSystem.Utilities"."NumberHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::NumberHelpers {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
