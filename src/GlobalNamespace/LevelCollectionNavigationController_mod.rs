#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelCollectionNavigationController_AlphaAnimationType {
    In = 0i32,
    Out = 1i32,
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType => ""
    ."LevelCollectionNavigationController/AlphaAnimationType"
);
#[cfg(feature = "LevelCollectionNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCollectionNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _loadingControl: *mut LoadingControl,
    pub _levelCollectionViewController: *mut LevelCollectionViewController,
    pub _levelPackDetailViewController: *mut LevelPackDetailViewController,
    pub _levelDetailViewController: *mut StandardLevelDetailViewController,
    pub _timeTweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub didChangeLevelDetailContentEvent: *mut crate::System::Action_2<
        *mut LevelCollectionNavigationController,
        crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    >,
    pub didSelectLevelPackEvent: *mut crate::System::Action_2<
        *mut LevelCollectionNavigationController,
        *mut BeatmapLevelPack,
    >,
    pub didPressActionButtonEvent: *mut crate::System::Action_1<
        *mut LevelCollectionNavigationController,
    >,
    pub didPressOpenPackButtonEvent: *mut crate::System::Action_2<
        *mut LevelCollectionNavigationController,
        *mut BeatmapLevelPack,
    >,
    pub didPressPracticeButtonEvent: *mut crate::System::Action_2<
        *mut LevelCollectionNavigationController,
        *mut BeatmapLevel,
    >,
    pub didChangeDifficultyBeatmapEvent: *mut crate::System::Action_1<
        *mut LevelCollectionNavigationController,
    >,
    pub _showPracticeButtonInDetailView: bool,
    pub _actionButtonTextInDetailView: *mut crate::System::String,
    pub _levelPack: *mut BeatmapLevelPack,
    pub _allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
    pub _beatmapLevelToBeSelectedAfterPresent: *mut BeatmapLevel,
    pub _loading: bool,
    pub _hideDetailViewController: bool,
    pub _notAllowedCharacteristics: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapCharacteristicSO,
    >,
    pub _floatTween: *mut crate::Tweening::FloatTween,
}
#[cfg(feature = "LevelCollectionNavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelCollectionNavigationController => ""
    ."LevelCollectionNavigationController"
);
#[cfg(feature = "LevelCollectionNavigationController")]
impl std::ops::Deref for LevelCollectionNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionNavigationController")]
impl std::ops::DerefMut for LevelCollectionNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionNavigationController")]
impl LevelCollectionNavigationController {
    #[cfg(feature = "LevelCollectionNavigationController+__c__DisplayClass53_0")]
    pub type __c__DisplayClass53_0 = crate::GlobalNamespace::LevelCollectionNavigationController___c__DisplayClass53_0;
    #[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
    pub type AlphaAnimationType = crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType;
    pub fn SelectLevel(
        &mut self,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLevel", (beatmapLevel))?;
        Ok(__cordl_ret)
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
    pub fn HandleLevelCollectionViewControllerDidSelectLevel(
        &mut self,
        viewController: *mut LevelCollectionViewController,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionViewControllerDidSelectLevel",
                (viewController, level),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeLevelDetailContentEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn HandleLevelDetailViewControllerDidPressActionButton(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressActionButton",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AnimateCanvasGroupAlpha(
        &mut self,
        animationType: crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateCanvasGroupAlpha", (animationType))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelDetailViewControllerLevelFavoriteStatusDidChange(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
        favoriteStatus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerLevelFavoriteStatusDidChange",
                (viewController, favoriteStatus),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelDetailViewControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidChangeDifficultyBeatmap",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PresentViewControllersForPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentViewControllersForPack", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressOpenPackButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressOpenPackButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        beatmapLevelPack: *mut BeatmapLevelPack,
        showPackHeader: bool,
        showPracticeButton: bool,
        actionButtonText: *mut crate::System::String,
        sortAlphabetically: bool,
        noDataInfoPrefab: *mut crate::UnityEngine::GameObject,
        allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
        notAllowedCharacteristics: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut BeatmapCharacteristicSO,
        >,
        isFiltered: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    beatmapLevelPack,
                    showPackHeader,
                    showPracticeButton,
                    actionButtonText,
                    sortAlphabetically,
                    noDataInfoPrefab,
                    allowedBeatmapDifficultyMask,
                    notAllowedCharacteristics,
                    isFiltered,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn SetDataForLevelCollection(
        &mut self,
        beatmapLevelPack: *mut BeatmapLevelPack,
        showPracticeButton: bool,
        actionButtonText: *mut crate::System::String,
        noDataInfoPrefab: *mut crate::UnityEngine::GameObject,
        sortBeatmapLevels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetDataForLevelCollection",
                (
                    beatmapLevelPack,
                    showPracticeButton,
                    actionButtonText,
                    noDataInfoPrefab,
                    sortBeatmapLevels,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PresentDetailViewController(
        &mut self,
        viewController: *mut crate::HMUI::ViewController,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentDetailViewController", (viewController, immediately))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn add_didChangeLevelDetailContentEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn HandleLevelDetailViewControllerDidPressPracticeButton(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressPracticeButton",
                (viewController, beatmapLevel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelCollectionViewControllerDidSelectPack(
        &mut self,
        viewController: *mut LevelCollectionViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionViewControllerDidSelectPack",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _AnimateCanvasGroupAlpha_b__39_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateCanvasGroupAlpha>b__39_1", ())?;
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
    pub fn HandleLevelDetailViewControllerDidPressOpenLevelPackButton(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
        levelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressOpenLevelPackButton",
                (viewController, levelPack),
            )?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressOpenPackButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressOpenPackButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetDataForPack(
        &mut self,
        levelPack: *mut BeatmapLevelPack,
        showPackHeader: bool,
        showPracticeButton: bool,
        actionButtonText: *mut crate::System::String,
        sortBeatmapLevels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetDataForPack",
                (
                    levelPack,
                    showPackHeader,
                    showPracticeButton,
                    actionButtonText,
                    sortBeatmapLevels,
                ),
            )?;
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
    pub fn _AnimateCanvasGroupAlpha_b__39_0(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateCanvasGroupAlpha>b__39_0", (f))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectLevelPackEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn HandleLevelDetailViewControllerDidPresentContent(
        &mut self,
        viewController: *mut StandardLevelDetailViewController,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPresentContent",
                (viewController, contentType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PresentViewControllersForLevelCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentViewControllersForLevelCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideDetailViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideDetailViewController", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut LevelCollectionNavigationController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
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
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut LevelCollectionNavigationController,
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
    pub fn HideLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLoading", ())?;
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
#[cfg(feature = "LevelCollectionNavigationController")]
impl quest_hook::libil2cpp::ObjectType for LevelCollectionNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
