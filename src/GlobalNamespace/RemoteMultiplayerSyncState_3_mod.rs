#[cfg(feature = "RemoteMultiplayerSyncState_3")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteMultiplayerSyncState_3<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::MultiplayerSyncState_3<
        TStateTable,
        TType,
        TState,
    >,
    pub _player: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _stateBuffer: *mut crate::GlobalNamespace::RemoteStateBuffer_3<
        TStateTable,
        TType,
        TState,
    >,
    __cordl_phantom_TStateTable: std::marker::PhantomData<TStateTable>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TState: std::marker::PhantomData<TState>,
}
#[cfg(feature = "RemoteMultiplayerSyncState_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RemoteMultiplayerSyncState_3 <
    TStateTable, TType, TState > => ""."RemoteMultiplayerSyncState`3" < TStateTable,
    TType, TState >
);
#[cfg(feature = "RemoteMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::RemoteMultiplayerSyncState_3<TStateTable, TType, TState> {
    type Target = crate::GlobalNamespace::MultiplayerSyncState_3<
        TStateTable,
        TType,
        TState,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RemoteMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::RemoteMultiplayerSyncState_3<TStateTable, TType, TState> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RemoteMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::RemoteMultiplayerSyncState_3<TStateTable, TType, TState> {
    pub fn New(
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
        _cordl_size: i32,
        interpolator: *mut crate::GlobalNamespace::StateBuffer_3_InterpolationDelegate<
            TStateTable,
            TType,
            TState,
        >,
        smoother: *mut crate::GlobalNamespace::StateBuffer_3_SmoothingDelegate<
            TStateTable,
            TType,
            TState,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (player, _cordl_size, interpolator, smoother))?;
        Ok(__cordl_object)
    }
    pub fn UpdateDelta<T>(
        &mut self,
        serializable: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDelta", (serializable))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateState<T>(
        &mut self,
        serializable: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateState", (serializable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
        _cordl_size: i32,
        interpolator: *mut crate::GlobalNamespace::StateBuffer_3_InterpolationDelegate<
            TStateTable,
            TType,
            TState,
        >,
        smoother: *mut crate::GlobalNamespace::StateBuffer_3_SmoothingDelegate<
            TStateTable,
            TType,
            TState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (player, _cordl_size, interpolator, smoother))?;
        Ok(__cordl_ret)
    }
    pub fn get_player(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::IConnectedPlayer>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IConnectedPlayer = __cordl_object
            .invoke("get_player", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_stateBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>,
    >
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::StateBuffer_3<
            TStateTable,
            TType,
            TState,
        > = __cordl_object.invoke("get_stateBuffer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RemoteMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RemoteMultiplayerSyncState_3<TStateTable, TType, TState> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
