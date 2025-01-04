#[cfg(feature = "GameplayRpcManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IMultiplayerSessionManager,
    >,
    pub _rpcHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RpcHandler_1<
            crate::GlobalNamespace::GameplayRpcManager_RpcType,
        >,
    >,
    pub setGameplaySceneSyncFinishedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub setGameplaySceneReadyEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    >,
    pub getGameplaySceneReadyEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub setPlayerDidConnectLateEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub setGameplaySongReadyEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub getGameplaySongReadyEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub setSongStartTimeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<*mut quest_hook::libil2cpp::Il2CppString, i64>,
    >,
    pub noteWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
            f32,
            *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    >,
    pub obstacleWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
            f32,
            *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    >,
    pub sliderWasSpawnedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
            f32,
            *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    >,
    pub noteWasCutEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
            f32,
            *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    >,
    pub noteWasMissedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            *mut quest_hook::libil2cpp::Il2CppString,
            i64,
            f32,
            *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
        >,
    >,
    pub levelFinishedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    >,
    pub returnToMenuEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub requestReturnToMenuEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "GameplayRpcManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayRpcManager => ""
    ."GameplayRpcManager"
);
#[cfg(feature = "GameplayRpcManager")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl crate::GlobalNamespace::GameplayRpcManager {
    pub const kGameplayState: &'static str = "in_gameplay";
    #[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
    pub type GetGameplaySceneReadyRpc = crate::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc;
    #[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
    pub type GetGameplaySongReadyRpc = crate::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc;
    #[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
    pub type LevelFinishedRpc = crate::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc;
    #[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
    pub type NoteCutRpc = crate::GlobalNamespace::GameplayRpcManager_NoteCutRpc;
    #[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
    pub type NoteMissedRpc = crate::GlobalNamespace::GameplayRpcManager_NoteMissedRpc;
    #[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
    pub type NoteSpawnedRpc = crate::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc;
    #[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
    pub type ObstacleSpawnedRpc = crate::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc;
    #[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
    pub type RequestReturnToMenuRpc = crate::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc;
    #[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
    pub type ReturnToMenuRpc = crate::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc;
    #[cfg(feature = "GameplayRpcManager+RpcType")]
    pub type RpcType = crate::GlobalNamespace::GameplayRpcManager_RpcType;
    #[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
    pub type SetGameplaySceneReadyRpc = crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc;
    #[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
    pub type SetGameplaySceneSyncFinishedRpc = crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc;
    #[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
    pub type SetGameplaySongReadyRpc = crate::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc;
    #[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
    pub type SetPlayerDidConnectLateRpc = crate::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc;
    #[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
    pub type SetSongStartTimeRpc = crate::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc;
    #[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
    pub type SliderSpawnedRpc = crate::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameplaySceneReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGameplaySceneReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameplaySongReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetGameplaySongReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGetGameplayLevelReadyCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetGameplayLevelReadyCallback", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeGetGameplaySceneReadyCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetGameplaySceneReadyCallback", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeLevelFinishedCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeLevelFinishedCallback", (userId, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeNoteWasCutCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteCutInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeNoteWasCutCallback",
                (userId, syncTime, songTime, noteCutInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeNoteWasMissedCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteMissInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMissInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeNoteWasMissedCallback",
                (userId, syncTime, songTime, noteMissInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeNoteWasSpawnedCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        noteSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeNoteWasSpawnedCallback",
                (userId, syncTime, songTime, noteSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeObstacleWasSpawnedCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        obstacleSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeObstacleWasSpawnedCallback",
                (userId, syncTime, songTime, obstacleSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeRequestReturnToMenuCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeRequestReturnToMenuCallback", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeReturnToMenuCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeReturnToMenuCallback", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSetGameplaySceneReadyCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerSpecificSettingsNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSetGameplaySceneReadyCallback",
                (userId, playerSpecificSettingsNetSerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSetGameplaySceneSyncFinishCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersAtGameStart: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSetGameplaySceneSyncFinishCallback",
                (userId, playersAtGameStart, sessionGameId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSetGameplaySongReadyCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetGameplaySongReadyCallback", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSetPlayerDidConnectLate(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failedUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersAtGameStartNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSetPlayerDidConnectLate",
                (userId, failedUserId, playersAtGameStartNetSerializable, sessionGameId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSetSongStartTimeCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetSongStartTimeCallback", (userId, startTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeSliderWasSpawnedCallback(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        syncTime: i64,
        songTime: f32,
        sliderSpawnInfo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSliderWasSpawnedCallback",
                (userId, syncTime, songTime, sliderSpawnInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LevelFinished(
        &mut self,
        results: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LevelFinished", (results))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object.into())
    }
    pub fn NoteCut(
        &mut self,
        songTime: f32,
        noteCutInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteCutInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteCut", (songTime, noteCutInfoNetSerializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoteMissed(
        &mut self,
        songTime: f32,
        noteMissInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteMissInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteMissed", (songTime, noteMissInfoNetSerializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn NoteSpawned(
        &mut self,
        songTime: f32,
        noteSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NoteSpawned", (songTime, noteSpawnInfoNetSerializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn ObstacleSpawned(
        &mut self,
        songTime: f32,
        obstacleSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ObstacleSpawned", (songTime, obstacleSpawnInfoNetSerializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestReturnToMenu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReturnToMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReturnToMenu", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameplaySceneReady(
        &mut self,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplaySceneReady", (playerSpecificSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameplaySceneSyncFinished(
        &mut self,
        playersAtGameStartNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetGameplaySceneSyncFinished",
                (playersAtGameStartNetSerializable, sessionGameId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGameplaySongReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGameplaySongReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerDidConnectLate(
        &mut self,
        usedId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersAtGameStartNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionGameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPlayerDidConnectLate",
                (usedId, playersAtGameStartNetSerializable, sessionGameId),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SliderSpawned(
        &mut self,
        songTime: f32,
        sliderSpawnInfoNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SliderSpawned", (songTime, sliderSpawnInfoNetSerializable))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_getGameplaySceneReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_getGameplaySongReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_noteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_obstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_requestReturnToMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_requestReturnToMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_returnToMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_returnToMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_setGameplaySceneReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_setGameplaySceneSyncFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySceneSyncFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_setGameplaySongReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_setPlayerDidConnectLateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setPlayerDidConnectLateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_setSongStartTimeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut quest_hook::libil2cpp::Il2CppString, i64>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setSongStartTimeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_getGameplaySceneReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_getGameplaySongReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasCutEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasCutEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasMissedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasMissedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_noteWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_noteWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_obstacleWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_obstacleWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_requestReturnToMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_requestReturnToMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_returnToMenuEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_returnToMenuEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_setGameplaySceneReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySceneReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_setGameplaySceneSyncFinishedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySceneSyncFinishedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_setGameplaySongReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setGameplaySongReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_setPlayerDidConnectLateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setPlayerDidConnectLateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_setSongStartTimeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<*mut quest_hook::libil2cpp::Il2CppString, i64>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setSongStartTimeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sliderWasSpawnedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                *mut quest_hook::libil2cpp::Il2CppString,
                i64,
                f32,
                *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sliderWasSpawnedEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameplayRpcManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl AsRef<crate::GlobalNamespace::IGameplayRpcManager>
for crate::GlobalNamespace::GameplayRpcManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IGameplayRpcManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl AsMut<crate::GlobalNamespace::IGameplayRpcManager>
for crate::GlobalNamespace::GameplayRpcManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IGameplayRpcManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::GameplayRpcManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayRpcManager")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::GameplayRpcManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_GetGameplaySceneReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc => ""
    ."GameplayRpcManager/GetGameplaySceneReadyRpc"
);
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc {
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
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySceneReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySceneReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_GetGameplaySongReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc => ""
    ."GameplayRpcManager/GetGameplaySongReadyRpc"
);
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc {
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
}
#[cfg(feature = "GameplayRpcManager+GetGameplaySongReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_GetGameplaySongReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_LevelFinishedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    >,
}
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc => ""
    ."GameplayRpcManager/LevelFinishedRpc"
);
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+LevelFinishedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_LevelFinishedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_NoteCutRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayRpcManager_NoteCutRpc
    => ""."GameplayRpcManager/NoteCutRpc"
);
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_NoteCutRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteCutInfoNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_NoteCutRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_NoteCutRpc {
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
}
#[cfg(feature = "GameplayRpcManager+NoteCutRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_NoteCutRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_NoteMissedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_NoteMissedRpc => ""
    ."GameplayRpcManager/NoteMissedRpc"
);
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_NoteMissedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteMissInfoNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_NoteMissedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_NoteMissedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+NoteMissedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_NoteMissedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_NoteSpawnedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc => ""
    ."GameplayRpcManager/NoteSpawnedRpc"
);
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+NoteSpawnedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_NoteSpawnedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_ObstacleSpawnedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc => ""
    ."GameplayRpcManager/ObstacleSpawnedRpc"
);
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::ObstacleSpawnInfoNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+ObstacleSpawnedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_ObstacleSpawnedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_RequestReturnToMenuRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc => ""
    ."GameplayRpcManager/RequestReturnToMenuRpc"
);
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc {
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
}
#[cfg(feature = "GameplayRpcManager+RequestReturnToMenuRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_RequestReturnToMenuRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_ReturnToMenuRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc => ""
    ."GameplayRpcManager/ReturnToMenuRpc"
);
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc {
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
}
#[cfg(feature = "GameplayRpcManager+ReturnToMenuRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_ReturnToMenuRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+RpcType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameplayRpcManager_RpcType {
    #[default]
    GetGameplaySceneReady = 2u8,
    GetGameplaySongReady = 5u8,
    LevelFinished = 9u8,
    NoteCut = 7u8,
    NoteMissed = 8u8,
    NoteSpawned = 12u8,
    ObstacleSpawned = 13u8,
    RequestReturnToMenu = 11u8,
    ReturnToMenu = 10u8,
    SetActivePlayerFailedToConnect = 3u8,
    SetGameplaySceneReady = 1u8,
    SetGameplaySceneSyncFinish = 0u8,
    SetGameplaySongReady = 4u8,
    SetSongStartTime = 6u8,
    SliderSpawned = 14u8,
}
#[cfg(feature = "GameplayRpcManager+RpcType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayRpcManager_RpcType =>
    ""."GameplayRpcManager/RpcType"
);
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SetGameplaySceneReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc => ""
    ."GameplayRpcManager/SetGameplaySceneReadyRpc"
);
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SetGameplaySceneSyncFinishedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc => ""
    ."GameplayRpcManager/SetGameplaySceneSyncFinishedRpc"
);
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        *mut quest_hook::libil2cpp::Il2CppString,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySceneSyncFinishedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySceneSyncFinishedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SetGameplaySongReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc => ""
    ."GameplayRpcManager/SetGameplaySongReadyRpc"
);
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SetGameplaySongReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SetGameplaySongReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SetPlayerDidConnectLateRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_3<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc => ""
    ."GameplayRpcManager/SetPlayerDidConnectLateRpc"
);
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_3<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        *mut quest_hook::libil2cpp::Il2CppString,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SetPlayerDidConnectLateRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SetPlayerDidConnectLateRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SetSongStartTimeRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<i64>,
}
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc => ""
    ."GameplayRpcManager/SetSongStartTimeRpc"
);
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SetSongStartTimeRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SetSongStartTimeRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayRpcManager_SliderSpawnedRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
    >,
}
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc => ""
    ."GameplayRpcManager/SliderSpawnedRpc"
);
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        f32,
        *mut crate::GlobalNamespace::SliderSpawnInfoNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
impl crate::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc {
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
}
#[cfg(feature = "GameplayRpcManager+SliderSpawnedRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayRpcManager_SliderSpawnedRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
