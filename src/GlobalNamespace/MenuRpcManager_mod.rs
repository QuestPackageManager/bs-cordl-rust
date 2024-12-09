#[cfg(feature = "MenuRpcManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _rpcHandler: *mut crate::GlobalNamespace::RpcHandler_1<
        crate::GlobalNamespace::MenuRpcManager_RpcType,
    >,
    pub getPlayersPermissionConfigurationEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub setPlayersPermissionConfigurationEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    >,
    pub setPlayersMissingEntitlementsToLevelEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    >,
    pub getIsEntitledToLevelEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub setIsEntitledToLevelEvent: *mut crate::System::Action_3<
        *mut crate::System::String,
        *mut crate::System::String,
        crate::GlobalNamespace::EntitlementsStatus,
    >,
    pub levelEntitlementStatusesInvalidatedEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub selectedLevelPackEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub setSelectedBeatmapEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >,
    pub clearSelectedBeatmapEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getSelectedBeatmapEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub recommendBeatmapEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >,
    pub clearRecommendedBeatmapEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getRecommendedBeatmapEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub setSelectedGameplayModifiersEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
    pub clearSelectedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getSelectedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub recommendGameplayModifiersEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
    pub clearRecommendedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getRecommendedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getIsStartButtonEnabledEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub setIsStartButtonEnabledEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        crate::GlobalNamespace::CannotStartGameReason,
    >,
    pub levelLoadErrorEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub levelLoadSuccessEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
    pub startedLevelEvent: *mut crate::System::Action_4<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        *mut crate::GlobalNamespace::GameplayModifiers,
        i64,
    >,
    pub getStartedLevelEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub getMultiplayerGameStateEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub setMultiplayerGameStateEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        crate::GlobalNamespace::MultiplayerGameState,
    >,
    pub cancelCountdownEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub setCountdownEndTimeEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        i64,
    >,
    pub getCountdownEndTimeEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub cancelledLevelStartEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getIsReadyEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub setIsReadyEvent: *mut crate::System::Action_2<*mut crate::System::String, bool>,
    pub setStartGameTimeEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        i64,
    >,
    pub cancelStartGameTimeEvent: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
    pub getIsInLobbyEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub setIsInLobbyEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        bool,
    >,
    pub getOwnedSongPacksEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub setOwnedSongPacksEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        crate::GlobalNamespace::SongPackMask,
    >,
    pub requestedKickPlayerEvent: *mut crate::System::Action_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager => ""
    ."MenuRpcManager"
);
#[cfg(feature = "MenuRpcManager")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager")]
impl crate::GlobalNamespace::MenuRpcManager {
    pub const kMenuState: &'static str = "in_menu";
    #[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
    pub type CancelCountdownRpc = crate::GlobalNamespace::MenuRpcManager_CancelCountdownRpc;
    #[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
    pub type CancelLevelStartRpc = crate::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc;
    #[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
    pub type CancelStartGameTimeRpc = crate::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc;
    #[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
    pub type ClearRecommendedBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
    pub type ClearRecommendedGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
    pub type ClearSelectedBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
    pub type ClearSelectedGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
    pub type GetCountdownEndTimeRpc = crate::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc;
    #[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
    pub type GetIsEntitledToLevelRpc = crate::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc;
    #[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
    pub type GetIsInLobbyRpc = crate::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc;
    #[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
    pub type GetIsReadyRpc = crate::GlobalNamespace::MenuRpcManager_GetIsReadyRpc;
    #[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
    pub type GetIsStartButtonEnabledRpc = crate::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc;
    #[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
    pub type GetMultiplayerGameStateRpc = crate::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc;
    #[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
    pub type GetOwnedSongPacksRpc = crate::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc;
    #[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
    pub type GetPlayersPermissionConfigurationRpc = crate::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc;
    #[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
    pub type GetRecommendedBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
    pub type GetRecommendedGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
    pub type GetSelectedBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
    pub type GetSelectedGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
    pub type GetStartedLevelRpc = crate::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc;
    #[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
    pub type InvalidateLevelEntitlementStatusesRpc = crate::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc;
    #[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
    pub type LevelLoadErrorRpc = crate::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc;
    #[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
    pub type LevelLoadSuccessRpc = crate::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc;
    #[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
    pub type RecommendBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
    pub type RecommendGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
    pub type RequestKickPlayerRpc = crate::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc;
    #[cfg(feature = "MenuRpcManager+RpcType")]
    pub type RpcType = crate::GlobalNamespace::MenuRpcManager_RpcType;
    #[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
    pub type SelectLevelPackRpc = crate::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc;
    #[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
    pub type SetCountdownEndTimeRpc = crate::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc;
    #[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
    pub type SetIsEntitledToLevelRpc = crate::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc;
    #[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
    pub type SetIsInLobbyRpc = crate::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc;
    #[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
    pub type SetIsReadyRpc = crate::GlobalNamespace::MenuRpcManager_SetIsReadyRpc;
    #[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
    pub type SetIsStartButtonEnabledRpc = crate::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc;
    #[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
    pub type SetMultiplayerGameStateRpc = crate::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc;
    #[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
    pub type SetOwnedSongPacksRpc = crate::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc;
    #[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
    pub type SetPlayersMissingEntitlementsToLevelRpc = crate::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc;
    #[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
    pub type SetPlayersPermissionConfigurationRpc = crate::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc;
    #[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
    pub type SetSelectedBeatmapRpc = crate::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc;
    #[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
    pub type SetSelectedGameplayModifiersRpc = crate::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc;
    #[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
    pub type SetStartGameTimeRpc = crate::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc;
    #[cfg(feature = "MenuRpcManager+StartLevelRpc")]
    pub type StartLevelRpc = crate::GlobalNamespace::MenuRpcManager_StartLevelRpc;
    pub fn CancelCountdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelCountdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn CancelLevelStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelLevelStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn CancelStartGameTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelStartGameTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearRecommendedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRecommendedBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearRecommendedGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRecommendedGameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearSelectedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelectedBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearSelectedGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelectedGameplayModifiers", ())?;
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
    pub fn EnabledForPlayer(
        &mut self,
        player: *mut crate::GlobalNamespace::IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("EnabledForPlayer", (player))?;
        Ok(__cordl_ret)
    }
    pub fn GetCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCountdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIsEntitledToLevel(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIsEntitledToLevel", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn GetIsInLobby(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIsInLobby", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIsReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIsReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIsStartButtonEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIsStartButtonEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMultiplayerGameState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetMultiplayerGameState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOwnedSongPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetOwnedSongPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayersPermissionConfiguration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetPlayersPermissionConfiguration", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRecommendedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetRecommendedBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRecommendedGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetRecommendedGameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedBeatmap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSelectedBeatmap", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSelectedGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSelectedGameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetStartedLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetStartedLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvalidateLevelEntitlementStatuses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvalidateLevelEntitlementStatuses", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeCancelCountdown(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCancelCountdown", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeCancelLevelStart(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCancelLevelStart", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeCancelStartGameCountdown(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeCancelStartGameCountdown", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeClearRecommendedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeClearRecommendedBeatmap", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeClearRecommendedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeClearRecommendedGameplayModifiers", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeClearSelectedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeClearSelectedBeatmap", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeClearSelectedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeClearSelectedGameplayModifiers", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetCountdownEndTime(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetCountdownEndTime", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetIsEntitledToLevel(
        &mut self,
        userId: *mut crate::System::String,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetIsEntitledToLevel", (userId, levelId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetIsInLobby(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetIsInLobby", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetIsReady(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetIsReady", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetIsStartButtonEnabled(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetIsStartButtonEnabled", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetMultiplayerGameState(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetMultiplayerGameState", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetOwnedSongPacks(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetOwnedSongPacks", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetPlayersPermissionConfiguration(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetPlayersPermissionConfiguration", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetRecommendedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetRecommendedBeatmap", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetRecommendedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetRecommendedGameplayModifiers", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetSelectedBeatmapRpc(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetSelectedBeatmapRpc", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetSelectedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetSelectedGameplayModifiers", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeGetStartedLevel(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeGetStartedLevel", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeKickPlayer(
        &mut self,
        userId: *mut crate::System::String,
        kickedPlayerId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeKickPlayer", (userId, kickedPlayerId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeLevelEntitlementStatusesInvalidated(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeLevelEntitlementStatusesInvalidated", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeLevelLoadError(
        &mut self,
        userId: *mut crate::System::String,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeLevelLoadError", (userId, levelId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeLevelLoadSuccess(
        &mut self,
        userId: *mut crate::System::String,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeLevelLoadSuccess", (userId, levelId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeOnSelectedLevelPackEvent(
        &mut self,
        userId: *mut crate::System::String,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeOnSelectedLevelPackEvent", (userId, levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeRecommendBeatmap(
        &mut self,
        userId: *mut crate::System::String,
        key: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeRecommendBeatmap", (userId, key))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeRecommendGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeRecommendGameplayModifiers", (userId, gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetCountdownEndTime(
        &mut self,
        userId: *mut crate::System::String,
        newTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetCountdownEndTime", (userId, newTime))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetIsEntitledToLevel(
        &mut self,
        userId: *mut crate::System::String,
        levelId: *mut crate::System::String,
        entitlementStatus: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetIsEntitledToLevel", (userId, levelId, entitlementStatus))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetIsInLobby(
        &mut self,
        userId: *mut crate::System::String,
        isBack: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetIsInLobby", (userId, isBack))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetIsReady(
        &mut self,
        userId: *mut crate::System::String,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetIsReady", (userId, isReady))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetIsStartButtonEnabled(
        &mut self,
        userId: *mut crate::System::String,
        reason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetIsStartButtonEnabled", (userId, reason))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetMultiplayerGameState(
        &mut self,
        userId: *mut crate::System::String,
        lobbyState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetMultiplayerGameState", (userId, lobbyState))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetOwnedSongPacks(
        &mut self,
        userId: *mut crate::System::String,
        songPackMask: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetOwnedSongPacks", (userId, songPackMask))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetPlayersMissingEntitlementsToLevelRpc(
        &mut self,
        userId: *mut crate::System::String,
        playersMissingEntitlements: *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSetPlayersMissingEntitlementsToLevelRpc",
                (userId, playersMissingEntitlements),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetPlayersPermissionConfiguration(
        &mut self,
        userId: *mut crate::System::String,
        playersPermissionConfiguration: *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeSetPlayersPermissionConfiguration",
                (userId, playersPermissionConfiguration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetSelectedBeatmap(
        &mut self,
        userId: *mut crate::System::String,
        key: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetSelectedBeatmap", (userId, key))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetSelectedGameplayModifiers(
        &mut self,
        userId: *mut crate::System::String,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetSelectedGameplayModifiers", (userId, gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeSetStartGameCountdown(
        &mut self,
        userId: *mut crate::System::String,
        newTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeSetStartGameCountdown", (userId, newTime))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeStartLevel(
        &mut self,
        userId: *mut crate::System::String,
        beatmapKeySerializable: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InvokeStartLevel",
                (userId, beatmapKeySerializable, gameplayModifiers, startTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LevelLoadError(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LevelLoadError", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn LevelLoadSuccess(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LevelLoadSuccess", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_object)
    }
    pub fn RecommendBeatmap(
        &mut self,
        key: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecommendBeatmap", (key))?;
        Ok(__cordl_ret)
    }
    pub fn RecommendGameplayModifiers(
        &mut self,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecommendGameplayModifiers", (gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn RequestKickPlayer(
        &mut self,
        kickedPlayerId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestKickPlayer", (kickedPlayerId))?;
        Ok(__cordl_ret)
    }
    pub fn SelectLevelPack(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLevelPack", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn SetCountdownEndTime(
        &mut self,
        newTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCountdownEndTime", (newTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsEntitledToLevel(
        &mut self,
        levelId: *mut crate::System::String,
        entitlementStatus: crate::GlobalNamespace::EntitlementsStatus,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsEntitledToLevel", (levelId, entitlementStatus))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsInLobby(
        &mut self,
        isBack: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsInLobby", (isBack))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsReady(
        &mut self,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsReady", (isReady))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsStartButtonEnabled(
        &mut self,
        reason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsStartButtonEnabled", (reason))?;
        Ok(__cordl_ret)
    }
    pub fn SetMultiplayerGameState(
        &mut self,
        lobbyState: crate::GlobalNamespace::MultiplayerGameState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMultiplayerGameState", (lobbyState))?;
        Ok(__cordl_ret)
    }
    pub fn SetOwnedSongPacks(
        &mut self,
        songPackMask: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOwnedSongPacks", (songPackMask))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayersMissingEntitlementsToLevel(
        &mut self,
        playersMissingEntitlements: *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPlayersMissingEntitlementsToLevel",
                (playersMissingEntitlements),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayersPermissionConfiguration(
        &mut self,
        playersPermissionConfiguration: *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPlayersPermissionConfiguration",
                (playersPermissionConfiguration),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectedBeatmap(
        &mut self,
        key: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedBeatmap", (key))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectedGameplayModifiers(
        &mut self,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedGameplayModifiers", (gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn SetStartGameTime(
        &mut self,
        newTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartGameTime", (newTime))?;
        Ok(__cordl_ret)
    }
    pub fn StartLevel(
        &mut self,
        beatmapKeySerializable: *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        startTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "StartLevel",
                (beatmapKeySerializable, gameplayModifiers, startTime),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager))?;
        Ok(__cordl_ret)
    }
    pub fn add_cancelCountdownEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelCountdownEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_cancelStartGameTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelStartGameTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_cancelledLevelStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelledLevelStartEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearRecommendedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearRecommendedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearRecommendedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearRecommendedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getCountdownEndTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getCountdownEndTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getIsEntitledToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getIsEntitledToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getIsInLobbyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getIsInLobbyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getIsReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getIsReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getIsStartButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getIsStartButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getMultiplayerGameStateEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getMultiplayerGameStateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getOwnedSongPacksEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getOwnedSongPacksEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getPlayersPermissionConfigurationEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getPlayersPermissionConfigurationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getRecommendedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getRecommendedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getRecommendedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getRecommendedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_getStartedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_getStartedLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelEntitlementStatusesInvalidatedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelEntitlementStatusesInvalidatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelLoadErrorEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelLoadErrorEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelLoadSuccessEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelLoadSuccessEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_recommendBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_recommendBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_recommendGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_recommendGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_requestedKickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_requestedKickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectedLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectedLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setCountdownEndTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setCountdownEndTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setIsEntitledToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::System::String,
            *mut crate::System::String,
            crate::GlobalNamespace::EntitlementsStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setIsEntitledToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setIsInLobbyEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setIsInLobbyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setIsReadyEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setIsReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setIsStartButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::CannotStartGameReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setIsStartButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setMultiplayerGameStateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::MultiplayerGameState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setMultiplayerGameStateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setOwnedSongPacksEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::SongPackMask,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setOwnedSongPacksEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setPlayersMissingEntitlementsToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setPlayersMissingEntitlementsToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setPlayersPermissionConfigurationEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setPlayersPermissionConfigurationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_setStartGameTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setStartGameTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
            *mut crate::GlobalNamespace::GameplayModifiers,
            i64,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startedLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_enabledForAllPlayers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enabledForAllPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerSessionManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::IMultiplayerSessionManager = __cordl_object
            .invoke("get_multiplayerSessionManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_syncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_syncTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_cancelCountdownEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelCountdownEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_cancelStartGameTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelStartGameTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_cancelledLevelStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelledLevelStartEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearRecommendedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearRecommendedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearRecommendedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearRecommendedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getCountdownEndTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getCountdownEndTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getIsEntitledToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getIsEntitledToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getIsInLobbyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getIsInLobbyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getIsReadyEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getIsReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getIsStartButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getIsStartButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getMultiplayerGameStateEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getMultiplayerGameStateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getOwnedSongPacksEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getOwnedSongPacksEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getPlayersPermissionConfigurationEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getPlayersPermissionConfigurationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getRecommendedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getRecommendedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getRecommendedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getRecommendedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_getStartedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_getStartedLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelEntitlementStatusesInvalidatedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelEntitlementStatusesInvalidatedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelLoadErrorEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelLoadErrorEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelLoadSuccessEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelLoadSuccessEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_recommendBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_recommendBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_recommendGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_recommendGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_requestedKickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_requestedKickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectedLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectedLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setCountdownEndTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setCountdownEndTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setIsEntitledToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_3<
            *mut crate::System::String,
            *mut crate::System::String,
            crate::GlobalNamespace::EntitlementsStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setIsEntitledToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setIsInLobbyEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setIsInLobbyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setIsReadyEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setIsReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setIsStartButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::CannotStartGameReason,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setIsStartButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setMultiplayerGameStateEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::MultiplayerGameState,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setMultiplayerGameStateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setOwnedSongPacksEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            crate::GlobalNamespace::SongPackMask,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setOwnedSongPacksEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setPlayersMissingEntitlementsToLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setPlayersMissingEntitlementsToLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setPlayersPermissionConfigurationEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setPlayersPermissionConfigurationEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setSelectedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setSelectedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setSelectedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setSelectedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_setStartGameTimeEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setStartGameTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_startedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut crate::System::String,
            *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
            *mut crate::GlobalNamespace::GameplayModifiers,
            i64,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startedLevelEvent", (value))?;
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
}
#[cfg(feature = "MenuRpcManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MenuRpcManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_CancelCountdownRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_CancelCountdownRpc => ""
    ."MenuRpcManager/CancelCountdownRpc"
);
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_CancelCountdownRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_CancelCountdownRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
impl crate::GlobalNamespace::MenuRpcManager_CancelCountdownRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+CancelCountdownRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_CancelCountdownRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_CancelLevelStartRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc => ""
    ."MenuRpcManager/CancelLevelStartRpc"
);
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
impl crate::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+CancelLevelStartRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_CancelLevelStartRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_CancelStartGameTimeRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc => ""
    ."MenuRpcManager/CancelStartGameTimeRpc"
);
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
impl crate::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+CancelStartGameTimeRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_CancelStartGameTimeRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_ClearRecommendedBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc => ""
    ."MenuRpcManager/ClearRecommendedBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_ClearRecommendedGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc => ""
    ."MenuRpcManager/ClearRecommendedGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+ClearRecommendedGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_ClearRecommendedGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_ClearSelectedBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc => ""
    ."MenuRpcManager/ClearSelectedBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+ClearSelectedBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_ClearSelectedBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_ClearSelectedGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc => ""
    ."MenuRpcManager/ClearSelectedGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+ClearSelectedGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_ClearSelectedGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetCountdownEndTimeRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc => ""
    ."MenuRpcManager/GetCountdownEndTimeRpc"
);
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetCountdownEndTimeRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetCountdownEndTimeRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetIsEntitledToLevelRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc => ""
    ."MenuRpcManager/GetIsEntitledToLevelRpc"
);
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetIsEntitledToLevelRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetIsEntitledToLevelRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetIsInLobbyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc
    => ""."MenuRpcManager/GetIsInLobbyRpc"
);
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetIsInLobbyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetIsInLobbyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetIsReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_GetIsReadyRpc =>
    ""."MenuRpcManager/GetIsReadyRpc"
);
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetIsReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_GetIsReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetIsReadyRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetIsReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetIsReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetIsStartButtonEnabledRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc => ""
    ."MenuRpcManager/GetIsStartButtonEnabledRpc"
);
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetIsStartButtonEnabledRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetIsStartButtonEnabledRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetMultiplayerGameStateRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc => ""
    ."MenuRpcManager/GetMultiplayerGameStateRpc"
);
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetMultiplayerGameStateRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetMultiplayerGameStateRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetOwnedSongPacksRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc => ""
    ."MenuRpcManager/GetOwnedSongPacksRpc"
);
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetOwnedSongPacksRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetOwnedSongPacksRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetPlayersPermissionConfigurationRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc => ""
    ."MenuRpcManager/GetPlayersPermissionConfigurationRpc"
);
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetPlayersPermissionConfigurationRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetPlayersPermissionConfigurationRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetRecommendedBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc => ""
    ."MenuRpcManager/GetRecommendedBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetRecommendedBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetRecommendedGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc => ""
    ."MenuRpcManager/GetRecommendedGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetRecommendedGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetRecommendedGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetSelectedBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc => ""
    ."MenuRpcManager/GetSelectedBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetSelectedBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetSelectedBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetSelectedGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc => ""
    ."MenuRpcManager/GetSelectedGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetSelectedGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetSelectedGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_GetStartedLevelRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc => ""
    ."MenuRpcManager/GetStartedLevelRpc"
);
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
impl crate::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+GetStartedLevelRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_GetStartedLevelRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_InvalidateLevelEntitlementStatusesRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall,
}
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc => ""
    ."MenuRpcManager/InvalidateLevelEntitlementStatusesRpc"
);
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
impl crate::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+InvalidateLevelEntitlementStatusesRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_InvalidateLevelEntitlementStatusesRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_LevelLoadErrorRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc => ""
    ."MenuRpcManager/LevelLoadErrorRpc"
);
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
impl crate::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+LevelLoadErrorRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_LevelLoadErrorRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_LevelLoadSuccessRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc => ""
    ."MenuRpcManager/LevelLoadSuccessRpc"
);
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
impl crate::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+LevelLoadSuccessRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_LevelLoadSuccessRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_RecommendBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >,
}
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc => ""
    ."MenuRpcManager/RecommendBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+RecommendBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_RecommendBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_RecommendGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
}
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc => ""
    ."MenuRpcManager/RecommendGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+RecommendGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_RecommendGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_RequestKickPlayerRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc => ""
    ."MenuRpcManager/RequestKickPlayerRpc"
);
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
impl crate::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+RequestKickPlayerRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_RequestKickPlayerRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+RpcType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuRpcManager_RpcType {
    CancelCountdown = 30u8,
    CancelLevelStart = 19u8,
    CancelStartGameTime = 25u8,
    ClearRecommendedBeatmap = 8u8,
    ClearRecommendedGameplayModifiers = 13u8,
    ClearSelectedBeatmap = 38u8,
    ClearSelectedGameplayModifiers = 39u8,
    GetCountdownEndTime = 28u8,
    GetIsEntitledToLevel = 1u8,
    GetIsInLobby = 26u8,
    GetIsReady = 22u8,
    GetIsStartButtonEnabled = 36u8,
    GetMultiplayerGameState = 20u8,
    GetOwnedSongPacks = 31u8,
    GetPermissionConfiguration = 34u8,
    GetRecommendedBeatmap = 9u8,
    GetRecommendedGameplayModifiers = 14u8,
    GetSelectedBeatmap = 6u8,
    GetSelectedGameplayModifiers = 11u8,
    GetStartedLevel = 18u8,
    InvalidateLevelEntitlementStatuses = 3u8,
    LevelLoadError = 15u8,
    LevelLoadSuccess = 16u8,
    RecommendBeatmap = 7u8,
    RecommendGameplayModifiers = 12u8,
    RequestKickPlayer = 33u8,
    SelectLevelPack = 4u8,
    SetCountdownEndTime = 29u8,
    SetIsEntitledToLevel = 2u8,
    SetIsInLobby = 27u8,
    SetIsReady = 23u8,
    SetIsStartButtonEnabled = 37u8,
    SetMultiplayerGameState = 21u8,
    SetOwnedSongPacks = 32u8,
    SetPermissionConfiguration = 35u8,
    SetPlayersMissingEntitlementsToLevel = 0u8,
    SetSelectedBeatmap = 5u8,
    SetSelectedGameplayModifiers = 10u8,
    SetStartGameTime = 24u8,
    StartLevel = 17u8,
}
#[cfg(feature = "MenuRpcManager+RpcType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_RpcType => ""
    ."MenuRpcManager/RpcType"
);
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SelectLevelPackRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc => ""
    ."MenuRpcManager/SelectLevelPackRpc"
);
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::System::String,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SelectLevelPackRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SelectLevelPackRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetCountdownEndTimeRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<i64>,
}
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc => ""
    ."MenuRpcManager/SetCountdownEndTimeRpc"
);
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetCountdownEndTimeRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetCountdownEndTimeRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetIsEntitledToLevelRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_2<
        *mut crate::System::String,
        i32,
    >,
}
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc => ""
    ."MenuRpcManager/SetIsEntitledToLevelRpc"
);
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_2<
        *mut crate::System::String,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetIsEntitledToLevelRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetIsEntitledToLevelRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetIsInLobbyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<bool>,
}
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc
    => ""."MenuRpcManager/SetIsInLobbyRpc"
);
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetIsInLobbyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetIsInLobbyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetIsReadyRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<bool>,
}
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_SetIsReadyRpc =>
    ""."MenuRpcManager/SetIsReadyRpc"
);
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetIsReadyRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<bool>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_SetIsReadyRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetIsReadyRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetIsReadyRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetIsReadyRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetIsStartButtonEnabledRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::CannotStartGameReason,
    >,
}
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc => ""
    ."MenuRpcManager/SetIsStartButtonEnabledRpc"
);
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::CannotStartGameReason,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetIsStartButtonEnabledRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetIsStartButtonEnabledRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetMultiplayerGameStateRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::MultiplayerGameState,
    >,
}
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc => ""
    ."MenuRpcManager/SetMultiplayerGameStateRpc"
);
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::MultiplayerGameState,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetMultiplayerGameStateRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetMultiplayerGameStateRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetOwnedSongPacksRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::SongPackMask,
    >,
}
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc => ""
    ."MenuRpcManager/SetOwnedSongPacksRpc"
);
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        crate::GlobalNamespace::SongPackMask,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetOwnedSongPacksRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetOwnedSongPacksRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    >,
}
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc => ""
    ."MenuRpcManager/SetPlayersMissingEntitlementsToLevelRpc"
);
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetPlayersMissingEntitlementsToLevelRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetPlayersMissingEntitlementsToLevelRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetPlayersPermissionConfigurationRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    >,
}
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc => ""
    ."MenuRpcManager/SetPlayersPermissionConfigurationRpc"
);
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetPlayersPermissionConfigurationRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetPlayersPermissionConfigurationRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetSelectedBeatmapRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >,
}
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc => ""
    ."MenuRpcManager/SetSelectedBeatmapRpc"
);
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetSelectedBeatmapRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetSelectedBeatmapRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetSelectedGameplayModifiersRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
}
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc => ""
    ."MenuRpcManager/SetSelectedGameplayModifiersRpc"
);
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
impl std::ops::Deref
for crate::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetSelectedGameplayModifiersRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetSelectedGameplayModifiersRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_SetStartGameTimeRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_1<i64>,
}
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc => ""
    ."MenuRpcManager/SetStartGameTimeRpc"
);
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_1<i64>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
impl crate::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+SetStartGameTimeRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_SetStartGameTimeRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
#[repr(C)]
#[derive(Debug)]
pub struct MenuRpcManager_StartLevelRpc {
    __cordl_parent: crate::GlobalNamespace::RemoteProcedureCall_3<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        *mut crate::GlobalNamespace::GameplayModifiers,
        i64,
    >,
}
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MenuRpcManager_StartLevelRpc =>
    ""."MenuRpcManager/StartLevelRpc"
);
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
impl std::ops::Deref for crate::GlobalNamespace::MenuRpcManager_StartLevelRpc {
    type Target = crate::GlobalNamespace::RemoteProcedureCall_3<
        *mut crate::GlobalNamespace::BeatmapKeyNetSerializable,
        *mut crate::GlobalNamespace::GameplayModifiers,
        i64,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
impl std::ops::DerefMut for crate::GlobalNamespace::MenuRpcManager_StartLevelRpc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
impl crate::GlobalNamespace::MenuRpcManager_StartLevelRpc {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MenuRpcManager+StartLevelRpc")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MenuRpcManager_StartLevelRpc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
