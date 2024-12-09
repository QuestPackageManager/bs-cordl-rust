#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputObjectInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameObject: *mut crate::UnityEngine::GameObject,
    pub _terrainReference: *mut crate::UnityEngine::Terrain,
    pub _boundingVolumeReference: *mut crate::GlobalNamespace::HEU_BoundingVolume,
    pub _tilemapReference: *mut crate::UnityEngine::Tilemaps::Tilemap,
    pub _syncdTransform: crate::UnityEngine::Matrix4x4,
    pub _useTransformOffset: bool,
    pub _translateOffset: crate::UnityEngine::Vector3,
    pub _rotateOffset: crate::UnityEngine::Vector3,
    pub _scaleOffset: crate::UnityEngine::Vector3,
    pub _inputInterfaceType: *mut crate::System::Type,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputObjectInfo =>
    "HoudiniEngineUnity"."HEU_InputObjectInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    pub fn CopyTo(
        &mut self,
        destObject: *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (destObject))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_InputObjectInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetReferencesFromGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReferencesFromGameObject", ())?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
