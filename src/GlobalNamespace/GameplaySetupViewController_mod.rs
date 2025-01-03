#[cfg(feature = "GameplaySetupViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplaySetupViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _selectionSegmentedControl: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControl,
    >,
    pub _playerSettingsPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSettingsPanelController,
    >,
    pub _gameplayModifiersPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersPanelController,
    >,
    pub _environmentOverrideSettingsPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentOverrideSettingsPanelController,
    >,
    pub _colorsOverrideSettingsPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorsOverrideSettingsPanelController,
    >,
    pub _multiplayerSettingsPanelController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSettingsPanelController,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _panels: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::GameplaySetupViewController_Panel,
        >,
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
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplaySetupViewController =>
    ""."GameplaySetupViewController"
);
#[cfg(feature = "GameplaySetupViewController")]
impl std::ops::Deref for crate::GlobalNamespace::GameplaySetupViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplaySetupViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl crate::GlobalNamespace::GameplaySetupViewController {
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerSettingsPanelControllerDidChangePlayerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSettingsPanelControllerDidChangePlayerSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectionSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshActivePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshActivePanel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetIsInteractable(
        &mut self,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsInteractable", (interactable))?;
        Ok(__cordl_ret.into())
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
    pub fn get_colorSchemesSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSchemesSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        > = __cordl_object.invoke("get_colorSchemesSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentOverrideSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OverrideEnvironmentSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        > = __cordl_object.invoke("get_environmentOverrideSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object.invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        > = __cordl_object.invoke("get_playerSettings", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplaySetupViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplaySetupViewController {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub refreshable: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRefreshable>,
    pub gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
}
#[cfg(feature = "GameplaySetupViewController+Panel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplaySetupViewController_Panel => ""
    ."GameplaySetupViewController/Panel"
);
#[cfg(feature = "GameplaySetupViewController+Panel")]
impl std::ops::Deref for crate::GlobalNamespace::GameplaySetupViewController_Panel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        refreshable: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRefreshable>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (title, refreshable, gameObject))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        refreshable: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IRefreshable>,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (title, refreshable, gameObject))?;
        Ok(__cordl_ret.into())
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
