#[cfg(feature = "MockNodePoseSyncStateSender")]
#[repr(C)]
#[derive(Debug)]
pub struct MockNodePoseSyncStateSender {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockNodePoseSyncStateSender => ""
    ."MockNodePoseSyncStateSender"
);
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl std::ops::Deref for MockNodePoseSyncStateSender {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl std::ops::DerefMut for MockNodePoseSyncStateSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl MockNodePoseSyncStateSender {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNodePoseSyncStateUpdate(
        &mut self,
        nodePose: *mut NodePoseSyncStateNetSerializable,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNodePoseSyncStateUpdate", (nodePose, connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        msm: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (msm))?;
        Ok(__cordl_object)
    }
    pub fn SendPose(
        &mut self,
        headPose: PoseSerializable,
        leftHandPose: PoseSerializable,
        rightHandPose: PoseSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendPose", (headPose, leftHandPose, rightHandPose))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        msm: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (msm))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockNodePoseSyncStateSender")]
impl quest_hook::libil2cpp::ObjectType for MockNodePoseSyncStateSender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
