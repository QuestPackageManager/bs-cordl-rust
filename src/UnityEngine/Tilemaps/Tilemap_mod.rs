#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
#[repr(C)]
#[derive(Debug)]
pub struct Tilemap {
    __cordl_parent: crate::UnityEngine::GridLayout,
    pub m_BufferSyncTile: bool,
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Tilemaps::Tilemap {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Tilemaps";
    const CLASS_NAME: &'static str = "Tilemap";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
impl std::ops::Deref for crate::UnityEngine::Tilemaps::Tilemap {
    type Target = crate::UnityEngine::GridLayout;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
impl std::ops::DerefMut for crate::UnityEngine::Tilemaps::Tilemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
impl crate::UnityEngine::Tilemaps::Tilemap {
    #[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
    pub type SyncTile = crate::UnityEngine::Tilemaps::Tilemap_SyncTile;
    #[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
    pub type SyncTileCallbackSettings = crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings;
    pub fn DoPositionsChangedCallback(
        &mut self,
        count: i32,
        positionsIntPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoPositionsChangedCallback", (count, positionsIntPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoSyncTileCallback(
        &mut self,
        syncTiles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoSyncTileCallback", (syncTiles))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSyncTileCallbackSettings(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSyncTileCallbackSettings", (settings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTile<T>(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetTile", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAsset(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("GetTileAsset", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAsset_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = __cordl_object
            .invoke("GetTileAsset_Injected", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAssetsBlock(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
        blockDimensions: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = __cordl_object.invoke("GetTileAssetsBlock", (position, blockDimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAssetsBlock_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
        blockDimensions: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = __cordl_object
            .invoke("GetTileAssetsBlock_Injected", (position, blockDimensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTilesBlock(
        &mut self,
        bounds: crate::UnityEngine::BoundsInt,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
            >,
        > = __cordl_object.invoke("GetTilesBlock", (bounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUsedTilesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetUsedTilesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUsedTilesNonAlloc(
        &mut self,
        usedTiles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetUsedTilesNonAlloc", (usedTiles))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePositionsChangedCallback(
        &mut self,
        count: i32,
        positionsIntPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePositionsChangedCallback", (count, positionsIntPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSyncTileCallback(
        &mut self,
        syncTiles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSyncTileCallback", (syncTiles))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasPositionsChangedCallback() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasPositionsChangedCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSyncTileCallback() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasSyncTileCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTile", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetUsedTilesNonAlloc(
        &mut self,
        usedTiles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Internal_GetUsedTilesNonAlloc", (usedTiles))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTile", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTile_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTile_Injected", (position))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTilesNative(
        &mut self,
        positions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshTilesNative", (positions, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTilemapPositionsChangedCallback(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Vector3Int,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendTilemapPositionsChangedCallback", (positions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTilemapTileChangedCallback(
        &mut self,
        syncTiles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendTilemapTileChangedCallback", (syncTiles))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bufferSyncTile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_bufferSyncTile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cellBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::BoundsInt> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::BoundsInt = __cordl_object
            .invoke("get_cellBounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutGrid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Grid>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Grid> = __cordl_object
            .invoke("get_layoutGrid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_orientationMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_orientationMatrix_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_origin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3Int = __cordl_object
            .invoke("get_origin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_origin_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_origin_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3Int = __cordl_object
            .invoke("get_size", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_size_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tileAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_tileAnchor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tileAnchor_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_tileAnchor_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Tilemaps::Tilemap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tilemap_SyncTile {
    pub m_Position: crate::UnityEngine::Vector3Int,
    pub m_Tile: quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
    pub m_TileData: crate::UnityEngine::Tilemaps::TileData,
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Tilemaps";
    const CLASS_NAME: &'static str = "SyncTile";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::Tilemap_SyncTile {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTile")]
impl crate::UnityEngine::Tilemaps::Tilemap_SyncTile {}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tilemap_SyncTileCallbackSettings {
    pub hasSyncTileCallback: bool,
    pub hasPositionsChangedCallback: bool,
    pub isBufferSyncTile: bool,
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Tilemaps";
    const CLASS_NAME: &'static str = "SyncTileCallbackSettings";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Tilemaps+Tilemap+SyncTileCallbackSettings")]
impl crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings {}
