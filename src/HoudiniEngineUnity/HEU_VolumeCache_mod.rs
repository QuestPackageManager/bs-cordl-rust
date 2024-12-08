#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_VolumeCache {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _ownerNode: *mut crate::HoudiniEngineUnity::HEU_GeoNode,
    pub _layers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_VolumeLayer,
    >,
    pub _updatedLayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HoudiniEngineUnity::HEU_VolumeLayer,
    >,
    pub _tileIndex: i32,
    pub _isDirty: bool,
    pub _geoName: *mut crate::System::String,
    pub _objName: *mut crate::System::String,
    pub _uiExpanded: bool,
    pub _terrainData: *mut crate::UnityEngine::TerrainData,
    pub _scatterTrees: *mut crate::HoudiniEngineUnity::HEU_VolumeScatterTrees,
    pub _detailProperties: *mut crate::HoudiniEngineUnity::HEU_DetailProperties,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_VolumeCache =>
    "HoudiniEngineUnity"."HEU_VolumeCache"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_VolumeCache {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_VolumeCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
impl crate::HoudiniEngineUnity::HEU_VolumeCache {
    pub fn LoadLayerTextureFromAttribute(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        attrName: *mut crate::System::String,
        outTexture: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadLayerTextureFromAttribute",
                (session, geoID, partID, attrName, outTexture),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadLayerVector2FromAttribute(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        attrName: *mut crate::System::String,
        vectorValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadLayerVector2FromAttribute",
                (session, geoID, partID, attrName, vectorValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PopulateDetailPrototype(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        layer: *mut crate::HoudiniEngineUnity::HEU_VolumeLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateDetailPrototype", (session, geoID, partID, layer))?;
        Ok(__cordl_ret)
    }
    pub fn CopyValuesTo(
        &mut self,
        destCache: *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyValuesTo", (destCache))?;
        Ok(__cordl_ret)
    }
    pub fn IsEquivalentTo(
        &mut self,
        other: *mut crate::HoudiniEngineUnity::HEU_VolumeCache,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEquivalentTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsDirty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn LoadLayerFloatFromAttribute(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        attrName: *mut crate::System::String,
        floatValue: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadLayerFloatFromAttribute",
                (session, geoID, partID, attrName, floatValue),
            )?;
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
    pub fn FinishUpdateLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishUpdateLayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulateScatterTrees(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        pointCount: i32,
        throwWarningIfNoTileAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopulateScatterTrees",
                (session, geoID, partID, pointCount, throwWarningIfNoTileAttribute),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadLayerPropertiesFromAttributes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        terrainLayer: *mut crate::UnityEngine::TerrainLayer,
        bNewTerrainLayer: bool,
        defaultTexture: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadLayerPropertiesFromAttributes",
                (session, geoID, partID, terrainLayer, bNewTerrainLayer, defaultTexture),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_TileIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TileIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTerrainWithAlphamaps(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        houdiniAsset: *mut crate::HoudiniEngineUnity::HEU_HoudiniAsset,
        bRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateTerrainWithAlphamaps", (session, houdiniAsset, bRebuild))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyPreset(
        &mut self,
        volumeCachePreset: *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ApplyPreset", (volumeCachePreset))?;
        Ok(__cordl_ret)
    }
    pub fn StartUpdateLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartUpdateLayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_UIExpanded(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_UIExpanded", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLayerFromPart(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        part: *mut crate::HoudiniEngineUnity::HEU_PartData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLayerFromPart", (session, part))?;
        Ok(__cordl_ret)
    }
    pub fn ResetParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UIExpanded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UIExpanded", ())?;
        Ok(__cordl_ret)
    }
    pub fn PopulatePreset(
        &mut self,
        cachePreset: *mut crate::HoudiniEngineUnity::HEU_VolumeCachePreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulatePreset", (cachePreset))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        ownerNode: *mut crate::HoudiniEngineUnity::HEU_GeoNode,
        tileIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (ownerNode, tileIndex))?;
        Ok(__cordl_ret)
    }
    pub fn LoadLayerColorFromAttribute(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        attrName: *mut crate::System::String,
        colorValue: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "LoadLayerColorFromAttribute",
                (session, geoID, partID, attrName, colorValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetPartLayerAttributes(
        &mut self,
        session: *mut crate::HoudiniEngineUnity::HEU_SessionBase,
        geoID: i32,
        partID: i32,
        layer: *mut crate::HoudiniEngineUnity::HEU_VolumeLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPartLayerAttributes", (session, geoID, partID, layer))?;
        Ok(__cordl_ret)
    }
    pub fn get_GeoName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_GeoName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLayer(
        &mut self,
        layerName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_VolumeLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_VolumeLayer = __cordl_object
            .invoke("GetLayer", (layerName))?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ObjectName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_VolumeCache")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_VolumeCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
