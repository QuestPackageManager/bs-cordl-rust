#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputUpdateCallbackReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::IInputUpdateCallbackReceiver =>
    "UnityEngine.InputSystem.LowLevel"."IInputUpdateCallbackReceiver"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::IInputUpdateCallbackReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::IInputUpdateCallbackReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
impl crate::UnityEngine::InputSystem::LowLevel::IInputUpdateCallbackReceiver {
    pub fn OnUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputUpdateCallbackReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::IInputUpdateCallbackReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
