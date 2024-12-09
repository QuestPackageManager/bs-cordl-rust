#[cfg(feature = "PlayerStatisticsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerStatisticsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _statsScopeSegmentedControl: *mut crate::HMUI::TextSegmentedControl,
    pub _playedLevelsCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _clearedLevelsCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _failedLevelsCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _timePlayedText: *mut crate::TMPro::TextMeshProUGUI,
    pub _goodCutsCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _badCutsCountCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _missedCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _totalScoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _fullComboCountText: *mut crate::TMPro::TextMeshProUGUI,
    pub _handDistanceTravelledText: *mut crate::TMPro::TextMeshProUGUI,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _statsScopeDatas: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::PlayerStatisticsViewController_StatsScopeData,
    >,
}
#[cfg(feature = "PlayerStatisticsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerStatisticsViewController
    => ""."PlayerStatisticsViewController"
);
#[cfg(feature = "PlayerStatisticsViewController")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerStatisticsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerStatisticsViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerStatisticsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerStatisticsViewController")]
impl crate::GlobalNamespace::PlayerStatisticsViewController {
    #[cfg(feature = "PlayerStatisticsViewController+StatsScopeData")]
    pub type StatsScopeData = crate::GlobalNamespace::PlayerStatisticsViewController_StatsScopeData;
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
    pub fn HandleStatsScopeSegmentedControlDidSelectCell(
        &mut self,
        segmentedControl: *mut crate::HMUI::SegmentedControl,
        cellIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleStatsScopeSegmentedControlDidSelectCell",
                (segmentedControl, cellIdx),
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
    pub fn UpdateView(
        &mut self,
        playerOverallStatsData: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateView", (playerOverallStatsData))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__14_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData = __cordl_object
            .invoke("<DidActivate>b__14_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__14_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData = __cordl_object
            .invoke("<DidActivate>b__14_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__14_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData = __cordl_object
            .invoke("<DidActivate>b__14_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__14_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData = __cordl_object
            .invoke("<DidActivate>b__14_3", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__14_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData = __cordl_object
            .invoke("<DidActivate>b__14_4", ())?;
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
}
#[cfg(feature = "PlayerStatisticsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerStatisticsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerStatisticsViewController+StatsScopeData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerStatisticsViewController_StatsScopeData {
    pub _text_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _playerOverallStatsDataFunc_k__BackingField: *mut crate::System::Func_1<
        *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
    >,
}
#[cfg(feature = "PlayerStatisticsViewController+StatsScopeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerStatisticsViewController_StatsScopeData => ""
    ."PlayerStatisticsViewController/StatsScopeData"
);
#[cfg(feature = "PlayerStatisticsViewController+StatsScopeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PlayerStatisticsViewController_StatsScopeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PlayerStatisticsViewController+StatsScopeData")]
impl crate::GlobalNamespace::PlayerStatisticsViewController_StatsScopeData {
    pub fn _ctor(
        &mut self,
        text: *mut quest_hook::libil2cpp::Il2CppString,
        playerOverallStatsDataFunc: *mut crate::System::Func_1<
            *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (text, playerOverallStatsDataFunc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_playerOverallStatsDataFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_1<
            *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    > {
        let __cordl_ret: *mut crate::System::Func_1<
            *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_playerOverallStatsDataFunc",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_text",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_playerOverallStatsDataFunc(
        &mut self,
        value: *mut crate::System::Func_1<
            *mut crate::GlobalNamespace::PlayerAllOverallStatsData_PlayerOverallStatsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_playerOverallStatsDataFunc",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_text",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
