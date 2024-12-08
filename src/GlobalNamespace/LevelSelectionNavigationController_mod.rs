#[cfg(feature = "LevelSelectionNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelSelectionNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _levelFilteringNavigationController: *mut LevelFilteringNavigationController,
    pub _levelCollectionNavigationController: *mut LevelCollectionNavigationController,
    pub _playerDataModel: *mut PlayerDataModel,
    pub didChangeLevelDetailContentEvent: *mut crate::System::Action_2<
        *mut LevelSelectionNavigationController,
        crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    >,
    pub didSelectLevelPackEvent: *mut crate::System::Action_2<
        *mut LevelSelectionNavigationController,
        *mut BeatmapLevelPack,
    >,
    pub didPressActionButtonEvent: *mut crate::System::Action_1<
        *mut LevelSelectionNavigationController,
    >,
    pub didPressPracticeButtonEvent: *mut crate::System::Action_2<
        *mut LevelSelectionNavigationController,
        *mut BeatmapLevel,
    >,
    pub didChangeDifficultyBeatmapEvent: *mut crate::System::Action_1<
        *mut LevelSelectionNavigationController,
    >,
    pub _hidePacksIfOneOrNone: bool,
    pub _hidePracticeButton: bool,
    pub _actionButtonText: *mut crate::System::String,
    pub _allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
    pub _notAllowedCharacteristics: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapCharacteristicSO,
    >,
}
#[cfg(feature = "LevelSelectionNavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelSelectionNavigationController => ""
    ."LevelSelectionNavigationController"
);
#[cfg(feature = "LevelSelectionNavigationController")]
impl std::ops::Deref for LevelSelectionNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl std::ops::DerefMut for LevelSelectionNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl LevelSelectionNavigationController {
    pub fn ClearSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelected", ())?;
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
    pub fn HandleLevelCollectionNavigationControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: *mut LevelCollectionNavigationController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidChangeDifficultyBeatmap",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionNavigationControllerDidChangeLevelDetailContent(
        &mut self,
        viewController: *mut LevelCollectionNavigationController,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidChangeLevelDetailContent",
                (viewController, contentType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressActionButton(
        &mut self,
        viewController: *mut LevelCollectionNavigationController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidPressActionButton",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressOpenPackButton(
        &mut self,
        viewController: *mut LevelCollectionNavigationController,
        beatmapLevelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidPressOpenPackButton",
                (viewController, beatmapLevelPack),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressPracticeButton(
        &mut self,
        _: *mut LevelCollectionNavigationController,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidPressPracticeButton",
                (_, beatmapLevel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionNavigationControllerDidSelectPack(
        &mut self,
        viewController: *mut LevelCollectionNavigationController,
        beatmapLevelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionNavigationControllerDidSelectPack",
                (viewController, beatmapLevelPack),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFilteringNavigationControllerDidCloseBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelFilteringNavigationControllerDidCloseBeatmapLevelCollections",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFilteringNavigationControllerDidOpenBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelFilteringNavigationControllerDidOpenBeatmapLevelCollections",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFilteringNavigationControllerDidPressAllSongs(
        &mut self,
        controller: *mut LevelFilteringNavigationController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelFilteringNavigationControllerDidPressAllSongs",
                (controller),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFilteringNavigationControllerDidSelectBeatmapLevelPack(
        &mut self,
        controller: *mut LevelFilteringNavigationController,
        annotatedBeatmapLevelPack: *mut BeatmapLevelPack,
        noDataInfoPrefab: *mut crate::UnityEngine::GameObject,
        options: LevelSelectionOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelFilteringNavigationControllerDidSelectBeatmapLevelPack",
                (controller, annotatedBeatmapLevelPack, noDataInfoPrefab, options),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelFilteringNavigationControllerDidStartLoading(
        &mut self,
        controller: *mut LevelFilteringNavigationController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelFilteringNavigationControllerDidStartLoading",
                (controller),
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
    pub fn RefreshDetail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshDetail", ())?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        songPackMask: SongPackMask,
        allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
        notAllowedCharacteristics: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut BeatmapCharacteristicSO,
        >,
        hidePacksIfOneOrNone: bool,
        hidePracticeButton: bool,
        actionButtonText: *mut crate::System::String,
        levelPackToBeSelectedAfterPresent: *mut BeatmapLevelPack,
        startLevelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        beatmapLevelToBeSelectedAfterPresent: *mut BeatmapLevel,
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
                    allowedBeatmapDifficultyMask,
                    notAllowedCharacteristics,
                    hidePacksIfOneOrNone,
                    hidePracticeButton,
                    actionButtonText,
                    levelPackToBeSelectedAfterPresent,
                    startLevelCategory,
                    beatmapLevelToBeSelectedAfterPresent,
                    enableCustomLevels,
                ),
            )?;
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
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelSelectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didChangeLevelDetailContentEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeLevelDetailContentEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelSelectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            *mut BeatmapLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevel = __cordl_object
            .invoke("get_beatmapLevel", ())?;
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
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelSelectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeLevelDetailContentEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeLevelDetailContentEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelSelectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            *mut BeatmapLevel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelSelectionNavigationController,
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectLevelPackEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl quest_hook::libil2cpp::ObjectType for LevelSelectionNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
