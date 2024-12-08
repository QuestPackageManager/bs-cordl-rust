#[cfg(feature = "MockPlayerGamePoseGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGenerator {
    __cordl_parent: crate::System::Object,
    pub multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub gameplayRpcManager: *mut IGameplayRpcManager,
    pub leftHanded: bool,
    pub mockNodePoseSyncStateSender: *mut MockNodePoseSyncStateSender,
    pub mockScoreSyncStateSender: *mut MockScoreSyncStateSender,
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerGamePoseGenerator => ""
    ."MockPlayerGamePoseGenerator"
);
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl std::ops::Deref for MockPlayerGamePoseGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl std::ops::DerefMut for MockPlayerGamePoseGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl MockPlayerGamePoseGenerator {
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
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: *mut MockBeatmapData,
        gameplayModifiers: *mut GameplayModifiers,
        onSongFinished: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (introStartTime, beatmapData, gameplayModifiers, onSongFinished),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, leftHanded),
            )?;
        Ok(__cordl_object)
    }
    pub fn SimulateFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateFail", ())?;
        Ok(__cordl_ret)
    }
    pub fn SimulateGiveUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SimulateGiveUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        leftHanded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, leftHanded),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MockPlayerGamePoseGenerator")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerGamePoseGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
