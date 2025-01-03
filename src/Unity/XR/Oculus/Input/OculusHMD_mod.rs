#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusHMD {
    __cordl_parent: crate::UnityEngine::InputSystem::XR::XRHMD,
    pub _userPresence_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::ButtonControl,
    >,
    pub _trackingState_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::IntegerControl,
    >,
    pub _isTracked_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::ButtonControl,
    >,
    pub _devicePosition_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _deviceRotation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    >,
    pub _deviceAngularVelocity_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _deviceAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _deviceAngularAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _leftEyePosition_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _leftEyeRotation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    >,
    pub _leftEyeAngularVelocity_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _leftEyeAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _leftEyeAngularAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _rightEyePosition_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _rightEyeRotation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    >,
    pub _rightEyeAngularVelocity_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _rightEyeAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _rightEyeAngularAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _centerEyePosition_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _centerEyeRotation_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    >,
    pub _centerEyeAngularVelocity_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _centerEyeAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _centerEyeAngularAcceleration_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Input::OculusHMD =>
    "Unity.XR.Oculus.Input"."OculusHMD"
);
#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Input::OculusHMD {
    type Target = crate::UnityEngine::InputSystem::XR::XRHMD;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Input::OculusHMD {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
impl crate::Unity::XR::Oculus::Input::OculusHMD {
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_centerEyeAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_centerEyeAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_centerEyeAngularAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_centerEyeAngularAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_centerEyeAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_centerEyeAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_centerEyePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_centerEyePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_centerEyeRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        > = __cordl_object.invoke("get_centerEyeRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_deviceAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceAngularAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_deviceAngularAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_deviceAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_devicePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_devicePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        > = __cordl_object.invoke("get_deviceRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isTracked(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object.invoke("get_isTracked", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftEyeAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_leftEyeAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftEyeAngularAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_leftEyeAngularAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftEyeAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_leftEyeAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftEyePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_leftEyePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftEyeRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        > = __cordl_object.invoke("get_leftEyeRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightEyeAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_rightEyeAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightEyeAngularAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_rightEyeAngularAcceleration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightEyeAngularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_rightEyeAngularVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightEyePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        > = __cordl_object.invoke("get_rightEyePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightEyeRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        > = __cordl_object.invoke("get_rightEyeRotation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trackingState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        > = __cordl_object.invoke("get_trackingState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userPresence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        > = __cordl_object.invoke("get_userPresence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_centerEyeAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyeAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_centerEyeAngularAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyeAngularAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_centerEyeAngularVelocity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyeAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_centerEyePosition(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_centerEyeRotation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_centerEyeRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceAngularAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceAngularAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceAngularVelocity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_devicePosition(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_devicePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceRotation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_isTracked(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isTracked", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftEyeAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyeAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftEyeAngularAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyeAngularAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftEyeAngularVelocity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyeAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftEyePosition(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftEyeRotation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftEyeRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightEyeAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyeAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightEyeAngularAcceleration(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyeAngularAcceleration", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightEyeAngularVelocity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyeAngularVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightEyePosition(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyePosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightEyeRotation(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::QuaternionControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightEyeRotation", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_trackingState(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::IntegerControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackingState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_userPresence(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_userPresence", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Input+OculusHMD")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Input::OculusHMD {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
