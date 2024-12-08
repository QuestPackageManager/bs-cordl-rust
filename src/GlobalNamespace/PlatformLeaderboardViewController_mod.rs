#[cfg(feature = "PlatformLeaderboardViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardViewController {
    __cordl_parent: crate::GlobalNamespace::LeaderboardViewController,
    pub _leaderboardTableView: *mut crate::GlobalNamespace::LeaderboardTableView,
    pub _scopeSegmentedControl: *mut crate::HMUI::IconSegmentedControl,
    pub _loadingControl: *mut crate::GlobalNamespace::LoadingControl,
    pub _globalLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _aroundPlayerLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _friendsLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _levelStatsView: *mut crate::GlobalNamespace::LevelStatsView,
    pub _leaderboardsModel: *mut crate::GlobalNamespace::PlatformLeaderboardsModel,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _getScoresAsyncRequest: *mut crate::GlobalNamespace::HMAsyncRequest,
    pub _scores: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardTableView_ScoreData,
    >,
    pub _playerScorePos: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _refreshIsNeeded: bool,
    pub _hasScoresData: bool,
    pub _scoreScopes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
    >,
}
#[cfg(feature = "PlatformLeaderboardViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlatformLeaderboardViewController => ""
    ."PlatformLeaderboardViewController"
);
#[cfg(feature = "PlatformLeaderboardViewController")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformLeaderboardViewController {
    type Target = crate::GlobalNamespace::LeaderboardViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformLeaderboardViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardViewController")]
impl crate::GlobalNamespace::PlatformLeaderboardViewController {
    pub const kMaxLeaderboardResults: i32 = 10i32;
    #[cfg(feature = "PlatformLeaderboardViewController+_RefreshDelayed_d__33")]
    pub type _RefreshDelayed_d__33 = crate::GlobalNamespace::PlatformLeaderboardViewController__RefreshDelayed_d__33;
    #[cfg(feature = "PlatformLeaderboardViewController+_Refresh_d__31")]
    pub type _Refresh_d__31 = crate::GlobalNamespace::PlatformLeaderboardViewController__Refresh_d__31;
    pub fn ClearContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearContent", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn HandleDidPressRefreshButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPressRefreshButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLeaderboardsResultsReturned(
        &mut self,
        result: crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult,
        scores: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
        playerScoreIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLeaderboardsResultsReturned",
                (result, scores, playerScoreIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlatformLeaderboardsModelAllScoresDidUpload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlatformLeaderboardsModelAllScoresDidUpload", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleScopeSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleScopeSegmentedControlDidSelectCell",
                (segmentedControl, cellNumber),
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
    pub fn Refresh(
        &mut self,
        showLoadingIndicator: bool,
        clear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (showLoadingIndicator, clear))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshAsync(
        &mut self,
        showLoadingIndicator: bool,
        clear: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RefreshAsync", (showLoadingIndicator, clear))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshDelayed(
        &mut self,
        showLoadingIndicator: bool,
        clear: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("RefreshDelayed", (showLoadingIndicator, clear))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshLevelStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshLevelStats", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScopeScopeIndexToScoreScope(
        &mut self,
        scoreScopeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope = __cordl_object
            .invoke("ScopeScopeIndexToScoreScope", (scoreScopeIndex))?;
        Ok(__cordl_ret)
    }
    pub fn ScoreScopeToScoreScopeIndex(
        &mut self,
        scoresScope: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ScoreScopeToScoreScopeIndex", (scoresScope))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (beatmapKey))?;
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
    pub fn get_leaderboardsModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlatformLeaderboardsModel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlatformLeaderboardsModel = __cordl_object
            .invoke("get_leaderboardsModel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlatformLeaderboardViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
