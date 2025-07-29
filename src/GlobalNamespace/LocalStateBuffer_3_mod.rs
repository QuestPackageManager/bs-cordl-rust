#[cfg(feature = "cordl_class_LocalStateBuffer_3")]
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
    pub _sentStates: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::StateBuffer_3_TimestampedStateTable<
                TStateTable,
                TType,
                TState,
            >,
        >,
    >,
    pub _sentDeltas: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::StateBuffer_3_TimestampedStateTable<
                TStateTable,
                TType,
                TState,
            >,
        >,
    >,
    pub _lastSentSyncStateId: crate::GlobalNamespace::SyncStateId,
    __cordl_phantom_TStateTable: std::marker::PhantomData<TStateTable>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
    __cordl_phantom_TState: std::marker::PhantomData<TState>,
}
#[cfg(feature = "cordl_class_LocalStateBuffer_3")]
unsafe impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LocalStateBuffer`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("", "LocalStateBuffer`3")
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
#[cfg(feature = "LocalStateBuffer_3")]
impl<
    TStateTable: quest_hook::libil2cpp::Type,
    TType: quest_hook::libil2cpp::Type,
    TState: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::LocalStateBuffer_3<TStateTable, TType, TState> {
    type Target = crate::GlobalNamespace::StateBuffer_3<TStateTable, TType, TState>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
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
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TType), TState, 1usize>("GetState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetState", 1usize
                        )
                    })
            });
        let __cordl_ret: TState = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("MarkDirty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarkDirty", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TType, TState),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetState", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_type, state))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>("SetTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "SetTime",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_time))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TStateTable>,
                            quest_hook::libil2cpp::ByRefMut<i64>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::SyncStateId,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetSerializedState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSerializedState", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, _cordl_time, id))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TStateTable>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::SyncStateId,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetSerializedStateDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSerializedStateDelta", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (delta, timeOffset, baseId))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i64,
                            i64,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::StateBuffer_3_InterpolationDelegate<
                                    TStateTable,
                                    TType,
                                    TState,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::StateBuffer_3_SmoothingDelegate<
                                    TStateTable,
                                    TType,
                                    TState,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        fullStateUpdateFrequency,
                        deltaUpdateFrequency,
                        _cordl_size,
                        interpolator,
                        smoother,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LocalStateBuffer_3")]
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
