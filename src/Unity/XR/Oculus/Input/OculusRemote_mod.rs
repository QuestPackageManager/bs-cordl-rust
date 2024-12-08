#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusRemote {
    __cordl_parent: crate::UnityEngine::InputSystem::InputDevice,
    pub _back_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _start_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub _touchpad_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Input::OculusRemote =>
    "Unity.XR.Oculus.Input"."OculusRemote"
);
#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Input::OculusRemote {
    type Target = crate::UnityEngine::InputSystem::InputDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Input::OculusRemote {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
impl crate::Unity::XR::Oculus::Input::OculusRemote {
    pub fn set_back(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_back", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_start", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_touchpad(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control = __cordl_object
            .invoke("get_touchpad", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_start(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_start", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_back(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::ButtonControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::ButtonControl = __cordl_object
            .invoke("get_back", ())?;
        Ok(__cordl_ret)
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
    pub fn set_touchpad(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector2Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_touchpad", (value))?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusRemote")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::Input::OculusRemote {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
