#[cfg(feature = "PlayerTransforms")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerTransforms {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _headTransform: *mut crate::UnityEngine::Transform,
    pub _originTransform: *mut crate::UnityEngine::Transform,
    pub _leftHandTransform: *mut crate::UnityEngine::Transform,
    pub _rightHandTransform: *mut crate::UnityEngine::Transform,
    pub _overrideHeadPos: bool,
    pub _overriddenHeadPos: crate::UnityEngine::Vector3,
    pub _headWorldPos: crate::UnityEngine::Vector3,
    pub _headWorldRot: crate::UnityEngine::Quaternion,
    pub _headPseudoLocalPos: crate::UnityEngine::Vector3,
    pub _headPseudoLocalRot: crate::UnityEngine::Quaternion,
    pub _leftHandPseudoLocalPos: crate::UnityEngine::Vector3,
    pub _leftHandPseudoLocalRot: crate::UnityEngine::Quaternion,
    pub _rightHandPseudoLocalPos: crate::UnityEngine::Vector3,
    pub _rightHandPseudoLocalRot: crate::UnityEngine::Quaternion,
    pub _originParentTransform: *mut crate::UnityEngine::Transform,
    pub _useOriginParentTransformForPseudoLocalCalculations: bool,
}
#[cfg(feature = "PlayerTransforms")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerTransforms => ""
    ."PlayerTransforms"
);
#[cfg(feature = "PlayerTransforms")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerTransforms {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerTransforms")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerTransforms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerTransforms")]
impl crate::GlobalNamespace::PlayerTransforms {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearHeadPositionOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearHeadPositionOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetZPos(
        &mut self,
        start: f32,
        end: f32,
        headOffsetZ: f32,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetZPos", (start, end, headOffsetZ, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetZPosOffsetByHeadPosAtTime(
        &mut self,
        start: f32,
        end: f32,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetZPosOffsetByHeadPosAtTime", (start, end, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn HeadOffsetZ(
        &mut self,
        noteInverseWorldRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("HeadOffsetZ", (noteInverseWorldRotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveTowardsHead(
        &mut self,
        start: f32,
        end: f32,
        noteInverseWorldRotation: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("MoveTowardsHead", (start, end, noteInverseWorldRotation, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OverrideHeadPos(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverrideHeadPos", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_headPseudoLocalPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headPseudoLocalPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headPseudoLocalRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_headPseudoLocalRot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headWorldPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_headWorldPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headWorldRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_headWorldRot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHandPseudoLocalPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_leftHandPseudoLocalPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftHandPseudoLocalRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_leftHandPseudoLocalRot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightHandPseudoLocalPos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_rightHandPseudoLocalPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightHandPseudoLocalRot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Quaternion = __cordl_object
            .invoke("get_rightHandPseudoLocalRot", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerTransforms")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerTransforms {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
