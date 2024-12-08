#[cfg(feature = "NodePoseSyncStateManager")]
#[repr(C)]
#[derive(Debug)]
pub struct NodePoseSyncStateManager {
    __cordl_parent: MultiplayerSyncStateManager_5<
        NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        PoseSerializable,
        *mut NodePoseSyncStateNetSerializable,
        *mut NodePoseSyncStateDeltaNetSerializable,
    >,
}
#[cfg(feature = "NodePoseSyncStateManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NodePoseSyncStateManager => ""
    ."NodePoseSyncStateManager"
);
#[cfg(feature = "NodePoseSyncStateManager")]
impl std::ops::Deref for NodePoseSyncStateManager {
    type Target = MultiplayerSyncStateManager_5<
        NodePoseSyncState,
        crate::GlobalNamespace::NodePoseSyncState_NodePose,
        PoseSerializable,
        *mut NodePoseSyncStateNetSerializable,
        *mut NodePoseSyncStateDeltaNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl std::ops::DerefMut for NodePoseSyncStateManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl NodePoseSyncStateManager {
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_localBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_localBufferSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Interpolate(
        &mut self,
        prev: PoseSerializable,
        prevTime: i64,
        curr: PoseSerializable,
        currTime: i64,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<PoseSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: PoseSerializable = __cordl_object
            .invoke("Interpolate", (prev, prevTime, curr, currTime, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaSerializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut IPacketPool_1<*mut NodePoseSyncStateDeltaNetSerializable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IPacketPool_1<
            *mut NodePoseSyncStateDeltaNetSerializable,
        > = __cordl_object.invoke("get_deltaSerializablePool", ())?;
        Ok(__cordl_ret)
    }
    pub fn Smooth(
        &mut self,
        a: PoseSerializable,
        b: PoseSerializable,
        smooth: f32,
    ) -> quest_hook::libil2cpp::Result<PoseSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: PoseSerializable = __cordl_object
            .invoke("Smooth", (a, b, smooth))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deltaUpdateFrequencyMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_deltaUpdateFrequencyMs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_remoteBufferSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_remoteBufferSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_serializablePool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut IPacketPool_1<*mut NodePoseSyncStateNetSerializable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IPacketPool_1<*mut NodePoseSyncStateNetSerializable> = __cordl_object
            .invoke("get_serializablePool", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fullStateUpdateFrequencyMs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_fullStateUpdateFrequencyMs", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "NodePoseSyncStateManager")]
impl quest_hook::libil2cpp::ObjectType for NodePoseSyncStateManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
