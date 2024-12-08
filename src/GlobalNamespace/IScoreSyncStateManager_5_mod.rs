#[cfg(feature = "IScoreSyncStateManager_5")]
#[repr(C)]
#[derive(Debug)]
pub struct IScoreSyncStateManager_5<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
    TSerializable: quest_hook::libil2cpp::Type,
    TDeltaSerializable: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TStateTable: std::marker::PhantomData<TStateTable>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TState: std::marker::PhantomData<TState>,
    __cordl_phantom_TSerializable: std::marker::PhantomData<TSerializable>,
    __cordl_phantom_TDeltaSerializable: std::marker::PhantomData<TDeltaSerializable>,
}
#[cfg(feature = "IScoreSyncStateManager_5")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IScoreSyncStateManager_5 <
    TStateTable, TType, TState, TSerializable, TDeltaSerializable > => ""
    ."IScoreSyncStateManager`5" < TStateTable, TType, TState, TSerializable,
    TDeltaSerializable >
);
#[cfg(feature = "IScoreSyncStateManager_5")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
    TSerializable: quest_hook::libil2cpp::Type,
    TDeltaSerializable: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::IScoreSyncStateManager_5<
    TStateTable,
    TType,
    TState,
    TSerializable,
    TDeltaSerializable,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager_5")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
    TSerializable: quest_hook::libil2cpp::Type,
    TDeltaSerializable: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::IScoreSyncStateManager_5<
    TStateTable,
    TType,
    TState,
    TSerializable,
    TDeltaSerializable,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IScoreSyncStateManager_5")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
    TSerializable: quest_hook::libil2cpp::Type,
    TDeltaSerializable: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::IScoreSyncStateManager_5<
    TStateTable,
    TType,
    TState,
    TSerializable,
    TDeltaSerializable,
> {
    pub fn GetSyncState(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerSyncState_3<TStateTable, TType, TState>,
    >
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDeltaSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerSyncState_3<
            TStateTable,
            TType,
            TState,
        > = __cordl_object.invoke("GetSyncState", (i))?;
        Ok(__cordl_ret)
    }
    pub fn GetSyncStateForPlayer(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerSyncState_3<TStateTable, TType, TState>,
    >
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDeltaSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerSyncState_3<
            TStateTable,
            TType,
            TState,
        > = __cordl_object.invoke("GetSyncStateForPlayer", (player))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_connectedPlayerCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDeltaSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_connectedPlayerCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::LocalMultiplayerSyncState_3<
            TStateTable,
            TType,
            TState,
        >,
    >
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDeltaSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::LocalMultiplayerSyncState_3<
            TStateTable,
            TType,
            TState,
        > = __cordl_object.invoke("get_localState", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDeltaSerializable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "IScoreSyncStateManager_5")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
    TSerializable: quest_hook::libil2cpp::Type,
    TDeltaSerializable: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IScoreSyncStateManager_5<
    TStateTable,
    TType,
    TState,
    TSerializable,
    TDeltaSerializable,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
