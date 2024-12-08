#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
#[repr(C)]
#[derive(Debug)]
pub struct DualShock4GamepadAndroid {
    __cordl_parent: crate::UnityEngine::InputSystem::DualShock::DualShockGamepad,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::DualShock4GamepadAndroid =>
    "UnityEngine.InputSystem.Android"."DualShock4GamepadAndroid"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Android::DualShock4GamepadAndroid {
    type Target = crate::UnityEngine::InputSystem::DualShock::DualShockGamepad;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Android::DualShock4GamepadAndroid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
impl crate::UnityEngine::InputSystem::Android::DualShock4GamepadAndroid {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+DualShock4GamepadAndroid")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Android::DualShock4GamepadAndroid {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
