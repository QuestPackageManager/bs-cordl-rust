#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct XInputSupport {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XInput::XInputSupport
    => "UnityEngine.InputSystem.XInput"."XInputSupport"
);
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XInput::XInputSupport {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XInput::XInputSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
impl crate::UnityEngine::InputSystem::XInput::XInputSupport {}
#[cfg(feature = "UnityEngine+InputSystem+XInput+XInputSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XInput::XInputSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
