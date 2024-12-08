#[cfg(feature = "ServerPlayerListViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ServerPlayerListViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _gameServerPlayersTableView: *mut crate::GlobalNamespace::GameServerPlayersTableView,
    pub _invitePlayerButton: *mut crate::UnityEngine::UI::Button,
    pub _cantInvitePlayerHoverHint: *mut crate::HMUI::HoverHint,
    pub _invitePlatformHandler: *mut crate::GlobalNamespace::IInvitePlatformHandler,
    pub _lobbyPlayersDataModel: *mut crate::GlobalNamespace::ILobbyPlayersDataModel,
    pub _lobbyStateDataModel: *mut crate::GlobalNamespace::ILobbyStateDataModel,
    pub _lobbyPlayerPermissionsModel: *mut crate::GlobalNamespace::LobbyPlayerPermissionsModel,
    pub _lobbyGameStateController: *mut crate::GlobalNamespace::ILobbyGameStateController,
    pub selectSuggestedBeatmapEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::BeatmapKey,
    >,
    pub selectSuggestedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
    pub kickPlayerEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub didOpenInvitePanelEvent: *mut crate::System::Action,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
}
#[cfg(feature = "ServerPlayerListViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ServerPlayerListViewController
    => ""."ServerPlayerListViewController"
);
#[cfg(feature = "ServerPlayerListViewController")]
impl std::ops::Deref for crate::GlobalNamespace::ServerPlayerListViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ServerPlayerListViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::ServerPlayerListViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ServerPlayerListViewController")]
impl crate::GlobalNamespace::ServerPlayerListViewController {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn HandleKickPlayer(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleKickPlayer", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyGameStateControllerLobbyStateChanged(
        &mut self,
        _: crate::GlobalNamespace::MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyGameStateControllerLobbyStateChanged", (_))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyPlayerPermissionChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayerPermissionChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLobbyPlayersDataDidChange(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLobbyPlayersDataDidChange", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleOpenPlatformInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOpenPlatformInvitePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectSuggestedGameplayModifiers(
        &mut self,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectSuggestedGameplayModifiers", (gameplayModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectSuggestedLevel(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelectSuggestedLevel", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetDataToTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataToTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn TrySetInviteButtonEnabled(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrySetInviteButtonEnabled", ())?;
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
    pub fn add_didOpenInvitePanelEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectSuggestedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didOpenInvitePanelEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenInvitePanelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectSuggestedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ServerPlayerListViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ServerPlayerListViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
