#[cfg(feature = "PoseObject")]
#[repr(C)]
#[derive(Debug)]
pub struct PoseObject {
    __cordl_parent: crate::System::Object,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _poseObjectId: *mut PoseObjectIdSO,
}
#[cfg(feature = "PoseObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PoseObject => ""."PoseObject"
);
#[cfg(feature = "PoseObject")]
impl std::ops::Deref for PoseObject {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PoseObject")]
impl std::ops::DerefMut for PoseObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PoseObject")]
impl PoseObject {
    pub fn New(
        transform: *mut crate::UnityEngine::Transform,
        poseObjectId: *mut PoseObjectIdSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (transform, poseObjectId))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        transform: *mut crate::UnityEngine::Transform,
        poseObjectId: *mut PoseObjectIdSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (transform, poseObjectId))?;
        Ok(__cordl_ret)
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_objectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_objectTransform", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PoseObject")]
impl quest_hook::libil2cpp::ObjectType for PoseObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
