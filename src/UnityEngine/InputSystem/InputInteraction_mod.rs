#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
#[repr(C)]
#[derive(Debug)]
pub struct InputInteraction {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputInteraction =>
    "UnityEngine.InputSystem"."InputInteraction"
);
#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputInteraction {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputInteraction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
impl crate::UnityEngine::InputSystem::InputInteraction {}
#[cfg(feature = "UnityEngine+InputSystem+InputInteraction")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputInteraction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
