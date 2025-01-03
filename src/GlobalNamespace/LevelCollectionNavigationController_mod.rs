#[cfg(feature = "LevelCollectionNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCollectionNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _loadingControl: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LoadingControl,
    >,
    pub _levelCollectionViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelCollectionViewController,
    >,
    pub _levelPackDetailViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelPackDetailViewController,
    >,
    pub _levelDetailViewController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelDetailViewController,
    >,
    pub _timeTweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::TimeTweeningManager,
    >,
    pub didChangeLevelDetailContentEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    >,
    pub didSelectLevelPackEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    >,
    pub didPressActionButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
    >,
    pub didPressOpenPackButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    >,
    pub didPressPracticeButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            *mut crate::GlobalNamespace::BeatmapLevel,
        >,
    >,
    pub didChangeDifficultyBeatmapEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
    >,
    pub _showPracticeButtonInDetailView: bool,
    pub _actionButtonTextInDetailView: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    pub _allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub _beatmapLevelToBeSelectedAfterPresent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevel,
    >,
    pub _loading: bool,
    pub _hideDetailViewController: bool,
    pub _notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    >,
    pub _floatTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
}
#[cfg(feature = "LevelCollectionNavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelCollectionNavigationController => ""
    ."LevelCollectionNavigationController"
);
#[cfg(feature = "LevelCollectionNavigationController")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCollectionNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionNavigationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelCollectionNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCollectionNavigationController")]
impl crate::GlobalNamespace::LevelCollectionNavigationController {
    #[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
    pub type AlphaAnimationType = crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType;
    pub fn AnimateCanvasGroupAlpha(
        &mut self,
        animationType: crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AnimateCanvasGroupAlpha", (animationType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelected", ())?;
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
    pub fn HandleLevelCollectionViewControllerDidSelectLevel(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionViewController,
        >,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionViewControllerDidSelectLevel",
                (viewController, level),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionViewControllerDidSelectPack(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelCollectionViewControllerDidSelectPack",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidChangeDifficultyBeatmap",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPresentContent(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressActionButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressActionButton",
                (viewController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressOpenLevelPackButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressOpenLevelPackButton",
                (viewController, levelPack),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressPracticeButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDetailViewControllerDidPressPracticeButton",
                (viewController, beatmapLevel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerLevelFavoriteStatusDidChange(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HideDetailViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideDetailViewController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideLoading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PresentDetailViewController(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>,
        immediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentDetailViewController", (viewController, immediately))?;
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewControllersForLevelCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentViewControllersForLevelCollection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewControllersForPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentViewControllersForPack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshDetail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshDetail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectLevel(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLevel", (beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        showPackHeader: bool,
        showPracticeButton: bool,
        actionButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortAlphabetically: bool,
        noDataInfoPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetDataForLevelCollection(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        showPracticeButton: bool,
        actionButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noDataInfoPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetDataForPack(
        &mut self,
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        showPackHeader: bool,
        showPracticeButton: bool,
        actionButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ShowLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLoading", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateCanvasGroupAlpha_b__39_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AnimateCanvasGroupAlpha>b__39_1", ())?;
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
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeLevelDetailContentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressOpenPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressOpenPackButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectLevelPackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeLevelDetailContentEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressActionButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressOpenPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressOpenPackButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevel,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPracticeButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::LevelCollectionNavigationController,
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectLevelPackEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelCollectionNavigationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCollectionNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
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
