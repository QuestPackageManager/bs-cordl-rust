#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputObjectInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _terrainReference: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    pub _boundingVolumeReference: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HEU_BoundingVolume,
    >,
    pub _tilemapReference: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Tilemaps::Tilemap,
    >,
    pub _syncdTransform: crate::UnityEngine::Matrix4x4,
    pub _useTransformOffset: bool,
    pub _translateOffset: crate::UnityEngine::Vector3,
    pub _rotateOffset: crate::UnityEngine::Vector3,
    pub _scaleOffset: crate::UnityEngine::Vector3,
    pub _inputInterfaceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputObjectInfo =>
    "HoudiniEngineUnity"."HEU_InputObjectInfo"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        destObject: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_InputObjectInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (destObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetReferencesFromGameObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReferencesFromGameObject", ())?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
    >,
> for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputObjectInfo")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
    >,
> for crate::HoudiniEngineUnity::HEU_InputObjectInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_InputObjectInfo>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
