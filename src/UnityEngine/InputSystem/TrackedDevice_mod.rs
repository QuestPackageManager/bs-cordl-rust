#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackedDevice {
    __cordl_parent: crate::UnityEngine::InputSystem::InputDevice,
    pub _trackingState_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    pub _isTracked_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _devicePosition_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _deviceRotation_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
}
#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::TrackedDevice =>
    "UnityEngine.InputSystem"."TrackedDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::TrackedDevice {
    type Target = crate::UnityEngine::InputSystem::InputDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::TrackedDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
impl crate::UnityEngine::InputSystem::TrackedDevice {
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
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
    pub fn get_devicePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_devicePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deviceRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl = __cordl_object
            .invoke("get_deviceRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isTracked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_isTracked", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl = __cordl_object
            .invoke("get_trackingState", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_devicePosition(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_devicePosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_deviceRotation(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_isTracked(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isTracked", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackingState(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingState", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+TrackedDevice")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::TrackedDevice {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}