#[cfg(feature = "LobbySetupViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LobbySetupViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _startGameReadyButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _cancelGameUnreadyButton: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Button,
    >,
    pub _startReadyText: quest_hook::libil2cpp::Gc<
        crate::BGLib::Polyglot::LocalizedTextMeshProUGUI,
    >,
    pub _cancelUnreadyText: quest_hook::libil2cpp::Gc<
        crate::BGLib::Polyglot::LocalizedTextMeshProUGUI,
    >,
    pub _serverSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServersFilterText,
    >,
    pub _suggestionHeader: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _beatmapSelectionView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EditableBeatmapSelectionView,
    >,
    pub _modifiersSelectionView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EditableModifiersSelectionView,
    >,
    pub _cantStartGameHoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    pub _playerMissingLevelHoverHintWrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _playersMissingLevelHoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    pub _spectatorWarningTextWrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _toggleBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleBinder>,
    pub selectBeatmapEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub selectModifiersEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub startGameOrReadyEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub cancelGameOrUnreadyEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub clearSuggestedBeatmapEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub clearSuggestedModifiersEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _isPartyOwner: bool,
    pub _isQuickStart: bool,
}
#[cfg(feature = "LobbySetupViewController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LobbySetupViewController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LobbySetupViewController";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl std::ops::Deref for crate::GlobalNamespace::LobbySetupViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LobbySetupViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl crate::GlobalNamespace::LobbySetupViewController {
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyPlayerData(
        &mut self,
        lobbyPlayerData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILobbyPlayerData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyPlayerData", (lobbyPlayerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbyState(
        &mut self,
        lobbyState: crate::GlobalNamespace::MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLobbyState", (lobbyState))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetPlayersMissingLevelText(
        &mut self,
        playersMissingLevelText: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayersMissingLevelText", (playersMissingLevelText))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetReadyButtonActive(
        &mut self,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReadyButtonActive", (active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStartGameEnabled(
        &mut self,
        cannotStartGameReason: crate::GlobalNamespace::CannotStartGameReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartGameEnabled", (cannotStartGameReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
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
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__44_5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__44_5", ())?;
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
    pub fn add_cancelGameOrUnreadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_cancelGameOrUnreadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_clearSuggestedBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_clearSuggestedModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_clearSuggestedModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_cancelGameOrUnreadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_cancelGameOrUnreadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_clearSuggestedBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSuggestedBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_clearSuggestedModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_clearSuggestedModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_startGameOrReadyEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startGameOrReadyEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LobbySetupViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LobbySetupViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
