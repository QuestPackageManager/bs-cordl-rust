#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJoystick {
    __cordl_parent: crate::UnityEngine::InputSystem::Joystick,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::AndroidJoystick =>
    "UnityEngine.InputSystem.Android"."AndroidJoystick"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Android::AndroidJoystick {
    type Target = crate::UnityEngine::InputSystem::Joystick;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Android::AndroidJoystick {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
impl crate::UnityEngine::InputSystem::Android::AndroidJoystick {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+AndroidJoystick")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::AndroidJoystick {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
