#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerGamePoseGeneratorMirror {
    __cordl_parent: crate::GlobalNamespace::MockPlayerGamePoseGenerator,
    pub _nodePoseSyncStateManager: *mut crate::GlobalNamespace::NodePoseSyncStateManager,
    pub _mirroredPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _onSongFinished: *mut crate::System::Action,
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MockPlayerGamePoseGeneratorMirror => ""
    ."MockPlayerGamePoseGeneratorMirror"
);
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    type Target = crate::GlobalNamespace::MockPlayerGamePoseGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
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
    pub fn HandleLevelFinished(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        results: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFinished", (userId, results))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasCut(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        noteCutInfo: *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasCut", (userId, syncTime, songTime, noteCutInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasMissed(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        noteMissInfo: *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoteWasMissed", (userId, syncTime, songTime, noteMissInfo))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfoNetSerializable: *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
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
    pub fn HandleObstacleWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
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
    pub fn HandleOptionalAvatarDataReceived(
        &mut self,
        optionalAvatarDataPacket: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
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
        nodePose: *mut crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleScoreSyncStateUpdate", (nodePose, player))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSliderWasSpawned(
        &mut self,
        userId: *mut quest_hook::libil2cpp::Il2CppString,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfoNetSerializable: *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
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
    pub fn Init(
        &mut self,
        introStartTime: i64,
        beatmapData: *mut crate::GlobalNamespace::MockBeatmapData,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
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
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        leftHanded: bool,
        nodePoseSyncStateManager: *mut crate::GlobalNamespace::NodePoseSyncStateManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        leftHanded: bool,
        nodePoseSyncStateManager: *mut crate::GlobalNamespace::NodePoseSyncStateManager,
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
}
#[cfg(feature = "MockPlayerGamePoseGeneratorMirror")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlayerGamePoseGeneratorMirror {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
