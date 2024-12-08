#[cfg(feature = "LobbySetupViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbySetupViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _startGameReadyButton: *mut crate::UnityEngine::UI::Button,
    pub _cancelGameUnreadyButton: *mut crate::UnityEngine::UI::Button,
    pub _startReadyText: *mut crate::BGLib::Polyglot::LocalizedTextMeshProUGUI,
    pub _cancelUnreadyText: *mut crate::BGLib::Polyglot::LocalizedTextMeshProUGUI,
    pub _serverSettings: *mut GameServersFilterText,
    pub _suggestionHeader: *mut crate::UnityEngine::GameObject,
    pub _beatmapSelectionView: *mut EditableBeatmapSelectionView,
    pub _modifiersSelectionView: *mut EditableModifiersSelectionView,
    pub _cantStartGameHoverHint: *mut crate::HMUI::HoverHint,
    pub _playerMissingLevelHoverHintWrapper: *mut crate::UnityEngine::GameObject,
    pub _playersMissingLevelHoverHint: *mut crate::HMUI::HoverHint,
    pub _spectatorWarningTextWrapper: *mut crate::UnityEngine::GameObject,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub selectBeatmapEvent: *mut crate::System::Action,
    pub selectModifiersEvent: *mut crate::System::Action,
    pub startGameOrReadyEvent: *mut crate::System::Action,
    pub cancelGameOrUnreadyEvent: *mut crate::System::Action,
    pub clearSuggestedBeatmapEvent: *mut crate::System::Action,
    pub clearSuggestedModifiersEvent: *mut crate::System::Action,
    pub _isPartyOwner: bool,
    pub _isQuickStart: bool,
}
#[cfg(feature = "LobbySetupViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LobbySetupViewController => ""
    ."LobbySetupViewController"
);
#[cfg(feature = "LobbySetupViewController")]
impl std::ops::Deref for LobbySetupViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl std::ops::DerefMut for LobbySetupViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl LobbySetupViewController {
    pub const kCancelTextKey: &'static str = "BUTTON_CANCEL";
    pub const kReadyTextKey: &'static str = "LOBBY_READY";
    pub const kRetryTextKey: &'static str = "BUTTON_RETRY";
    pub const kStartTextKey: &'static str = "LOBBY_START_GAME";
    pub const kUnreadyTextKey: &'static str = "BUTTON_UNREADY";
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLobbyPlayerData(
        &mut self,
        lobbyPlayerData: *mut ILobbyPlayerData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyPlayerData", (lobbyPlayerData))?;
        Ok(__cordl_ret)
    }
    pub fn SetLobbyState(
        &mut self,
        lobbyState: MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyState", (lobbyState))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayerActiveState(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerActiveState", (isActive))?;
        Ok(__cordl_ret)
    }
    pub fn SetPlayersMissingLevelText(
        &mut self,
        playersMissingLevelText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayersMissingLevelText", (playersMissingLevelText))?;
        Ok(__cordl_ret)
    }
    pub fn SetStartGameEnabled(
        &mut self,
        cannotStartGameReason: CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartGameEnabled", (cannotStartGameReason))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        selectionMask: BeatmapLevelSelectionMask,
        isPartyOwner: bool,
        allowSongSelection: bool,
        allowModifierSelection: bool,
        isManaged: bool,
        isQuickStart: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Setup",
                (
                    selectionMask,
                    isPartyOwner,
                    allowSongSelection,
                    allowModifierSelection,
                    isManaged,
                    isQuickStart,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_3", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_4", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__44_5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_5", ())?;
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
    pub fn add_cancelGameOrUnreadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelGameOrUnreadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearSuggestedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_clearSuggestedModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSuggestedModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startGameOrReadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_cancelGameOrUnreadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelGameOrUnreadyEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearSuggestedBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_clearSuggestedModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSuggestedModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_startGameOrReadyEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl quest_hook::libil2cpp::ObjectType for LobbySetupViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}