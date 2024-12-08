#[cfg(feature = "GameplaySetupViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplaySetupViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _selectionSegmentedControl: *mut crate::HMUI::TextSegmentedControl,
    pub _playerSettingsPanelController: *mut PlayerSettingsPanelController,
    pub _gameplayModifiersPanelController: *mut GameplayModifiersPanelController,
    pub _environmentOverrideSettingsPanelController: *mut EnvironmentOverrideSettingsPanelController,
    pub _colorsOverrideSettingsPanelController: *mut ColorsOverrideSettingsPanelController,
    pub _multiplayerSettingsPanelController: *mut MultiplayerSettingsPanelController,
    pub _playerDataModel: *mut PlayerDataModel,
    pub didChangeGameplayModifiersEvent: *mut crate::System::Action,
    pub _panels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::GameplaySetupViewController_Panel,
    >,
    pub _activePanelIdx: i32,
    pub _showModifiers: bool,
    pub _showEnvironmentOverrideSettings: bool,
    pub _showColorSchemesSettings: bool,
    pub _showMultiplayer: bool,
    pub _shouldRefreshContent: bool,
    pub _isInitialized: bool,
}
#[cfg(feature = "GameplaySetupViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameplaySetupViewController => ""
    ."GameplaySetupViewController"
);
#[cfg(feature = "GameplaySetupViewController")]
impl std::ops::Deref for GameplaySetupViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl std::ops::DerefMut for GameplaySetupViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl GameplaySetupViewController {
    #[cfg(feature = "GameplaySetupViewController+Panel")]
    pub type Panel = crate::GlobalNamespace::GameplaySetupViewController_Panel;
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
    pub fn remove_didChangeGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentOverrideSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OverrideEnvironmentSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OverrideEnvironmentSettings = __cordl_object
            .invoke("get_environmentOverrideSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerSettingsPanelControllerDidChangePlayerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSettingsPanelControllerDidChangePlayerSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetActivePanel(
        &mut self,
        panelIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActivePanel", (panelIdx))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectionSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectionSegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshActivePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshActivePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didChangeGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        showModifiers: bool,
        showEnvironmentOverrideSettings: bool,
        showColorSchemesSettings: bool,
        showMultiplayer: bool,
        playerSettingsPanelLayout: crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Setup",
                (
                    showModifiers,
                    showEnvironmentOverrideSettings,
                    showColorSchemesSettings,
                    showMultiplayer,
                    playerSettingsPanelLayout,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_colorSchemesSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorSchemesSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorSchemesSettings = __cordl_object
            .invoke("get_colorSchemesSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameplayModifiersPanelControllerDidChangeGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameplayModifiersPanelControllerDidChangeGameplayModifiers",
                (),
            )?;
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
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl quest_hook::libil2cpp::ObjectType for GameplaySetupViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplaySetupViewController_Panel {
    __cordl_parent: crate::System::Object,
    pub title: *mut crate::System::String,
    pub refreshable: *mut IRefreshable,
    pub gameObject: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplaySetupViewController_Panel => ""
    ."GameplaySetupViewController/Panel"
);
#[cfg(feature = "GameplaySetupViewController+Panel")]
impl std::ops::Deref for crate::GlobalNamespace::GameplaySetupViewController_Panel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplaySetupViewController_Panel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
impl crate::GlobalNamespace::GameplaySetupViewController_Panel {
    pub fn _ctor(
        &mut self,
        title: *mut crate::System::String,
        refreshable: *mut IRefreshable,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (title, refreshable, gameObject))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        title: *mut crate::System::String,
        refreshable: *mut IRefreshable,
        gameObject: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (title, refreshable, gameObject))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplaySetupViewController_Panel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
