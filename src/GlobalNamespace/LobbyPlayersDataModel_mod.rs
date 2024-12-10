#[cfg(feature = "LobbyPlayersDataModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbyPlayersDataModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _menuRpcManager: *mut crate::GlobalNamespace::IMenuRpcManager,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _emptyLobbyPlayerData: *mut crate::GlobalNamespace::LobbyPlayerData,
    pub _playersData: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::LobbyPlayerData,
    >,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _partyOwnerId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub didChangeEvent: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "LobbyPlayersDataModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LobbyPlayersDataModel => ""
    ."LobbyPlayersDataModel"
);
#[cfg(feature = "LobbyPlayersDataModel")]
impl std::ops::Deref for crate::GlobalNamespace::LobbyPlayersDataModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl crate::GlobalNamespace::LobbyPlayersDataModel {
    #[cfg(feature = "LobbyPlayersDataModel+_GetEnumerator_d__59")]
    pub type _GetEnumerator_d__59 = crate::GlobalNamespace::LobbyPlayersDataModel__GetEnumerator_d__59;
    #[cfg(feature = "LobbyPlayersDataModel+_SetOwnedSongPacks_d__42")]
    pub type _SetOwnedSongPacks_d__42 = crate::GlobalNamespace::LobbyPlayersDataModel__SetOwnedSongPacks_d__42;
    #[cfg(feature = "LobbyPlayersDataModel+__c")]
    pub type __c = crate::GlobalNamespace::LobbyPlayersDataModel___c;
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLocalPlayerBeatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLocalPlayerBeatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLocalPlayerGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLocalPlayerGameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearRecommendations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRecommendations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::GlobalNamespace::ILobbyPlayerData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    *mut quest_hook::libil2cpp::Il2CppString,
                    *mut crate::GlobalNamespace::ILobbyPlayerData,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateLobbyPlayerDataModel(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        alreadyExists: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LobbyPlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LobbyPlayerData,
        > = __cordl_object
            .invoke("GetOrCreateLobbyPlayerDataModel", (userId, alreadyExists))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerClearBeatmap(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearBeatmap", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerClearRecommendedGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerClearRecommendedGameplayModifiers", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerGetIsInLobby(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerGetIsInLobby", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerGetIsReady(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerGetIsReady", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerGetOwnedSongPacks(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerGetOwnedSongPacks", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerGetRecommendedBeatmap(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerGetRecommendedBeatmap", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerGetRecommendedGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerGetRecommendedGameplayModifiers", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerRecommendBeatmap(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKeySerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapKeyNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerRecommendBeatmap",
                (userId, beatmapKeySerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerRecommendGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerRecommendGameplayModifiers",
                (userId, gameplayModifiers),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetIsInLobby(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isInLobby: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetIsInLobby", (userId, isInLobby))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetIsReady(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuRpcManagerSetIsReady", (userId, isReady))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMenuRpcManagerSetPlayersPermissionConfiguration(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersLobbyPermissionConfiguration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayersLobbyPermissionConfigurationNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMenuRpcManagerSetPlayersPermissionConfiguration",
                (userId, playersLobbyPermissionConfiguration),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSessionManagerPlayerConnected(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerPlayerConnected",
                (connectedPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSessionManagerPlayerDisconnected(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerPlayerDisconnected",
                (connectedPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleMultiplayerSessionManagerPlayerStateChanged(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMultiplayerSessionManagerPlayerStateChanged",
                (connectedPlayer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ILobbyPlayersDataModel_SetLocalPlayerBeatmapLevel(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ILobbyPlayersDataModel.SetLocalPlayerBeatmapLevel", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NotifyModelChange(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyModelChange", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestKickPlayer(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestKickPlayer", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerBeatmapLevel(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerBeatmapLevel", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerGameplayModifiers(
        &mut self,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerGameplayModifiers", (modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsActive__cordl_bool0(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsActive", (isActive))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsActive__cordl_bool1(
        &mut self,
        isActive: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsActive", (isActive, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsInLobby__cordl_bool0(
        &mut self,
        isInLobby: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsInLobby", (isInLobby))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsInLobby__cordl_bool1(
        &mut self,
        isInLobby: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsInLobby", (isInLobby, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsReady__cordl_bool0(
        &mut self,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsReady", (isReady))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsReady__cordl_bool1(
        &mut self,
        isReady: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsReady", (isReady, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetOwnedSongPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOwnedSongPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerBeatmapLevel(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerBeatmapLevel", (userId, beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerGameplayModifiers(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerGameplayModifiers", (userId, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerIsActive(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isActive: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerIsActive", (userId, isActive, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerIsInLobby(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isInLobby: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerIsInLobby", (userId, isInLobby, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerIsPartyOwner(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isPartyOwner: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerIsPartyOwner", (userId, isPartyOwner, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayerIsReady(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isReady: bool,
        notifyChange: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerIsReady", (userId, isReady, notifyChange))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (key, value))?;
        Ok(__cordl_ret.into())
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
    pub fn add_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILobbyPlayerData,
        > = __cordl_object.invoke("get_Item", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("get_Keys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILobbyPlayerData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::ILobbyPlayerData,
            >,
        > = __cordl_object.invoke("get_Values", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localUserId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_partyOwnerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_partyOwnerId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_partyOwnerId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_partyOwnerId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<crate::GlobalNamespace::ILobbyPlayersDataModel>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::ILobbyPlayersDataModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<crate::GlobalNamespace::ILobbyPlayersDataModel>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ILobbyPlayersDataModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyDictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::ILobbyPlayerData,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyDictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::ILobbyPlayerData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyDictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::ILobbyPlayerData,
    >,
> for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::ILobbyPlayerData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LobbyPlayersDataModel")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::LobbyPlayersDataModel {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
