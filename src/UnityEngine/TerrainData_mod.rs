#[cfg(feature = "UnityEngine+TerrainData")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainData {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+TerrainData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainData => "UnityEngine"
    ."TerrainData"
);
#[cfg(feature = "UnityEngine+TerrainData")]
impl std::ops::Deref for crate::UnityEngine::TerrainData {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainData")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainData")]
impl crate::UnityEngine::TerrainData {
    pub const k_DetailDatabasePrefix: &'static str = "GetDetailDatabase().";
    pub const k_HeightmapPrefix: &'static str = "GetHeightmap().";
    pub const k_ScriptingInterfaceName: &'static str = "TerrainDataScriptingInterface";
    pub const k_ScriptingInterfacePrefix: &'static str = "TerrainDataScriptingInterface::";
    pub const k_SplatDatabasePrefix: &'static str = "GetSplatDatabase().";
    pub const k_TreeDatabasePrefix: &'static str = "GetTreeDatabase().";
    #[cfg(feature = "UnityEngine+TerrainData+BoundaryValueType")]
    pub type BoundaryValueType = crate::UnityEngine::TerrainData_BoundaryValueType;
    pub fn GetAlphamapResolutionInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetAlphamapResolutionInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAlphamaps(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetAlphamaps", (x, y, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoundaryValue(
        _cordl_type: crate::UnityEngine::TerrainData_BoundaryValueType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBoundaryValue", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHeights(
        &mut self,
        xBase: i32,
        yBase: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetHeights", (xBase, yBase, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        terrainData: quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Create", (terrainData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetAlphamaps(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Internal_GetAlphamaps", (x, y, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetHeights(
        &mut self,
        xBase: i32,
        yBase: i32,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("Internal_GetHeights", (xBase, yBase, width, height))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetTreeInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TreeInstance>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TreeInstance>,
        > = __cordl_object.invoke("Internal_GetTreeInstances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetAlphamaps(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        map: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_SetAlphamaps", (x, y, width, height, map))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetDetailLayer(
        &mut self,
        xBase: i32,
        yBase: i32,
        totalWidth: i32,
        totalHeight: i32,
        detailIndex: i32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_SetDetailLayer",
                (xBase, yBase, totalWidth, totalHeight, detailIndex, data),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetDetailResolution(
        &mut self,
        patchCount: i32,
        resolutionPerPatch: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_SetDetailResolution", (patchCount, resolutionPerPatch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetHeights(
        &mut self,
        xBase: i32,
        yBase: i32,
        width: i32,
        height: i32,
        heights: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_SetHeights", (xBase, yBase, width, height, heights))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshPrototypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshPrototypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAlphamaps(
        &mut self,
        x: i32,
        y: i32,
        map: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAlphamaps", (x, y, map))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDetailLayer(
        &mut self,
        xBase: i32,
        yBase: i32,
        layer: i32,
        details: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDetailLayer", (xBase, yBase, layer, details))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDetailResolution(
        &mut self,
        detailResolution: i32,
        resolutionPerPatch: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDetailResolution", (detailResolution, resolutionPerPatch))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHeights(
        &mut self,
        xBase: i32,
        yBase: i32,
        heights: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHeights", (xBase, yBase, heights))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTreeInstances(
        &mut self,
        instances: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TreeInstance>,
        >,
        snapToHeightmap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTreeInstances", (instances, snapToHeightmap))?;
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
    pub fn get_Internal_alphamapResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_Internal_alphamapResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alphamapHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_alphamapHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alphamapLayers(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_alphamapLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alphamapResolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_alphamapResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alphamapWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_alphamapWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_detailResolutionPerPatch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_detailResolutionPerPatch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_heightmapResolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_heightmapResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_internalHeightmapResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_internalHeightmapResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_size", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_size_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_terrainLayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TerrainLayer>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TerrainLayer>,
        > = __cordl_object.invoke("get_terrainLayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_treeInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TreeInstance>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TreeInstance>,
        > = __cordl_object.invoke("get_treeInstances", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_treePrototypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TreePrototype>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TreePrototype>,
        > = __cordl_object.invoke("get_treePrototypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_users(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Terrain>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Terrain>,
        > = __cordl_object.invoke("get_users", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Internal_alphamapResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Internal_alphamapResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_alphamapResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alphamapResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_detailPrototypes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::DetailPrototype>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_detailPrototypes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_heightmapResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_heightmapResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_internalHeightmapResolution(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_internalHeightmapResolution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_size_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_size_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_terrainLayers(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TerrainLayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_terrainLayers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_treePrototypes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::TreePrototype>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_treePrototypes", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TerrainData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+TerrainData+BoundaryValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainData_BoundaryValueType {
    MaxAlphamapRes = 6i32,
    MaxBaseMapRes = 8i32,
    MaxCoveragePerRes = 4i32,
    MaxDetailPatchCount = 3i32,
    MaxDetailResPerPatch = 2i32,
    MaxHeightmapRes = 0i32,
    MinAlphamapRes = 5i32,
    MinBaseMapRes = 7i32,
    MinDetailResPerPatch = 1i32,
}
#[cfg(feature = "UnityEngine+TerrainData+BoundaryValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainData_BoundaryValueType =>
    "UnityEngine"."TerrainData/BoundaryValueType"
);
