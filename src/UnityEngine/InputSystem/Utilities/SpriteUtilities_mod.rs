#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct SpriteUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::SpriteUtilities =>
    "UnityEngine.InputSystem.Utilities"."SpriteUtilities"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::SpriteUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::SpriteUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
impl crate::UnityEngine::InputSystem::Utilities::SpriteUtilities {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+SpriteUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::SpriteUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
