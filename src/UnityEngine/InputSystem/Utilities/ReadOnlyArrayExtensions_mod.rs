#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadOnlyArrayExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions =>
    "UnityEngine.InputSystem.Utilities"."ReadOnlyArrayExtensions"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+ReadOnlyArrayExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::ReadOnlyArrayExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}