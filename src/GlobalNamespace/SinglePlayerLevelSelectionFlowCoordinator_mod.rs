#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct SinglePlayerLevelSelectionFlowCoordinator {
    __cordl_parent: LevelSelectionFlowCoordinator,
    pub _degree360BeatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub _practiceViewController: *mut PracticeViewController,
    pub _gameplaySetupViewController: *mut GameplaySetupViewController,
    pub _menuTransitionsHelper: *mut MenuTransitionsHelper,
    pub _vrPlatformHelper: *mut IVRPlatformHelper,
    pub _appStaticSettings: *mut AppStaticSettingsSO,
    pub _safeAreaFocusedSimpleDialogPromptViewController: *mut SafeAreaFocusedSimpleDialogPromptViewController,
    pub _environmentsListModel: *mut EnvironmentsListModel,
    pub didFinishEvent: *mut crate::System::Action_1<
        *mut SinglePlayerLevelSelectionFlowCoordinator,
    >,
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SinglePlayerLevelSelectionFlowCoordinator => ""
    ."SinglePlayerLevelSelectionFlowCoordinator"
);
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl std::ops::Deref for SinglePlayerLevelSelectionFlowCoordinator {
    type Target = LevelSelectionFlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl std::ops::DerefMut for SinglePlayerLevelSelectionFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl SinglePlayerLevelSelectionFlowCoordinator {
    #[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator+__c__DisplayClass38_0")]
    pub type __c__DisplayClass38_0 = crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator___c__DisplayClass38_0;
    pub fn ActionButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActionButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackButtonWasPressed", (topViewController))?;
        Ok(__cordl_ret)
    }
    pub fn DismissPracticeViewController(
        &mut self,
        finishedCallback: *mut crate::System::Action,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DismissPracticeViewController", (finishedCallback, immediately))?;
        Ok(__cordl_ret)
    }
    pub fn HandleBasicLevelCompletionResults(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "HandleBasicLevelCompletionResults",
                (levelCompletionResults, practice),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandlePracticeViewControllerDidPressPlayButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePracticeViewControllerDidPressPlayButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleStandardLevelDidFinish(
        &mut self,
        standardLevelScenesTransitionSetupData: *mut StandardLevelScenesTransitionSetupDataSO,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleStandardLevelDidFinish",
                (standardLevelScenesTransitionSetupData, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LevelSelectionFlowCoordinatorDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorDidActivate",
                (firstActivation, addedToHierarchy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LevelSelectionFlowCoordinatorDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorDidDeactivate",
                (removedFromHierarchy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LevelSelectionFlowCoordinatorTopViewControllerWillChange(
        &mut self,
        oldViewController: *mut crate::HMUI::ViewController,
        newViewController: *mut crate::HMUI::ViewController,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LevelSelectionFlowCoordinatorTopViewControllerWillChange",
                (oldViewController, newViewController, animationType),
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
    pub fn PracticeButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PracticeButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessLevelCompletionResultsAfterLevelDidFinish(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        transformedBeatmapData: *mut IReadonlyBeatmapData,
        beatmapKey: BeatmapKey,
        beatmapLevel: *mut BeatmapLevel,
        gameplayModifiers: *mut GameplayModifiers,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessLevelCompletionResultsAfterLevelDidFinish",
                (
                    levelCompletionResults,
                    transformedBeatmapData,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    practice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SelectionDidChange(
        &mut self,
        pack: *mut BeatmapLevelPack,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (pack, beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn SetupGameplaySetupViewController(
        &mut self,
        showModifiers: bool,
        showEnvironmentOverrideSettings: bool,
        showColorSchemesSettings: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetupGameplaySetupViewController",
                (
                    showModifiers,
                    showEnvironmentOverrideSettings,
                    showColorSchemesSettings,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SinglePlayerLevelSelectionFlowCoordinatorDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SinglePlayerLevelSelectionFlowCoordinatorDidActivate",
                (firstActivation, addedToHierarchy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SinglePlayerLevelSelectionFlowCoordinatorDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SinglePlayerLevelSelectionFlowCoordinatorDidDeactivate",
                (removedFromHierarchy),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartLevel(
        &mut self,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLevel", (beforeSceneSwitchCallback, practice))?;
        Ok(__cordl_ret)
    }
    pub fn StartLevelOrShow360Prompt(
        &mut self,
        beforeSceneSwitchCallback: *mut crate::System::Action,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartLevelOrShow360Prompt", (beforeSceneSwitchCallback, practice))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut SinglePlayerLevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_enableCustomLevels(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableCustomLevels", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_gameMode", ())?;
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
    pub fn get_hideGameplaySetup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hideGameplaySetup", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_initialLeftScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_initialLeftScreenViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_initialTopScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::ViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::ViewController = __cordl_object
            .invoke("get_initialTopScreenViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInPracticeView(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInPracticeView", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leaderboardViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LeaderboardViewController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LeaderboardViewController = __cordl_object
            .invoke("get_leaderboardViewController", ())?;
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
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut SinglePlayerLevelSelectionFlowCoordinator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType for SinglePlayerLevelSelectionFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}