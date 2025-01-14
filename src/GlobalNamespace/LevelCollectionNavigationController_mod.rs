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
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    >,
    pub didSelectLevelPackEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub didPressActionButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
        >,
    >,
    pub didPressOpenPackButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub didPressPracticeButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    >,
    pub didChangeDifficultyBeatmapEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelCollectionNavigationController,
            >,
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub _floatTween: quest_hook::libil2cpp::Gc<crate::Tweening::FloatTween>,
}
#[cfg(feature = "LevelCollectionNavigationController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelCollectionNavigationController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelCollectionNavigationController";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AnimateCanvasGroupAlpha")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AnimateCanvasGroupAlpha", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (animationType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearSelected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearSelected", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DidActivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidActivate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DidDeactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DidDeactivate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionViewControllerDidSelectLevel(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionViewController,
        >,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::LevelCollectionViewController,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelCollectionViewControllerDidSelectLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelCollectionViewControllerDidSelectLevel", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, level))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionViewControllerDidSelectPack(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionViewController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLevelCollectionViewControllerDidSelectPack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelCollectionViewControllerDidSelectPack", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelDetailViewController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLevelDetailViewControllerDidChangeDifficultyBeatmap")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerDidChangeDifficultyBeatmap",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPresentContent(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardLevelDetailViewController,
                    >,
                    crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelDetailViewControllerDidPresentContent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerDidPresentContent", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, contentType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressActionButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardLevelDetailViewController,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("HandleLevelDetailViewControllerDidPressActionButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerDidPressActionButton", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressOpenLevelPackButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardLevelDetailViewController,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelDetailViewControllerDidPressOpenLevelPackButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerDidPressOpenLevelPackButton",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, levelPack))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerDidPressPracticeButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardLevelDetailViewController,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelDetailViewControllerDidPressPracticeButton")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerDidPressPracticeButton", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, beatmapLevel))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelDetailViewControllerLevelFavoriteStatusDidChange(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelDetailViewController,
        >,
        favoriteStatus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::StandardLevelDetailViewController,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("HandleLevelDetailViewControllerLevelFavoriteStatusDidChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HandleLevelDetailViewControllerLevelFavoriteStatusDidChange",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, favoriteStatus))
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideDetailViewController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HideDetailViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HideDetailViewController", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HideLoading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HideLoading", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::HMUI::ViewController>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PresentDetailViewController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PresentDetailViewController", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (viewController, immediately))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewControllersForLevelCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("PresentViewControllersForLevelCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PresentViewControllersForLevelCollection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn PresentViewControllersForPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("PresentViewControllersForPack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PresentViewControllersForPack", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshDetail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RefreshDetail")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshDetail", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SelectLevel(
        &mut self,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SelectLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SelectLevel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapLevel))
        };
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
        isFiltered: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    crate::GlobalNamespace::BeatmapDifficultyMask,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("SetData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetData", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetDataForLevelCollection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDataForLevelCollection", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatmapLevelPack,
                        showPracticeButton,
                        actionButtonText,
                        noDataInfoPrefab,
                        sortBeatmapLevels,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
                    bool,
                    bool,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("SetDataForPack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDataForPack", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        levelPack,
                        showPackHeader,
                        showPracticeButton,
                        actionButtonText,
                        sortBeatmapLevels,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ShowLoading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShowLoading", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateCanvasGroupAlpha_b__39_0(
        &mut self,
        f: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (f32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("<AnimateCanvasGroupAlpha>b__39_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<AnimateCanvasGroupAlpha>b__39_0", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (f))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _AnimateCanvasGroupAlpha_b__39_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("<AnimateCanvasGroupAlpha>b__39_1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<AnimateCanvasGroupAlpha>b__39_1", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didChangeDifficultyBeatmapEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didChangeDifficultyBeatmapEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didChangeLevelDetailContentEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didChangeLevelDetailContentEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didPressActionButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didPressActionButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressOpenPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didPressOpenPackButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didPressOpenPackButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didPressPracticeButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didPressPracticeButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didSelectLevelPackEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didSelectLevelPackEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::BeatmapKey,
                0usize,
            >("get_beatmapKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beatmapKey", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                0usize,
            >("get_beatmapLevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_beatmapLevel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didChangeDifficultyBeatmapEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didChangeDifficultyBeatmapEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didChangeLevelDetailContentEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didChangeLevelDetailContentEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didPressActionButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didPressActionButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressOpenPackButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didPressOpenPackButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didPressOpenPackButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didPressPracticeButtonEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didPressPracticeButtonEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelCollectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didSelectLevelPackEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didSelectLevelPackEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LevelCollectionNavigationController_AlphaAnimationType {
    #[default]
    In = 0i32,
    Out = 1i32,
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelCollectionNavigationController/AlphaAnimationType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "LevelCollectionNavigationController+AlphaAnimationType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LevelCollectionNavigationController_AlphaAnimationType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
