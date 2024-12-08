#[cfg(feature = "LevelSearchViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelSearchViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _searchButton: *mut crate::UnityEngine::UI::Button,
    pub _clearFiltersButton: *mut crate::UnityEngine::UI::Button,
    pub _filterParamsText: *mut crate::TMPro::TextMeshProUGUI,
    pub _filterPlaceholder: *mut crate::UnityEngine::GameObject,
    pub _searchTextInputFieldView: *mut crate::HMUI::InputFieldView,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _songPackMasksModel: *mut crate::GlobalNamespace::SongPackMasksModel,
    pub _entitlementModel: *mut crate::GlobalNamespace::IEntitlementModel,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _uiKeyboardManager: *mut crate::GlobalNamespace::UIKeyboardManager,
    pub didPressSearchButtonEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::LevelSearchViewController,
        crate::GlobalNamespace::LevelFilter,
    >,
    pub didFilterBeatmapLevelCollectionEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::BeatmapLevelPack,
        crate::GlobalNamespace::LevelSelectionOptions,
    >,
    pub didStartLoadingEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::LevelSearchViewController,
    >,
    pub _beatmapLevelPack: *mut crate::GlobalNamespace::BeatmapLevelPack,
    pub _preferredBeatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _preferredBeatmapDifficulty: crate::System::Nullable_1<
        crate::GlobalNamespace::BeatmapDifficulty,
    >,
    pub _currentSearchFilter: crate::GlobalNamespace::LevelFilter,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _beatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevelPack,
    >,
}
#[cfg(feature = "LevelSearchViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelSearchViewController => ""
    ."LevelSearchViewController"
);
#[cfg(feature = "LevelSearchViewController")]
impl std::ops::Deref for crate::GlobalNamespace::LevelSearchViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSearchViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelSearchViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSearchViewController")]
impl crate::GlobalNamespace::LevelSearchViewController {
    #[cfg(feature = "LevelSearchViewController+_RefreshAsync_d__35")]
    pub type _RefreshAsync_d__35 = crate::GlobalNamespace::LevelSearchViewController__RefreshAsync_d__35;
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
    pub fn IsFilteringPlayCounts(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFilteringPlayCounts", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn Refresh_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn Refresh_ByRefMut1(
        &mut self,
        filter: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::LevelFilter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", (filter))?;
        Ok(__cordl_ret)
    }
    pub fn ResetAllFilterSettings(
        &mut self,
        onlyFavorites: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetAllFilterSettings", (onlyFavorites))?;
        Ok(__cordl_ret)
    }
    pub fn ResetFilter(
        &mut self,
        onlyFavorites: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetFilter", (onlyFavorites))?;
        Ok(__cordl_ret)
    }
    pub fn ResetOptionFilterSettings(
        &mut self,
        onlyFavorites: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetOptionFilterSettings", (onlyFavorites))?;
        Ok(__cordl_ret)
    }
    pub fn ResetTextFilterSettings(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetTextFilterSettings", (text))?;
        Ok(__cordl_ret)
    }
    pub fn SearchTextInputFieldViewOnValueChanged(
        &mut self,
        inputFieldView: *mut crate::HMUI::InputFieldView,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SearchTextInputFieldViewOnValueChanged", (inputFieldView))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        beatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (beatmapLevelPacks))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__30_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__30_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__30_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__30_1", ())?;
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
    pub fn add_didFilterBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapLevelPack,
            crate::GlobalNamespace::LevelSelectionOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFilterBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressSearchButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelSearchViewController,
            crate::GlobalNamespace::LevelFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressSearchButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didStartLoadingEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::LevelSearchViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didStartLoadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFilterBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapLevelPack,
            crate::GlobalNamespace::LevelSelectionOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFilterBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressSearchButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelSearchViewController,
            crate::GlobalNamespace::LevelFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressSearchButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didStartLoadingEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::LevelSearchViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didStartLoadingEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelSearchViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelSearchViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
