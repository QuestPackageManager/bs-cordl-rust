#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct EnhancedTouchSupport {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport =>
    "UnityEngine.InputSystem.EnhancedTouch"."EnhancedTouchSupport"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+EnhancedTouchSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::EnhancedTouch::EnhancedTouchSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
