#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct SinglePlayerLevelSelectionFlowCoordinator {
    __cordl_parent: crate::GlobalNamespace::LevelSelectionFlowCoordinator,
    pub _degree360BeatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _practiceViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PracticeViewController,
    >,
    pub _gameplaySetupViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplaySetupViewController,
    >,
    pub _menuTransitionsHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MenuTransitionsHelper,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _appStaticSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppStaticSettingsSO,
    >,
    pub _customLevelsSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CustomLevelsSettings,
    >,
    pub _safeAreaFocusedSimpleDialogPromptViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SafeAreaFocusedSimpleDialogPromptViewController,
    >,
    pub _environmentsListModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentsListModel,
    >,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
            >,
        >,
    >,
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SinglePlayerLevelSelectionFlowCoordinator";
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
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl std::ops::Deref
for crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator {
    type Target = crate::GlobalNamespace::LevelSelectionFlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator {
    pub fn ActionButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ActionButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ActionButtonWasPressed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn BackButtonWasPressed(
        &mut self,
        topViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BackButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BackButtonWasPressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (topViewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DismissPracticeViewController(
        &mut self,
        finishedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DismissPracticeViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DismissPracticeViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (finishedCallback, immediately))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleBasicLevelCompletionResults(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::LevelCompletionResults,
                    >,
                    bool,
                ),
                bool,
                2usize,
            >("HandleBasicLevelCompletionResults")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleBasicLevelCompletionResults", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (levelCompletionResults, practice))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePracticeViewControllerDidPressPlayButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandlePracticeViewControllerDidPressPlayButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandlePracticeViewControllerDidPressPlayButton", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleStandardLevelDidFinish(
        &mut self,
        standardLevelScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        >,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::LevelCompletionResults,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleStandardLevelDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleStandardLevelDidFinish", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (standardLevelScenesTransitionSetupData, levelCompletionResults),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LevelSelectionFlowCoordinatorDidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LevelSelectionFlowCoordinatorDidActivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (firstActivation, addedToHierarchy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LevelSelectionFlowCoordinatorDidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LevelSelectionFlowCoordinatorDidDeactivate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LevelSelectionFlowCoordinatorTopViewControllerWillChange(
        &mut self,
        oldViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        newViewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        animationType: crate::HMUI::ViewController_AnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                    crate::HMUI::ViewController_AnimationType,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("LevelSelectionFlowCoordinatorTopViewControllerWillChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LevelSelectionFlowCoordinatorTopViewControllerWillChange",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (oldViewController, newViewController, animationType),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PracticeButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("PracticeButtonWasPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PracticeButtonWasPressed", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLevelCompletionResultsAfterLevelDidFinish(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        transformedBeatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::LevelCompletionResults,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IReadonlyBeatmapData,
                    >,
                    crate::GlobalNamespace::BeatmapKey,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("ProcessLevelCompletionResultsAfterLevelDidFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessLevelCompletionResultsAfterLevelDidFinish", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        levelCompletionResults,
                        transformedBeatmapData,
                        beatmapKey,
                        beatmapLevel,
                        gameplayModifiers,
                        practice,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectionDidChange(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SelectionDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SelectionDidChange", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pack, beatmapKey))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupGameplaySetupViewController(
        &mut self,
        showModifiers: bool,
        showEnvironmentOverrideSettings: bool,
        showColorSchemesSettings: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetupGameplaySetupViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetupGameplaySetupViewController", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        showModifiers,
                        showEnvironmentOverrideSettings,
                        showColorSchemesSettings,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SinglePlayerLevelSelectionFlowCoordinatorDidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SinglePlayerLevelSelectionFlowCoordinatorDidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SinglePlayerLevelSelectionFlowCoordinatorDidActivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (firstActivation, addedToHierarchy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SinglePlayerLevelSelectionFlowCoordinatorDidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SinglePlayerLevelSelectionFlowCoordinatorDidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SinglePlayerLevelSelectionFlowCoordinatorDidDeactivate",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartLevel(
        &mut self,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("StartLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartLevel", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beforeSceneSwitchCallback, practice))
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartLevelOrShow360Prompt(
        &mut self,
        beforeSceneSwitchCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("StartLevelOrShow360Prompt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "StartLevelOrShow360Prompt", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beforeSceneSwitchCallback, practice))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_enableCustomLevels(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_enableCustomLevels")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_enableCustomLevels", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_gameMode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_gameMode", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                0usize,
            >("get_gameplayModifiers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_gameplayModifiers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_hideGameplaySetup(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hideGameplaySetup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_hideGameplaySetup", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_initialLeftScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                0usize,
            >("get_initialLeftScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_initialLeftScreenViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_initialTopScreenViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
                0usize,
            >("get_initialTopScreenViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_initialTopScreenViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isInPracticeView(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isInPracticeView")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isInPracticeView", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_leaderboardViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardViewController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LeaderboardViewController,
                >,
                0usize,
            >("get_leaderboardViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_leaderboardViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardViewController,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerSpecificSettings,
                >,
                0usize,
            >("get_playerSettings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_playerSettings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SinglePlayerLevelSelectionFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
