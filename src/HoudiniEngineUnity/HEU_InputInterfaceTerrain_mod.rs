#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputData,
    pub _heightFieldName: *mut crate::System::String,
    pub _parentNodeID: i32,
    pub _voxelSize: f32,
    pub _terrain: *mut crate::UnityEngine::Terrain,
    pub _terrainData: *mut crate::UnityEngine::TerrainData,
    pub _numPointsX: i32,
    pub _numPointsY: i32,
    pub _transform: crate::HoudiniEngineUnity::HAPI_Transform,
    pub _heightScale: f32,
    pub _heightfieldNodeID: i32,
    pub _heightNodeID: i32,
    pub _maskNodeID: i32,
    pub _mergeNodeID: i32,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain =>
    "HoudiniEngineUnity"."HEU_InputInterfaceTerrain/HEU_InputDataTerrain"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    type Target = crate::HoudiniEngineUnity::HEU_InputData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_InputInterfaceTerrain {
    __cordl_parent: crate::HoudiniEngineUnity::HEU_InputInterface,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_InputInterfaceTerrain =>
    "HoudiniEngineUnity"."HEU_InputInterfaceTerrain"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    type Target = crate::HoudiniEngineUnity::HEU_InputInterface;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    #[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain+HEU_InputDataTerrain")]
    pub type HEU_InputDataTerrain = crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain;
    pub fn CreateHeightFieldInputNode(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        idt: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateHeightFieldInputNode", (session, idt))?;
        Ok(__cordl_ret)
    }
    pub fn CreateInputNodeWithDataUpload(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        connectNodeID: i32,
        inputObject: *mut crate::UnityEngine::GameObject,
        inputNodeID: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateInputNodeWithDataUpload",
                (session, connectNodeID, inputObject, inputNodeID),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTerrainDataFromGameObject(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain = __cordl_object
            .invoke("GenerateTerrainDataFromGameObject", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn IsThisInputObjectSupported(
        &mut self,
        inputObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsThisInputObjectSupported", (inputObject))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetHeightFieldData(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        volumeNodeID: i32,
        partID: i32,
        heightValues: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        heightFieldName: *mut crate::System::String,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetHeightFieldData",
                (
                    session,
                    volumeNodeID,
                    partID,
                    heightValues,
                    heightFieldName,
                    baseVolumeInfo,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetMaskLayer(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        idt: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetMaskLayer", (session, idt, baseVolumeInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SetTerrainDataAttributesToHeightField(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoNodeID: i32,
        partID: i32,
        terrainData: *mut crate::UnityEngine::TerrainData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetTerrainDataAttributesToHeightField",
                (session, geoNodeID, partID, terrainData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetTerrainLayerAttributesToHeightField(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoNodeID: i32,
        partID: i32,
        terrainLayer: *mut crate::UnityEngine::TerrainLayer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetTerrainLayerAttributesToHeightField",
                (session, geoNodeID, partID, terrainLayer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetTreeInstances(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoNodeID: i32,
        partID: i32,
        terrainData: *mut crate::UnityEngine::TerrainData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTreeInstances", (session, geoNodeID, partID, terrainData))?;
        Ok(__cordl_ret)
    }
    pub fn SetTreePrototypes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoNodeID: i32,
        partID: i32,
        terrainData: *mut crate::UnityEngine::TerrainData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTreePrototypes", (session, geoNodeID, partID, terrainData))?;
        Ok(__cordl_ret)
    }
    pub fn UploadAlphaMaps(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        idt: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        baseVolumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
        bMaskSet: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadAlphaMaps", (session, idt, baseVolumeInfo, bMaskSet))?;
        Ok(__cordl_ret)
    }
    pub fn UploadHeightValuesWithTransform(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        idt: *mut crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain_HEU_InputDataTerrain,
        volumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("UploadHeightValuesWithTransform", (session, idt, volumeInfo))?;
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
#[cfg(feature = "HoudiniEngineUnity+HEU_InputInterfaceTerrain")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_InputInterfaceTerrain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
