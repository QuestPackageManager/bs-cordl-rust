#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_LoadBufferVolumeLayer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _partID: i32,
    pub _heightMapWidth: i32,
    pub _heightMapHeight: i32,
    pub _strength: f32,
    pub _diffuseTexturePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _maskTexturePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _metallic: f32,
    pub _normalTexturePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _normalScale: f32,
    pub _smoothness: f32,
    pub _specularColor: crate::UnityEngine::Color,
    pub _tileSize: crate::UnityEngine::Vector2,
    pub _tileOffset: crate::UnityEngine::Vector2,
    pub _uiExpanded: bool,
    pub _tile: i32,
    pub _normalizedHeights: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
    pub _minHeight: f32,
    pub _maxHeight: f32,
    pub _heightRange: f32,
    pub _terrainSizeX: f32,
    pub _terrainSizeY: f32,
    pub _position: crate::UnityEngine::Vector3,
    pub _minBounds: crate::UnityEngine::Vector3,
    pub _maxBounds: crate::UnityEngine::Vector3,
    pub _center: crate::UnityEngine::Vector3,
    pub _layerPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _hasLayerAttributes: bool,
    pub _layerType: crate::HoudiniEngineUnity::HFLayerType,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer =>
    "HoudiniEngineUnity"."HEU_LoadBufferVolumeLayer"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolumeLayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
