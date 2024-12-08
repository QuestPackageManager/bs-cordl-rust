#[cfg(feature = "LocalLeaderboardViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardViewController {
    __cordl_parent: LeaderboardViewController,
    pub _maxNumberOfCells: i32,
    pub _leaderboardTableView: *mut LocalLeaderboardTableView,
    pub _clearLeaderboardsWrapper: *mut crate::UnityEngine::GameObject,
    pub _clearLeaderboardsButton: *mut crate::HMUI::NoTransitionsButton,
    pub _scopeSegmentedControl: *mut crate::HMUI::IconSegmentedControl,
    pub _allTimeLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _todayLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _clearLeaderboardIcon: *mut crate::UnityEngine::Sprite,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _localLeaderboardsModel: *mut LocalLeaderboardsModel,
    pub _beatmapKey: BeatmapKey,
    pub _refreshIsNeeded: bool,
    pub _enableClear: bool,
}
#[cfg(feature = "LocalLeaderboardViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LocalLeaderboardViewController => ""
    ."LocalLeaderboardViewController"
);
#[cfg(feature = "LocalLeaderboardViewController")]
impl std::ops::Deref for LocalLeaderboardViewController {
    type Target = LeaderboardViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardViewController")]
impl std::ops::DerefMut for LocalLeaderboardViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardViewController")]
impl LocalLeaderboardViewController {
    #[cfg(feature = "LocalLeaderboardViewController+_ClearLeaderboardsAsync_d__24")]
    pub type _ClearLeaderboardsAsync_d__24 = crate::GlobalNamespace::LocalLeaderboardViewController__ClearLeaderboardsAsync_d__24;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
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
    pub fn RefreshScopeSegmentedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshScopeSegmentedControl", ())?;
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
    pub fn Setup(
        &mut self,
        enableClear: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (enableClear))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_leaderboardsModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LocalLeaderboardsModel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LocalLeaderboardsModel = __cordl_object
            .invoke("get_leaderboardsModel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_0", ())?;
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
    pub fn ClearLeaderboardsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("ClearLeaderboardsAsync", ())?;
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
    pub fn SetContent(
        &mut self,
        leaderboardID: *mut crate::System::String,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContent", (leaderboardID, leaderboardType))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNewScoreWasAddedToLeaderboard(
        &mut self,
        leaderboardID: *mut crate::System::String,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNewScoreWasAddedToLeaderboard",
                (leaderboardID, leaderboardType),
            )?;
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
#[cfg(feature = "LocalLeaderboardViewController")]
impl quest_hook::libil2cpp::ObjectType for LocalLeaderboardViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
