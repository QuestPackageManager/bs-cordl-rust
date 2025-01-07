#[cfg(feature = "LevelFilteringNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelFilteringNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _emptyFavoritesListInfoPrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _emptyCustomSongListInfoPrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub _selectLevelCategoryViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SelectLevelCategoryViewController,
    >,
    pub _annotatedBeatmapLevelCollectionsViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsViewController,
    >,
    pub _levelSearchViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelSearchViewController,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub didSelectBeatmapLevelPackEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelFilteringNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            crate::GlobalNamespace::LevelSelectionOptions,
        >,
    >,
    pub didStartLoadingEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelFilteringNavigationController,
            >,
        >,
    >,
    pub didPressAllSongsEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelFilteringNavigationController,
            >,
        >,
    >,
    pub didOpenBeatmapLevelCollectionsEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didCloseBeatmapLevelCollectionsEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _cancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _currentNoDataInfoPrefab: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::GameObject,
    >,
    pub _levelPackIdToBeSelectedAfterPresent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _hidePacksIfOneOrNone: bool,
    pub _enableCustomLevels: bool,
    pub _songPackMask: crate::GlobalNamespace::SongPackMask,
    pub _enabledLevelCategories: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        >,
    >,
    pub _ostBeatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _musicPacksBeatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _customLevelPacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _allOfficialBeatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _allBeatmapLevelPacks: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
}
#[cfg(feature = "LevelFilteringNavigationController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelFilteringNavigationController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelFilteringNavigationController";
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
#[cfg(feature = "LevelFilteringNavigationController")]
impl std::ops::Deref for crate::GlobalNamespace::LevelFilteringNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFilteringNavigationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelFilteringNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelFilteringNavigationController")]
impl crate::GlobalNamespace::LevelFilteringNavigationController {
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleAnnotatedBeatmapLevelCollectionsViewControllerDidSelectAnnotatedBeatmapLevelCollection(
        &mut self,
        annotatedBeatmapLevelCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleAnnotatedBeatmapLevelCollectionsViewControllerDidSelectAnnotatedBeatmapLevelCollection",
                (annotatedBeatmapLevelCollection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleIncreaseNumberOfGameplays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleIncreaseNumberOfGameplays", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSearchViewControllerDidFilterBeatmapLevelCollection(
        &mut self,
        annotatedBeatmapLevelCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        options: crate::GlobalNamespace::LevelSelectionOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelSearchViewControllerDidFilterBeatmapLevelCollection",
                (annotatedBeatmapLevelCollection, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelSearchViewControllerDidStartLoading(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelSearchViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelSearchViewControllerDidStartLoading", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerDataFavoriteLevelsSetDidChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDataFavoriteLevelsSetDidChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSelectLevelCategoryViewControllerDidSelectLevelCategory(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SelectLevelCategoryViewController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReplaceSecondViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceSecondViewController", (viewController))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectAnnotatedBeatmapLevelCollection(
        &mut self,
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAnnotatedBeatmapLevelCollection", (levelPack))?;
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        songPackMask: crate::GlobalNamespace::SongPackMask,
        levelPackToBeSelectedAfterPresent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetupBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupBeatmapLevelPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowPacksInSecondChildController(
        &mut self,
        beatmapLevelPacks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowPacksInSecondChildController", (beatmapLevelPacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCustomSongs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCustomSongs", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _SetupBeatmapLevelPacks_b__39_0(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetupBeatmapLevelPacks>b__39_0", (pack))?;
        Ok(__cordl_ret.into())
    }
    pub fn _SetupBeatmapLevelPacks_b__39_1(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<SetupBeatmapLevelPacks>b__39_1", (pack))?;
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
    pub fn add_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressAllSongsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressAllSongsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectBeatmapLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::GlobalNamespace::LevelSelectionOptions,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectBeatmapLevelPackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didStartLoadingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didStartLoadingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapLevelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("get_selectedBeatmapLevelPack", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn remove_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressAllSongsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressAllSongsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectBeatmapLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                crate::GlobalNamespace::LevelSelectionOptions,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectBeatmapLevelPackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didStartLoadingEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelFilteringNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didStartLoadingEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelFilteringNavigationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelFilteringNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
