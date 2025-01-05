#[cfg(feature = "LocalLeaderboardViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardViewController {
    __cordl_parent: crate::GlobalNamespace::LeaderboardViewController,
    pub _maxNumberOfCells: i32,
    pub _leaderboardTableView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalLeaderboardTableView,
    >,
    pub _clearLeaderboardsWrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _clearLeaderboardsButton: quest_hook::libil2cpp::Gc<
        crate::HMUI::NoTransitionsButton,
    >,
    pub _scopeSegmentedControl: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControl,
    >,
    pub _allTimeLeaderboardIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _todayLeaderboardIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _clearLeaderboardIcon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _localLeaderboardsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LocalLeaderboardsModel,
    >,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _refreshIsNeeded: bool,
    pub _enableClear: bool,
}
#[cfg(feature = "LocalLeaderboardViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalLeaderboardViewController
    => ""."LocalLeaderboardViewController"
);
#[cfg(feature = "LocalLeaderboardViewController")]
impl std::ops::Deref for crate::GlobalNamespace::LocalLeaderboardViewController {
    type Target = crate::GlobalNamespace::LeaderboardViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalLeaderboardViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardViewController")]
impl crate::GlobalNamespace::LocalLeaderboardViewController {
    pub fn ClearLeaderboardsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("ClearLeaderboardsAsync", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleNewScoreWasAddedToLeaderboard(
        &mut self,
        leaderboardID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleScopeSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
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
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshScopeSegmentedControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshScopeSegmentedControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContent(
        &mut self,
        leaderboardID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardType: crate::GlobalNamespace::LocalLeaderboardsModel_LeaderboardType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContent", (leaderboardID, leaderboardType))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__19_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__19_0", ())?;
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
    pub fn get_leaderboardsModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LocalLeaderboardsModel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LocalLeaderboardsModel,
        > = __cordl_object.invoke("get_leaderboardsModel", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalLeaderboardViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
