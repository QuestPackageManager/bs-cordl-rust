#[cfg(feature = "ScoreSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ScoreSyncStateManager {
    __cordl_parent: crate::GlobalNamespace::MultiplayerSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >,
}
#[cfg(feature = "ScoreSyncStateManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ScoreSyncStateManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScoreSyncStateManager";
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
#[cfg(feature = "ScoreSyncStateManager")]
impl std::ops::Deref for crate::GlobalNamespace::ScoreSyncStateManager {
    type Target = crate::GlobalNamespace::MultiplayerSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScoreSyncStateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl crate::GlobalNamespace::ScoreSyncStateManager {
    pub fn Interpolate(
        &mut self,
        prev: i32,
        prevTime: i64,
        curr: i32,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i64, i32, i64, i64), i32, 5usize>("Interpolate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Interpolate", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (prev, prevTime, curr, currTime, _cordl_time))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaMessageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
                0usize,
            >("get_deltaMessageType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_deltaMessageType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaSerializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IPacketPool_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
                        >,
                    >,
                >,
                0usize,
            >("get_deltaSerializablePool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_deltaSerializablePool", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaUpdateFrequencyMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_deltaUpdateFrequencyMs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_deltaUpdateFrequencyMs", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_fullStateUpdateFrequencyMs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_fullStateUpdateFrequencyMs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_fullStateUpdateFrequencyMs", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_localBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_localBufferSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_localBufferSize", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_messageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
                0usize,
            >("get_messageType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_messageType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_remoteBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_remoteBufferSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_remoteBufferSize", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_serializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IPacketPool_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                        >,
                    >,
                >,
                0usize,
            >("get_serializablePool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_serializablePool", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScoreSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl AsRef<crate::GlobalNamespace::IScoreSyncStateManager>
for crate::GlobalNamespace::ScoreSyncStateManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IScoreSyncStateManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl AsMut<crate::GlobalNamespace::IScoreSyncStateManager>
for crate::GlobalNamespace::ScoreSyncStateManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IScoreSyncStateManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl AsRef<
    crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::ScoreSyncStateManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ScoreSyncStateManager")]
impl AsMut<
    crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    >,
> for crate::GlobalNamespace::ScoreSyncStateManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IScoreSyncStateManager_5<
        crate::GlobalNamespace::StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
