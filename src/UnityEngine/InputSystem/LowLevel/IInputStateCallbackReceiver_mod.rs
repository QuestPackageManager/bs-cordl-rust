#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputStateCallbackReceiver {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver =>
    "UnityEngine.InputSystem.LowLevel"."IInputStateCallbackReceiver"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
impl crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
    pub fn GetStateOffsetForEvent(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        offset: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetStateOffsetForEvent", (control, eventPtr, offset))?;
        Ok(__cordl_ret)
    }
    pub fn OnNextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNextUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnStateEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStateEvent", (eventPtr))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputStateCallbackReceiver")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::IInputStateCallbackReceiver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
