#[cfg(feature = "LocalStateBuffer_3")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalStateBuffer_3<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>,
    pub _current: crate::GlobalNamespace::StateBuffer_3_TimestampedStateTable<
        TStateTable,
        TType,
        TState,
    >,
    pub _fullStateUpdateFrequency: i64,
    pub _deltaUpdateFrequency: i64,
    pub _sentStateIndex: i32,
    pub _sentDeltaIndex: i32,
    pub _sentStates: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::StateBuffer_3_TimestampedStateTable<
            TStateTable,
            TType,
            TState,
        >,
    >,
    pub _sentDeltas: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::StateBuffer_3_TimestampedStateTable<
            TStateTable,
            TType,
            TState,
        >,
    >,
    pub _lastSentSyncStateId: crate::GlobalNamespace::SyncStateId,
    __cordl_phantom_TStateTable: std::marker::PhantomData<TStateTable>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TState: std::marker::PhantomData<TState>,
}
#[cfg(feature = "LocalStateBuffer_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalStateBuffer_3 <
    TStateTable, TType, TState > => ""."LocalStateBuffer`3" < TStateTable, TType, TState
    >
);
#[cfg(feature = "LocalStateBuffer_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    type Target = crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalStateBuffer_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalStateBuffer_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    pub const kMaxSentDeltas: i32 = 4i32;
    pub const kMaxSentStates: i32 = 4i32;
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn MarkDirty(
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
            .invoke("MarkDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
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
                    fullStateUpdateFrequency,
                    deltaUpdateFrequency,
                    _cordl_size,
                    interpolator,
                    smoother,
                ),
            )?;
        Ok(__cordl_object.into())
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
    pub fn SetTime(
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
            .invoke("SetTime", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSerializedState(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<TStateTable>,
        _cordl_time: quest_hook::libil2cpp::ByRefMut<i64>,
        id: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::SyncStateId>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSerializedState", (state, _cordl_time, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSerializedStateDelta(
        &mut self,
        delta: quest_hook::libil2cpp::ByRefMut<TStateTable>,
        timeOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        baseId: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::SyncStateId>,
    ) -> quest_hook::libil2cpp::Result<bool>
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
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSerializedStateDelta", (delta, timeOffset, baseId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
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
                    fullStateUpdateFrequency,
                    deltaUpdateFrequency,
                    _cordl_size,
                    interpolator,
                    smoother,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalStateBuffer_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
