#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputDeviceCommandInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo =>
    "UnityEngine.InputSystem.LowLevel"."IInputDeviceCommandInfo"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
impl crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = __cordl_object
            .invoke("get_typeStatic", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+IInputDeviceCommandInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
