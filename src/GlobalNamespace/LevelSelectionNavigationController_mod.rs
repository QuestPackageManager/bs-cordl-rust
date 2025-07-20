#[cfg(feature = "LevelSelectionNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelSelectionNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _levelFilteringNavigationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelFilteringNavigationController,
    >,
    pub _levelCollectionNavigationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelCollectionNavigationController,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub didChangeLevelDetailContentEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelSelectionNavigationController,
            >,
            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        >,
    >,
    pub didSelectLevelPackEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelSelectionNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub didPressActionButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelSelectionNavigationController,
            >,
        >,
    >,
    pub didPressPracticeButtonEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelSelectionNavigationController,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        >,
    >,
    pub didChangeDifficultyBeatmapEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::LevelSelectionNavigationController,
            >,
        >,
    >,
    pub _hidePacksIfOneOrNone: bool,
    pub _hidePracticeButton: bool,
    pub _actionButtonText: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub _notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
}
#[cfg(feature = "LevelSelectionNavigationController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LevelSelectionNavigationController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LevelSelectionNavigationController";
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
#[cfg(feature = "LevelSelectionNavigationController")]
impl std::ops::Deref for crate::GlobalNamespace::LevelSelectionNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelSelectionNavigationController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl crate::GlobalNamespace::LevelSelectionNavigationController {
    pub fn ClearSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ClearSelected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearSelected", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool, bool),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("DidActivate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DidActivate", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (firstActivation, addedToHierarchy, screenSystemEnabling),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool, bool),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("DidDeactivate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DidDeactivate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (removedFromHierarchy, screenSystemDisabling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidChangeDifficultyBeatmap(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "HandleLevelCollectionNavigationControllerDidChangeDifficultyBeatmap",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidChangeDifficultyBeatmap",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (viewController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidChangeLevelDetailContent(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCollectionNavigationController,
                            >,
                            crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(
                        "HandleLevelCollectionNavigationControllerDidChangeLevelDetailContent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidChangeLevelDetailContent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (viewController, contentType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressActionButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelCollectionNavigationController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleLevelCollectionNavigationControllerDidPressActionButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidPressActionButton",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (viewController))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressOpenPackButton(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCollectionNavigationController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleLevelCollectionNavigationControllerDidPressOpenPackButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidPressOpenPackButton",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (viewController, beatmapLevelPack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidPressPracticeButton(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCollectionNavigationController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleLevelCollectionNavigationControllerDidPressPracticeButton")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidPressPracticeButton",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl__, beatmapLevel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelCollectionNavigationControllerDidSelectPack(
        &mut self,
        viewController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCollectionNavigationController,
        >,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelCollectionNavigationController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleLevelCollectionNavigationControllerDidSelectPack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelCollectionNavigationControllerDidSelectPack",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (viewController, beatmapLevelPack))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFilteringNavigationControllerDidCloseBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "HandleLevelFilteringNavigationControllerDidCloseBeatmapLevelCollections",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelFilteringNavigationControllerDidCloseBeatmapLevelCollections",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFilteringNavigationControllerDidOpenBeatmapLevelCollections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "HandleLevelFilteringNavigationControllerDidOpenBeatmapLevelCollections",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelFilteringNavigationControllerDidOpenBeatmapLevelCollections",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFilteringNavigationControllerDidPressAllSongs(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelFilteringNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelFilteringNavigationController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleLevelFilteringNavigationControllerDidPressAllSongs")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelFilteringNavigationControllerDidPressAllSongs",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (controller))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFilteringNavigationControllerDidSelectBeatmapLevelPack(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelFilteringNavigationController,
        >,
        annotatedBeatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        noDataInfoPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        options: crate::GlobalNamespace::LevelSelectionOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LevelFilteringNavigationController,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                            crate::GlobalNamespace::LevelSelectionOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(
                        "HandleLevelFilteringNavigationControllerDidSelectBeatmapLevelPack",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelFilteringNavigationControllerDidSelectBeatmapLevelPack",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (controller, annotatedBeatmapLevelPack, noDataInfoPrefab, options),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleLevelFilteringNavigationControllerDidStartLoading(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelFilteringNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::LevelFilteringNavigationController,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleLevelFilteringNavigationControllerDidStartLoading")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleLevelFilteringNavigationControllerDidStartLoading",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (controller))?
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
    pub fn RefreshDetail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("RefreshDetail")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RefreshDetail", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        songPackMask: crate::GlobalNamespace::SongPackMask,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
        hidePacksIfOneOrNone: bool,
        hidePracticeButton: bool,
        actionButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelPackToBeSelectedAfterPresent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        startLevelCategory: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
        beatmapLevelToBeSelectedAfterPresent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        >,
        enableCustomLevels: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::SongPackMask,
                            crate::GlobalNamespace::BeatmapDifficultyMask,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                                    >,
                                >,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                            crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevel,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        10usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_didChangeDifficultyBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_didChangeDifficultyBeatmapEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_didChangeLevelDetailContentEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_didPressActionButtonEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_didPressActionButtonEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_didPressPracticeButtonEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_didPressPracticeButtonEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_didSelectLevelPackEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::BeatmapKey,
                        0usize,
                    >("get_beatmapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_beatmapKey", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                        0usize,
                    >("get_beatmapLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_beatmapLevel", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedBeatmapLevelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                        0usize,
                    >("get_selectedBeatmapLevelPack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_selectedBeatmapLevelPack", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedLevelCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory,
                        0usize,
                    >("get_selectedLevelCategory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_selectedLevelCategory", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::SelectLevelCategoryViewController_LevelCategory = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_didChangeDifficultyBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_didChangeDifficultyBeatmapEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeLevelDetailContentEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_didChangeLevelDetailContentEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressActionButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_didPressActionButtonEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_didPressActionButtonEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressPracticeButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevel,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_didPressPracticeButtonEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_didPressPracticeButtonEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectLevelPackEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LevelSelectionNavigationController,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::LevelSelectionNavigationController,
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_didSelectLevelPackEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelSelectionNavigationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelSelectionNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
