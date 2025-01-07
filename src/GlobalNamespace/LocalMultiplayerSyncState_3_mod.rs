#[cfg(feature = "LocalMultiplayerSyncState_3")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalMultiplayerSyncState_3<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::MultiplayerSyncState_3<
        TStateTable,
        TType,
        TState,
    >,
    pub _player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
    pub _stateBuffer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState>,
    >,
    __cordl_phantom_TStateTable: std::marker::PhantomData<TStateTable>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TState: std::marker::PhantomData<TState>,
}
#[cfg(feature = "LocalMultiplayerSyncState_3")]
unsafe impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LocalMultiplayerSyncState_3<TStateTable, TType, TState> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalMultiplayerSyncState`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "",
                        "LocalMultiplayerSyncState`3",
                    )
                    .unwrap()
                    .make_generic::<(TStateTable, TType, TState)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "LocalMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::LocalMultiplayerSyncState_3<TStateTable, TType, TState> {
    type Target = crate::GlobalNamespace::MultiplayerSyncState_3<
        TStateTable,
        TType,
        TState,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::LocalMultiplayerSyncState_3<TStateTable, TType, TState> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::LocalMultiplayerSyncState_3<TStateTable, TType, TState> {
    pub fn GetState(
        &mut self,
        _cordl_type: TType,
    ) -> quest_hook::libil2cpp::Result<TState>
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
        let __cordl_ret: TState = __cordl_object.invoke("GetState", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        fullStateUpdateFrequency: i64,
        deltaUpdateFrequency: i64,
        _cordl_size: i32,
        interpolator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3_InterpolationDelegate<
                TStateTable,
                TType,
                TState,
            >,
        >,
        smoother: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3_SmoothingDelegate<
                TStateTable,
                TType,
                TState,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TStateTable: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TState: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    player,
                    fullStateUpdateFrequency,
                    deltaUpdateFrequency,
                    _cordl_size,
                    interpolator,
                    smoother,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetCurrentTime(
        &mut self,
        _cordl_time: i64,
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
            .invoke("SetCurrentTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDirty(
        &mut self,
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
            .invoke("SetDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetState(
        &mut self,
        _cordl_type: TType,
        state: TState,
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
            .invoke("SetState", (_cordl_type, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSerializedState<T>(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPacketPool_1<T>>,
        serializable: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSerializedState", (pool, serializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSerializedStateDelta<T>(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPacketPool_1<T>>,
        serializable: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSerializedStateDelta", (pool, serializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
        fullStateUpdateFrequency: i64,
        deltaUpdateFrequency: i64,
        _cordl_size: i32,
        interpolator: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3_InterpolationDelegate<
                TStateTable,
                TType,
                TState,
            >,
        >,
        smoother: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3_SmoothingDelegate<
                TStateTable,
                TType,
                TState,
            >,
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
            .invoke(
                ".ctor",
                (
                    player,
                    fullStateUpdateFrequency,
                    deltaUpdateFrequency,
                    _cordl_size,
                    interpolator,
                    smoother,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, t: TType) -> quest_hook::libil2cpp::Result<TState>
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
        let __cordl_ret: TState = __cordl_object.invoke("get_Item", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_player(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnectedPlayer>,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_player", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stateBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>,
        >,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>,
        > = __cordl_object.invoke("get_stateBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        t: TType,
        value: TState,
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
            .invoke("set_Item", (t, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalMultiplayerSyncState_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalMultiplayerSyncState_3<TStateTable, TType, TState> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
