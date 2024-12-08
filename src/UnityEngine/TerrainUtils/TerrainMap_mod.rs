#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
#[repr(C)]
#[derive(Debug)]
pub struct TerrainMap {
    __cordl_parent: crate::System::Object,
    pub m_patchSize: crate::UnityEngine::Vector3,
    pub m_errorCode: crate::UnityEngine::TerrainUtils::TerrainMapStatusCode,
    pub m_terrainTiles: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::TerrainUtils::TerrainTileCoord,
        *mut crate::UnityEngine::Terrain,
    >,
}
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TerrainUtils::TerrainMap =>
    "UnityEngine.TerrainUtils"."TerrainMap"
);
#[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap")]
impl std::ops::Deref for crate::UnityEngine::TerrainUtils::TerrainMap {
    type Target = crate::System::Object;
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
    #[cfg(feature = "UnityEngine+TerrainUtils+TerrainMap+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::UnityEngine::TerrainUtils::TerrainMap___c__DisplayClass3_0;
    pub fn AddTerrainInternal(
        &mut self,
        x: i32,
        z: i32,
        terrain: *mut crate::UnityEngine::Terrain,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTerrainInternal", (x, z, terrain))?;
        Ok(__cordl_ret)
    }
    pub fn GetTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Terrain> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Terrain = __cordl_object
            .invoke("GetTerrain", (tileX, tileZ))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryToAddTerrain(
        &mut self,
        tileX: i32,
        tileZ: i32,
        terrain: *mut crate::UnityEngine::Terrain,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryToAddTerrain", (tileX, tileZ, terrain))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
    pub fn get_terrainTiles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::TerrainUtils::TerrainTileCoord,
            *mut crate::UnityEngine::Terrain,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::TerrainUtils::TerrainTileCoord,
            *mut crate::UnityEngine::Terrain,
        > = __cordl_object.invoke("get_terrainTiles", ())?;
        Ok(__cordl_ret)
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