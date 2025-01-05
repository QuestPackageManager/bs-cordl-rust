#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
#[repr(C)]
#[derive(Debug)]
pub struct HololensHand {
    __cordl_parent: crate::UnityEngine::InputSystem::XR::XRController,
    pub _deviceVelocity_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
    pub _airTap_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::ButtonControl,
    >,
    pub _sourceLossRisk_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::AxisControl,
    >,
    pub _sourceLossMitigationDirection_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Controls::Vector3Control,
    >,
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::WindowsMR::Input::HololensHand
    => "UnityEngine.XR.WindowsMR.Input"."HololensHand"
);
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
impl std::ops::Deref for crate::UnityEngine::XR::WindowsMR::Input::HololensHand {
    type Target = crate::UnityEngine::InputSystem::XR::XRController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
impl std::ops::DerefMut for crate::UnityEngine::XR::WindowsMR::Input::HololensHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
impl crate::UnityEngine::XR::WindowsMR::Input::HololensHand {
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
    pub fn get_airTap(
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
        > = __cordl_object.invoke("get_airTap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deviceVelocity(
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
        > = __cordl_object.invoke("get_deviceVelocity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sourceLossMitigationDirection(
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
        > = __cordl_object.invoke("get_sourceLossMitigationDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sourceLossRisk(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Controls::AxisControl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        > = __cordl_object.invoke("get_sourceLossRisk", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_airTap(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::ButtonControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_airTap", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_deviceVelocity(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deviceVelocity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sourceLossMitigationDirection(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::Vector3Control,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sourceLossMitigationDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sourceLossRisk(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Controls::AxisControl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sourceLossRisk", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+WindowsMR+Input+HololensHand")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::XR::WindowsMR::Input::HololensHand {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
