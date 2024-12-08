#[cfg(feature = "IGameplayRpcManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IGameplayRpcManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IGameplayRpcManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IGameplayRpcManager => ""."IGameplayRpcManager"
);
#[cfg(feature = "IGameplayRpcManager")]
impl std::ops::Deref for IGameplayRpcManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IGameplayRpcManager")]
impl std::ops::DerefMut for IGameplayRpcManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IGameplayRpcManager")]
impl IGameplayRpcManager {
    pub fn NoteSpawned(
        &mut self,
        songTime: f32,
        noteSpawnInfoNetSerializable: *mut NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteSpawned", (songTime, noteSpawnInfoNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn set_enabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_requestReturnToMenuEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_requestReturnToMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn LevelFinished(
        &mut self,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LevelFinished", (results))?;
        Ok(__cordl_ret)
    }
    pub fn NoteCut(
        &mut self,
        songTime: f32,
        noteCutInfoNetSerializable: *mut NoteCutInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteCut", (songTime, noteCutInfoNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getGameplaySongReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteMissInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn NoteMissed(
        &mut self,
        songTime: f32,
        noteMissInfoNetSerializable: *mut NoteMissInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteMissed", (songTime, noteMissInfoNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameplaySongReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplaySongReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_returnToMenuEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_returnToMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnToMenu", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_getGameplaySongReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasCutEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameplaySceneReady(
        &mut self,
        playerSpecificSettings: *mut PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplaySceneReady", (playerSpecificSettings))?;
        Ok(__cordl_ret)
    }
    pub fn GetGameplaySongReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGameplaySongReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_setGameplaySceneSyncFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::System::String,
            *mut PlayerSpecificSettingsAtStartNetSerializable,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySceneSyncFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setGameplaySongReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_obstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_sliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_returnToMenuEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_returnToMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setPlayerDidConnectLateEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            *mut crate::System::String,
            *mut PlayerSpecificSettingsAtStartNetSerializable,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setPlayerDidConnectLateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RequestReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestReturnToMenu", ())?;
        Ok(__cordl_ret)
    }
    pub fn ObstacleSpawned(
        &mut self,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: *mut ObstacleSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ObstacleSpawned", (songTime, obstacleSpawnInfoNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn add_requestReturnToMenuEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_requestReturnToMenuEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_sliderWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setGameplaySceneSyncFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::System::String,
            *mut PlayerSpecificSettingsAtStartNetSerializable,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySceneSyncFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayerDidConnectLate(
        &mut self,
        userId: *mut crate::System::String,
        playersAtGameStartNetSerializable: *mut PlayerSpecificSettingsAtStartNetSerializable,
        sessionGameId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPlayerDidConnectLate",
                (userId, playersAtGameStartNetSerializable, sessionGameId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_setSongStartTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setSongStartTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetSongStartTime(
        &mut self,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSongStartTime", (startTime))?;
        Ok(__cordl_ret)
    }
    pub fn add_getGameplaySceneReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_obstacleWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetGameplaySceneReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGameplaySceneReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setPlayerDidConnectLateEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            *mut crate::System::String,
            *mut PlayerSpecificSettingsAtStartNetSerializable,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setPlayerDidConnectLateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getGameplaySceneReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setSongStartTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setSongStartTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setGameplaySceneReadyEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setGameplaySongReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setGameplaySceneReadyEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasCutEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasCutEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_noteWasMissedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteMissInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetGameplaySceneSyncFinished(
        &mut self,
        playersAtGameStartNetSerializable: *mut PlayerSpecificSettingsAtStartNetSerializable,
        sessionGameId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetGameplaySceneSyncFinished",
                (playersAtGameStartNetSerializable, sessionGameId),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SliderSpawned(
        &mut self,
        songTime: f32,
        sliderSpawnInfoNetSerializable: *mut SliderSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SliderSpawned", (songTime, sliderSpawnInfoNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn add_noteWasSpawnedEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            i64,
            f32,
            *mut NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IGameplayRpcManager")]
impl quest_hook::libil2cpp::ObjectType for IGameplayRpcManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
