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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScoreSyncStateManager => ""
    ."ScoreSyncStateManager"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Interpolate", (prev, prevTime, curr, currTime, _cordl_time))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaMessageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = __cordl_object
            .invoke("get_deltaMessageType", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateDeltaNetSerializable,
                >,
            >,
        > = __cordl_object.invoke("get_deltaSerializablePool", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_deltaUpdateFrequencyMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_deltaUpdateFrequencyMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fullStateUpdateFrequencyMs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_fullStateUpdateFrequencyMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_localBufferSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_messageType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerSessionManager_MessageType = __cordl_object
            .invoke("get_messageType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_remoteBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_remoteBufferSize", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                >,
            >,
        > = __cordl_object.invoke("get_serializablePool", ())?;
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
