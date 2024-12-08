#[cfg(feature = "LevelFilteringNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFilteringNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _emptyFavoritesListInfoPrefab: *mut crate::UnityEngine::GameObject,
    pub _emptyCustomSongListInfoPrefab: *mut crate::UnityEngine::GameObject,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _selectLevelCategoryViewController: *mut SelectLevelCategoryViewController,
    pub _annotatedBeatmapLevelCollectionsViewController: *mut AnnotatedBeatmapLevelCollectionsViewController,
    pub _levelSearchViewController: *mut LevelSearchViewController,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub didSelectBeatmapLevelPackEvent: *mut crate::System::Action_4<
        *mut LevelFilteringNavigationController,
        *mut BeatmapLevelPack,
        *mut crate::UnityEngine::GameObject,
        LevelSelectionOptions,
    >,
    pub didStartLoadingEvent: *mut crate::System::Action_1<
        *mut LevelFilteringNavigationController,
    >,
    pub didPressAllSongsEvent: *mut crate::System::Action_1<
        *mut LevelFilteringNavigationController,
    >,
    pub didOpenBeatmapLevelCollectionsEvent: *mut crate::System::Action,
    pub didCloseBeatmapLevelCollectionsEvent: *mut crate::System::Action,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _currentNoDataInfoPrefab: *mut crate::UnityEngine::GameObject,
    pub _levelPackIdToBeSelectedAfterPresent: *mut crate::System::String,
    pub _hidePacksIfOneOrNone: bool,
    pub _enableCustomLevels: bool,
    pub _songPackMask: SongPackMask,
    pub _enabledLevelCategories: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    >,
    pub _ostBeatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapLevelPack,
    >,
    pub _musicPacksBeatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapLevelPack,
    >,
    pub _customLevelPacks: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut BeatmapLevelPack,
    >,
    pub _allOfficialBeatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapLevelPack,
    >,
    pub _allBeatmapLevelPacks: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapLevelPack,
    >,
}
#[cfg(feature = "LevelFilteringNavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelFilteringNavigationController => ""
    ."LevelFilteringNavigationController"
);
#[cfg(feature = "LevelFilteringNavigationController")]
impl std::ops::Deref for LevelFilteringNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFilteringNavigationController")]
impl std::ops::DerefMut for LevelFilteringNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFilteringNavigationController")]
impl LevelFilteringNavigationController {
    #[cfg(feature = "LevelFilteringNavigationController+_UpdateCustomSongs_d__54")]
    pub type _UpdateCustomSongs_d__54 = crate::GlobalNamespace::LevelFilteringNavigationController__UpdateCustomSongs_d__54;
    #[cfg(feature = "LevelFilteringNavigationController+__c")]
    pub type __c = crate::GlobalNamespace::LevelFilteringNavigationController___c;
    #[cfg(feature = "LevelFilteringNavigationController+__c__DisplayClass53_0")]
    pub type __c__DisplayClass53_0 = crate::GlobalNamespace::LevelFilteringNavigationController___c__DisplayClass53_0;
    pub fn SetupBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupBeatmapLevelPacks", ())?;
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
    pub fn HandleIncreaseNumberOfGameplays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleIncreaseNumberOfGameplays", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedBeatmapLevelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("get_selectedBeatmapLevelPack", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelSearchViewControllerDidFilterBeatmapLevelCollection(
        &mut self,
        annotatedBeatmapLevelCollection: *mut BeatmapLevelPack,
        options: LevelSelectionOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSearchViewControllerDidFilterBeatmapLevelCollection",
                (annotatedBeatmapLevelCollection, options),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleAnnotatedBeatmapLevelCollectionsViewControllerDidCloseBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAnnotatedBeatmapLevelCollectionsViewControllerDidCloseBeatmapLevelCollections",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        songPackMask: SongPackMask,
        levelPackToBeSelectedAfterPresent: *mut BeatmapLevelPack,
        startLevelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        hidePacksIfOneOrNone: bool,
        enableCustomLevels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Setup",
                (
                    songPackMask,
                    levelPackToBeSelectedAfterPresent,
                    startLevelCategory,
                    hidePacksIfOneOrNone,
                    enableCustomLevels,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SelectAnnotatedBeatmapLevelCollection(
        &mut self,
        levelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAnnotatedBeatmapLevelCollection", (levelPack))?;
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
    pub fn _SetupBeatmapLevelPacks_b__39_1(
        &mut self,
        pack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetupBeatmapLevelPacks>b__39_1", (pack))?;
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
    pub fn add_didSelectBeatmapLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut LevelFilteringNavigationController,
            *mut BeatmapLevelPack,
            *mut crate::UnityEngine::GameObject,
            LevelSelectionOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectBeatmapLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressAllSongsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelFilteringNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressAllSongsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceSecondViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceSecondViewController", (viewController))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDataFavoriteLevelsSetDidChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDataFavoriteLevelsSetDidChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressAllSongsEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelFilteringNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressAllSongsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSecondChildControllerContent(
        &mut self,
        levelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSecondChildControllerContent", (levelCategory))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAnnotatedBeatmapLevelCollectionsViewControllerDidSelectAnnotatedBeatmapLevelCollection(
        &mut self,
        annotatedBeatmapLevelCollection: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAnnotatedBeatmapLevelCollectionsViewControllerDidSelectAnnotatedBeatmapLevelCollection",
                (annotatedBeatmapLevelCollection),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCustomSongs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCustomSongs", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowPacksInSecondChildController(
        &mut self,
        beatmapLevelPacks: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowPacksInSecondChildController", (beatmapLevelPacks))?;
        Ok(__cordl_ret)
    }
    pub fn add_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedLevelCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory = __cordl_object
            .invoke("get_selectedLevelCategory", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelSearchViewControllerDidStartLoading(
        &mut self,
        obj: *mut LevelSearchViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelSearchViewControllerDidStartLoading", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectBeatmapLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_4<
            *mut LevelFilteringNavigationController,
            *mut BeatmapLevelPack,
            *mut crate::UnityEngine::GameObject,
            LevelSelectionOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectBeatmapLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didStartLoadingEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelFilteringNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didStartLoadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelectLevelCategoryViewControllerDidSelectLevelCategory(
        &mut self,
        viewController: *mut SelectLevelCategoryViewController,
        levelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSelectLevelCategoryViewControllerDidSelectLevelCategory",
                (viewController, levelCategory),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _SetupBeatmapLevelPacks_b__39_0(
        &mut self,
        pack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetupBeatmapLevelPacks>b__39_0", (pack))?;
        Ok(__cordl_ret)
    }
    pub fn HandleAnnotatedBeatmapLevelCollectionsViewControllerDidOpenBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAnnotatedBeatmapLevelCollectionsViewControllerDidOpenBeatmapLevelCollections",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didStartLoadingEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelFilteringNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didStartLoadingEvent", (value))?;
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
#[cfg(feature = "LevelFilteringNavigationController")]
impl quest_hook::libil2cpp::ObjectType for LevelFilteringNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
