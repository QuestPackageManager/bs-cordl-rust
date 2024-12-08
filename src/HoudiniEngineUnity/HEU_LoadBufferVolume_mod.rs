#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_LoadBufferVolume {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_LoadBufferBase,
    pub _tileIndex: i32,
    pub _splatLayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_LoadBufferVolumeLayer,
    >,
    pub _heightMapWidth: i32,
    pub _heightMapHeight: i32,
    pub _heightMap: *mut crate::System::Object,
    pub _splatMaps: *mut crate::System::Object,
    pub _terrainSizeX: f32,
    pub _terrainSizeY: f32,
    pub _heightRange: f32,
    pub _position: crate::UnityEngine::Vector3,
    pub _terrainDataPath: *mut crate::System::String,
    pub _terrainDataExportPath: *mut crate::System::String,
    pub _scatterTrees: *mut crate::HoudiniEngineUnity::HEU_VolumeScatterTrees,
    pub _detailPrototypes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_DetailPrototype,
    >,
    pub _detailMaps: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Object,
    >,
    pub _detailProperties: *mut crate::HoudiniEngineUnity::HEU_DetailProperties,
    pub _specifiedTerrainMaterialName: *mut crate::System::String,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_LoadBufferVolume =>
    "HoudiniEngineUnity"."HEU_LoadBufferVolume"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_LoadBufferVolume {
    type Target = crate::HoudiniEngineUnity::HEU_LoadBufferBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_LoadBufferVolume {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
impl crate::HoudiniEngineUnity::HEU_LoadBufferVolume {
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_LoadBufferVolume")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_LoadBufferVolume {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}