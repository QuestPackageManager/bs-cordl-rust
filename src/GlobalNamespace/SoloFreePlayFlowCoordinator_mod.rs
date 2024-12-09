#[cfg(feature = "SoloFreePlayFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct SoloFreePlayFlowCoordinator {
    __cordl_parent: crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
    pub _defaultLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _resultsClearedLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _resultsFailedLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _menuLightsManager: *mut crate::GlobalNamespace::MenuLightsManager,
    pub _resultsViewController: *mut crate::GlobalNamespace::ResultsViewController,
    pub _platformLeaderboardViewController: *mut crate::GlobalNamespace::PlatformLeaderboardViewController,
    pub _platformLeaderboardsModel: *mut crate::GlobalNamespace::PlatformLeaderboardsModel,
}
#[cfg(feature = "SoloFreePlayFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SoloFreePlayFlowCoordinator =>
    ""."SoloFreePlayFlowCoordinator"
);
#[cfg(feature = "SoloFreePlayFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::SoloFreePlayFlowCoordinator {
    type Target = crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SoloFreePlayFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::SoloFreePlayFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SoloFreePlayFlowCoordinator")]
impl crate::GlobalNamespace::SoloFreePlayFlowCoordinator {
    #[cfg(feature = "SoloFreePlayFlowCoordinator+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::GlobalNamespace::SoloFreePlayFlowCoordinator___c__DisplayClass21_0;
    pub fn HandleResultsViewControllerContinueButtonPressed(
        &mut self,
        viewController: *mut crate::GlobalNamespace::ResultsViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleResultsViewControllerContinueButtonPressed",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleResultsViewControllerRestartButtonPressed(
        &mut self,
        resultsViewController: *mut crate::GlobalNamespace::ResultsViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleResultsViewControllerRestartButtonPressed",
                (resultsViewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IsNewHighScore(
        &mut self,
        playerLevelStats: *mut crate::GlobalNamespace::PlayerLevelStatsData,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNewHighScore", (playerLevelStats, levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessLevelCompletionResultsAfterLevelDidFinish(
        &mut self,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
        transformedBeatmapData: *mut crate::GlobalNamespace::IReadonlyBeatmapData,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        modifiers: *mut crate::GlobalNamespace::GameplayModifiers,
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
                    modifiers,
                    practice,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
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
    pub fn __SetupFromDestination(
        &mut self,
        runLevelMenuDestination: *mut crate::GlobalNamespace::RunLevelMenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__SetupFromDestination", (runLevelMenuDestination))?;
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
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_gameMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leaderboardViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::LeaderboardViewController,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::LeaderboardViewController = __cordl_object
            .invoke("get_leaderboardViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mainTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_mainTitle", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showBackButtonForMainViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_showBackButtonForMainViewController", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SoloFreePlayFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SoloFreePlayFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
