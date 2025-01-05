#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
#[repr(C)]
#[derive(Debug)]
pub struct TileBase {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Tilemaps::TileBase =>
    "UnityEngine.Tilemaps"."TileBase"
);
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl std::ops::Deref for crate::UnityEngine::Tilemaps::TileBase {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl std::ops::DerefMut for crate::UnityEngine::Tilemaps::TileBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl crate::UnityEngine::Tilemaps::TileBase {
    pub fn GetTileAnimationData(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileAnimationData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::TileAnimationData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetTileAnimationData", (position, tilemap, tileAnimationData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAnimationDataNoRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Tilemaps::TileAnimationData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Tilemaps::TileAnimationData = __cordl_object
            .invoke("GetTileAnimationDataNoRef", (position, tilemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAnimationDataRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileAnimationData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::TileAnimationData,
        >,
        hasAnimation: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTileAnimationDataRef",
                (position, tilemap, tileAnimationData, hasAnimation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileData(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        tileData: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Tilemaps::TileData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTileData", (position, tilemap, tileData))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileDataNoRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Tilemaps::TileData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Tilemaps::TileData = __cordl_object
            .invoke("GetTileDataNoRef", (position, tilemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTile", (position, tilemap))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartUp(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartUp", (position, tilemap, go))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartUpRef(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        tilemap: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::ITilemap>,
        go: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        startUpInvokedByUser: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartUpRef", (position, tilemap, go, startUpInvokedByUser))?;
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
#[cfg(feature = "UnityEngine+Tilemaps+TileBase")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Tilemaps::TileBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
