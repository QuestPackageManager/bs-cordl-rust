#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_patchSize: crate::UnityEngine::Vector3,
    pub m_errorCode: crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
    pub m_terrainTiles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::TerrainUtils::TerrainTileCoord,
            *mut crate::UnityEngine::Terrain,
        >,
    >,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainMap =>
    "UnityEngine.TerrainUtils"."TerrainMap"
);
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl std::ops::DerefMut for crate::UnityEngine::TerrainUtils::TerrainMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl crate::UnityEngine::TerrainUtils::TerrainMap {
    pub fn AddTerrainInternal(
        &mut self,
        x: i32,
        z: i32,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTerrainInternal", (x, z, terrain))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromPlacement_Terrain_Predicate_1__cordl_bool0(
        originTerrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Predicate_1<*mut crate::UnityEngine::Terrain>,
        >,
        fullValidation: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TerrainUtils::TerrainMap,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromPlacement", (originTerrain, filter, fullValidation))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromPlacement_Vector2_Vector2_Predicate_1__cordl_bool1(
        gridOrigin: crate::UnityEngine::Vector2,
        gridSize: crate::UnityEngine::Vector2,
        filter: quest_hook::libil2cpp::Gc<
            crate::System::Predicate_1<*mut crate::UnityEngine::Terrain>,
        >,
        fullValidation: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TerrainUtils::TerrainMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TerrainUtils::TerrainMap,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateFromPlacement",
                (gridOrigin, gridSize, filter, fullValidation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain> = __cordl_object
            .invoke("GetTerrain", (tileX, tileZ))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryToAddTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
        terrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Terrain>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryToAddTerrain", (tileX, tileZ, terrain))?;
        Ok(__cordl_ret.into())
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::TerrainUtils::TerrainMapStatusCode = __cordl_object
            .invoke("Validate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateTerrain", (tileX, tileZ))?;
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
    pub fn get_terrainTiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::TerrainUtils::TerrainTileCoord,
                *mut crate::UnityEngine::Terrain,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::TerrainUtils::TerrainTileCoord,
                *mut crate::UnityEngine::Terrain,
            >,
        > = __cordl_object.invoke("get_terrainTiles", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TerrainUtils::TerrainMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
