#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
#[repr(C)]
#[derive(Debug)]
pub struct BoneControl {
    __cordl_parent: crate::UnityEngine::InputSystem::InputControl_1<
        crate::UnityEngine::InputSystem::XR::Bone,
    >,
    pub _parentBoneIndex_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    pub _position_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    pub _rotation_k__BackingField: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::XR::BoneControl =>
    "UnityEngine.InputSystem.XR"."BoneControl"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::XR::BoneControl {
    type Target = crate::UnityEngine::InputSystem::InputControl_1<
        crate::UnityEngine::InputSystem::XR::Bone,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::XR::BoneControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
impl crate::UnityEngine::InputSystem::XR::BoneControl {
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
    pub fn ReadUnprocessedValueFromState(
        &mut self,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::XR::Bone> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Bone = __cordl_object
            .invoke("ReadUnprocessedValueFromState", (statePtr))?;
        Ok(__cordl_ret)
    }
    pub fn WriteValueIntoState(
        &mut self,
        value: crate::UnityEngine::InputSystem::XR::Bone,
        statePtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValueIntoState", (value, statePtr))?;
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
    pub fn get_parentBoneIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl = __cordl_object
            .invoke("get_parentBoneIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl = __cordl_object
            .invoke("get_rotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_parentBoneIndex(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::IntegerControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_parentBoneIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_position(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::Vector3Control,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_position", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rotation(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::Controls::QuaternionControl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotation", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+BoneControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::XR::BoneControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
