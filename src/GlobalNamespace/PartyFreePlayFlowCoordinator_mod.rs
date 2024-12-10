#[cfg(feature = "PartyFreePlayFlowCoordinator")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyFreePlayFlowCoordinator {
    __cordl_parent: crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator,
    pub _defaultLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _resultsClearedLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _resultsFailedLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _menuLightsManager: *mut crate::GlobalNamespace::MenuLightsManager,
    pub _resultsViewController: *mut crate::GlobalNamespace::ResultsViewController,
    pub _localLeaderboardViewController: *mut crate::GlobalNamespace::LocalLeaderboardViewController,
    pub _enterNameViewController: *mut crate::GlobalNamespace::EnterPlayerGuestNameViewController,
    pub _fileStorage: *mut crate::GlobalNamespace::IFileStorage,
}
#[cfg(feature = "PartyFreePlayFlowCoordinator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PartyFreePlayFlowCoordinator =>
    ""."PartyFreePlayFlowCoordinator"
);
#[cfg(feature = "PartyFreePlayFlowCoordinator")]
impl std::ops::Deref for crate::GlobalNamespace::PartyFreePlayFlowCoordinator {
    type Target = crate::GlobalNamespace::SinglePlayerLevelSelectionFlowCoordinator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PartyFreePlayFlowCoordinator")]
impl std::ops::DerefMut for crate::GlobalNamespace::PartyFreePlayFlowCoordinator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PartyFreePlayFlowCoordinator")]
impl crate::GlobalNamespace::PartyFreePlayFlowCoordinator {
    #[cfg(feature = "PartyFreePlayFlowCoordinator+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::GlobalNamespace::PartyFreePlayFlowCoordinator___c__DisplayClass18_0;
    #[cfg(feature = "PartyFreePlayFlowCoordinator+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::GlobalNamespace::PartyFreePlayFlowCoordinator___c__DisplayClass23_0;
    pub fn HandleResultsViewControllerContinueButtonPressed(
        &mut self,
        resultsViewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleResultsViewControllerContinueButtonPressed",
                (resultsViewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleResultsViewControllerRestartButtonPressed(
        &mut self,
        resultsViewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleResultsViewControllerRestartButtonPressed",
                (resultsViewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNewHighScore(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsNewHighScore", (levelCompletionResults, leaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessScore(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ProcessScore",
                (levelCompletionResults, leaderboardId, playerName),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn WillScoreGoToLeaderboard(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        practice: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "WillScoreGoToLeaderboard",
                (levelCompletionResults, leaderboardId, practice),
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
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_gameMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leaderboardViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LeaderboardViewController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardViewController,
        > = __cordl_object.invoke("get_leaderboardViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mainTitle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_mainTitle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showBackButtonForMainViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_showBackButtonForMainViewController", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PartyFreePlayFlowCoordinator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PartyFreePlayFlowCoordinator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
