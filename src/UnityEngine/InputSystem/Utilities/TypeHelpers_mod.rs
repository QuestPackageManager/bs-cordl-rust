#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::Utilities::TypeHelpers
    => "UnityEngine.InputSystem.Utilities"."TypeHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::TypeHelpers {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+TypeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::TypeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
