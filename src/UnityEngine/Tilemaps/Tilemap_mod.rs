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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DoPositionsChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoPositionsChangedCallback", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (count, positionsIntPtr))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DoSyncTileCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoSyncTileCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (syncTiles))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSyncTileCallbackSettings(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    crate::UnityEngine::Tilemaps::Tilemap_SyncTileCallbackSettings,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("GetSyncTileCallbackSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSyncTileCallbackSettings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (settings))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::UnityEngine::Vector3Int), T, 1usize>("GetTile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTile", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked(self, (position)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAsset(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3Int),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                1usize,
            >("GetTileAsset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTileAsset", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = unsafe {
            method.invoke_unchecked(self, (position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTileAsset_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                1usize,
            >("GetTileAsset_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTileAsset_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object> = unsafe {
            method.invoke_unchecked(self, (position))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3Int, crate::UnityEngine::Vector3Int),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >,
                2usize,
            >("GetTileAssetsBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTileAssetsBlock", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = unsafe { method.invoke_unchecked(self, (position, blockDimensions)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >,
                2usize,
            >("GetTileAssetsBlock_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTileAssetsBlock_Injected", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        > = unsafe { method.invoke_unchecked(self, (position, blockDimensions)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::BoundsInt),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
                    >,
                >,
                1usize,
            >("GetTilesBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTilesBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
            >,
        > = unsafe { method.invoke_unchecked(self, (bounds)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetUsedTilesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetUsedTilesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUsedTilesCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Tilemaps::TileBase>,
                    >,
                >),
                i32,
                1usize,
            >("GetUsedTilesNonAlloc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUsedTilesNonAlloc", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (usedTiles)) };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePositionsChangedCallback(
        &mut self,
        count: i32,
        positionsIntPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandlePositionsChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandlePositionsChangedCallback", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (count, positionsIntPtr))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleSyncTileCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleSyncTileCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (syncTiles))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasPositionsChangedCallback() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("HasPositionsChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasPositionsChangedCallback", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn HasSyncTileCallback() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("HasSyncTileCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasSyncTileCallback", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn HasTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::UnityEngine::Vector3Int), bool, 1usize>("HasTile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasTile", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (position)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                    >,
                >),
                i32,
                1usize,
            >("Internal_GetUsedTilesNonAlloc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Internal_GetUsedTilesNonAlloc", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (usedTiles)) };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTile(
        &mut self,
        position: crate::UnityEngine::Vector3Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3Int),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RefreshTile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshTile", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTile_Injected(
        &mut self,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RefreshTile_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshTile_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshTilesNative(
        &mut self,
        positions: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RefreshTilesNative")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshTilesNative", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (positions, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendTilemapPositionsChangedCallback(
        &mut self,
        positions: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Vector3Int,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Collections::NativeArray_1<
                    crate::UnityEngine::Vector3Int,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SendTilemapPositionsChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendTilemapPositionsChangedCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (positions))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        crate::UnityEngine::Tilemaps::Tilemap_SyncTile,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SendTilemapTileChangedCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendTilemapTileChangedCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (syncTiles))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bufferSyncTile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_bufferSyncTile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_bufferSyncTile", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_cellBounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::BoundsInt> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::BoundsInt, 0usize>("get_cellBounds")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cellBounds", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::BoundsInt = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_layoutGrid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Grid>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Grid>,
                0usize,
            >("get_layoutGrid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_layoutGrid", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Grid> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Matrix4x4,
                0usize,
            >("get_orientationMatrix")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_orientationMatrix", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationMatrix_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_orientationMatrix_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_orientationMatrix_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_origin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3Int, 0usize>("get_origin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_origin", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3Int = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_origin_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_origin_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_origin_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_size(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3Int> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3Int, 0usize>("get_size")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_size", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3Int = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_size_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3Int>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_size_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_size_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tileAnchor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_tileAnchor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_tileAnchor", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_tileAnchor_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("get_tileAnchor_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_tileAnchor_Injected", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ret))
        };
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
    const CLASS_NAME: &'static str = "Tilemap/SyncTile";
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
    const CLASS_NAME: &'static str = "Tilemap/SyncTileCallbackSettings";
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
