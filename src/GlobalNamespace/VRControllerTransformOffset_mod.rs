#[cfg(feature = "VRControllerTransformOffset")]
#[repr(C)]
#[derive(Debug)]
pub struct VRControllerTransformOffset {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "VRControllerTransformOffset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VRControllerTransformOffset =>
    ""."VRControllerTransformOffset"
);
#[cfg(feature = "VRControllerTransformOffset")]
impl std::ops::Deref for crate::GlobalNamespace::VRControllerTransformOffset {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllerTransformOffset")]
impl std::ops::DerefMut for crate::GlobalNamespace::VRControllerTransformOffset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRControllerTransformOffset")]
impl crate::GlobalNamespace::VRControllerTransformOffset {
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
    pub fn get_poseOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Pose> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Pose = __cordl_object
            .invoke("get_poseOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_positionOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_positionOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotationOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_rotationOffset", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRControllerTransformOffset")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::VRControllerTransformOffset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
