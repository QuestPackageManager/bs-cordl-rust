#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TerrainUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_TerrainUtility =>
    "HoudiniEngineUnity"."HEU_TerrainUtility"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TerrainUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TerrainUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
impl crate::HoudiniEngineUnity::HEU_TerrainUtility {
    pub fn AppendConvertedHeightFieldToAlphaMap(
        heightMapWidth: i32,
        heightMapHeight: i32,
        existingAlphaMaps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        heightFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppArray<f32>,
            >,
        >,
        strengths: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        alphaMapIndices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AppendConvertedHeightFieldToAlphaMap",
                (
                    heightMapWidth,
                    heightMapHeight,
                    existingAlphaMaps,
                    heightFields,
                    strengths,
                    alphaMapIndices,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyDetailLayers(
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
        detailProperties: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_DetailProperties,
        >,
        heuDetailPrototypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_DetailPrototype,
            >,
        >,
        convertedDetailMaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ApplyDetailLayers",
                (
                    terrain,
                    terrainData,
                    detailProperties,
                    heuDetailPrototypes,
                    convertedDetailMaps,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyScatterTrees(
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
        scatterTrees: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_VolumeScatterTrees,
        >,
        tileIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyScatterTrees", (terrainData, scatterTrees, tileIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertHeightFieldToAlphaMap(
        heightMapWidth: i32,
        heightMapHeight: i32,
        heightFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppArray<f32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertHeightFieldToAlphaMap",
                (heightMapWidth, heightMapHeight, heightFields),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertHeightMapHoudiniToUnity(
        heightMapWidth: i32,
        heightMapHeight: i32,
        heightValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertHeightMapHoudiniToUnity",
                (heightMapWidth, heightMapHeight, heightValues),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTerrainFromVolume(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        volumeInfo: quest_hook::libil2cpp::ByRefMut<
            crate::HoudiniEngineUnity::HAPI_VolumeInfo,
        >,
        geoID: i32,
        partID: i32,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        terrainData: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::TerrainData,
        >,
        volumePositionOffset: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector3,
        >,
        terrain: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Terrain>,
        bakedMaterialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GenerateTerrainFromVolume",
                (
                    session,
                    volumeInfo,
                    geoID,
                    partID,
                    gameObject,
                    terrainData,
                    volumePositionOffset,
                    terrain,
                    bakedMaterialPath,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultTerrainMaterialPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultTerrainMaterialPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultTerrainShaderName() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultTerrainShaderName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDetailMapFromPart(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        detailResolution: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDetailMapFromPart", (session, geoID, partID, detailResolution))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeightRangeFromHeightfield(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHeightRangeFromHeightfield", (session, geoID, partID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeightfieldLayerType(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        volumeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::HoudiniEngineUnity::HFLayerType> {
        let __cordl_ret: crate::HoudiniEngineUnity::HFLayerType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHeightfieldLayerType", (session, geoID, partID, volumeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeightmapFromPart(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        xLength: i32,
        yLength: i32,
        geoID: i32,
        partID: i32,
        heightValues: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        minHeight: quest_hook::libil2cpp::ByRefMut<f32>,
        maxHeight: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetHeightmapFromPart",
                (
                    session,
                    xLength,
                    yLength,
                    geoID,
                    partID,
                    heightValues,
                    minHeight,
                    maxHeight,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNormalizedHeightmapFromPartWithMinMax(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        heightMapWidth: i32,
        heightMapHeight: i32,
        minHeight: quest_hook::libil2cpp::ByRefMut<f32>,
        maxHeight: quest_hook::libil2cpp::ByRefMut<f32>,
        heightRange: quest_hook::libil2cpp::ByRefMut<f32>,
        bUseHeightRangeOverride: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNormalizedHeightmapFromPartWithMinMax",
                (
                    session,
                    geoID,
                    partID,
                    heightMapWidth,
                    heightMapHeight,
                    minHeight,
                    maxHeight,
                    heightRange,
                    bUseHeightRangeOverride,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTerrainDataExportPathFromHeightfieldAttribute(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetTerrainDataExportPathFromHeightfieldAttribute",
                (session, geoID, partID),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTerrainLayerIndex(
        layer: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainLayer>,
        terrainLayers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TerrainLayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTerrainLayerIndex", (layer, terrainLayers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTerrainLayerIndexByName(
        layerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        terrainLayers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TerrainLayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTerrainLayerIndexByName", (layerName, terrainLayers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTreePrototypeInfosFromPart(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_TreePrototypeInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::HoudiniEngineUnity::HEU_TreePrototypeInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTreePrototypeInfosFromPart", (session, geoID, partID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVolumePositionOffset(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        volumePosition: crate::UnityEngine::Vector3,
        terrainSizeX: f32,
        heightMapSize: f32,
        mapWidth: i32,
        mapHeight: i32,
        minHeight: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetVolumePositionOffset",
                (
                    session,
                    geoID,
                    partID,
                    volumePosition,
                    terrainSizeX,
                    heightMapSize,
                    mapWidth,
                    mapHeight,
                    minHeight,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateDetailProperties(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        detailProperties: quest_hook::libil2cpp::ByRefMut<
            *mut crate::HoudiniEngineUnity::HEU_DetailProperties,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PopulateDetailProperties",
                (session, geoID, partID, detailProperties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateDetailPrototype(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        detailPrototype: quest_hook::libil2cpp::ByRefMut<
            *mut crate::HoudiniEngineUnity::HEU_DetailPrototype,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PopulateDetailPrototype",
                (session, geoID, partID, detailPrototype),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateScatterTrees(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
        pointCount: i32,
        scatterTrees: quest_hook::libil2cpp::ByRefMut<
            *mut crate::HoudiniEngineUnity::HEU_VolumeScatterTrees,
        >,
        throwWarningIfNoTileAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PopulateScatterTrees",
                (
                    session,
                    geoID,
                    partID,
                    pointCount,
                    scatterTrees,
                    throwWarningIfNoTileAttribute,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResampleData(
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        oldWidth: i32,
        oldHeight: i32,
        newWidth: i32,
        newHeight: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResampleData", (data, oldWidth, oldHeight, newWidth, newHeight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTerrainMaterial(
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        specifiedMaterialName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bakedMaterialPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetTerrainMaterial",
                (terrain, specifiedMaterialName, bakedMaterialPath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn VolumeLayerHasAttributes(
        session: quest_hook::libil2cpp::Gc<crate::HoudiniEngineUnity::HEU_SessionBase>,
        geoID: i32,
        partID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VolumeLayerHasAttributes", (session, geoID, partID))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TerrainUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_TerrainUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
