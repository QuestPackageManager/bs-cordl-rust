#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGeneratorMirror {
    __cordl_parent: MockPlayerGamePoseGenerator,
    pub _nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    pub _mirroredPlayer: *mut IConnectedPlayer,
    pub _onSongFinished: *mut crate::System::Action,
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerGamePoseGeneratorMirror => ""
    ."MockPlayerGamePoseGeneratorMirror"
);
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::Deref for MockPlayerGamePoseGeneratorMirror {
    type Target = MockPlayerGamePoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::DerefMut for MockPlayerGamePoseGeneratorMirror {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl MockPlayerGamePoseGeneratorMirror {
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
    pub fn HandleSliderWasSpawned(
        &mut self,
        userId: *mut crate::System::String,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfoNetSerializable: *mut SliderSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSliderWasSpawned",
                (userId, syncTime, songTime, sliderSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleObstacleWasSpawned(
        &mut self,
        userId: *mut crate::System::String,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: *mut ObstacleSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleObstacleWasSpawned",
                (userId, syncTime, songTime, obstacleSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        leftHanded: bool,
        nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    leftHanded,
                    nodePoseSyncStateManager,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFinished(
        &mut self,
        userId: *mut crate::System::String,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFinished", (userId, results))?;
        Ok(__cordl_ret)
    }
    pub fn FindPlayerToMirror(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindPlayerToMirror", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        userId: *mut crate::System::String,
        syncTime: i64,
        songTime: f32,
        noteCutInfo: *mut NoteCutInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (userId, syncTime, songTime, noteCutInfo))?;
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
    pub fn HandleNoteWasMissed(
        &mut self,
        userId: *mut crate::System::String,
        syncTime: i64,
        songTime: f32,
        noteMissInfo: *mut NoteMissInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (userId, syncTime, songTime, noteMissInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleOptionalAvatarDataReceived(
        &mut self,
        optionalAvatarDataPacket: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleOptionalAvatarDataReceived",
                (optionalAvatarDataPacket, player),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleScoreSyncStateUpdate(
        &mut self,
        nodePose: *mut StandardScoreSyncStateNetSerializable,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreSyncStateUpdate", (nodePose, player))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        userId: *mut crate::System::String,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfoNetSerializable: *mut NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteWasSpawned",
                (userId, syncTime, songTime, noteSpawnInfoNetSerializable),
            )?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        gameplayRpcManager: *mut IGameplayRpcManager,
        leftHanded: bool,
        nodePoseSyncStateManager: *mut NodePoseSyncStateManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    multiplayerSessionManager,
                    gameplayRpcManager,
                    leftHanded,
                    nodePoseSyncStateManager,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerGamePoseGeneratorMirror {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
