#[cfg(feature = "StandardLevelDetailView")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelDetailView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _actionButton: *mut crate::UnityEngine::UI::Button,
    pub _actionButtonText: *mut crate::TMPro::TextMeshProUGUI,
    pub _practiceButton: *mut crate::UnityEngine::UI::Button,
    pub _levelBar: *mut LevelBar,
    pub _levelParamsPanel: *mut LevelParamsPanel,
    pub _levelParamsPanelCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _beatmapDifficultySegmentedControlController: *mut BeatmapDifficultySegmentedControlController,
    pub _beatmapCharacteristicSegmentedControlController: *mut BeatmapCharacteristicSegmentedControlController,
    pub _favoriteToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _buttonsWrapper: *mut crate::UnityEngine::GameObject,
    pub _loadingControl: *mut LoadingControl,
    pub _beatmapLevelVersionsWrapper: *mut crate::UnityEngine::GameObject,
    pub _beatmapLevelVersionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _beatmapLevelDataVersionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub _beatmapDataLoader: *mut BeatmapDataLoader,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _beatmapLevelsEntitlementModel: *mut BeatmapLevelsEntitlementModel,
    pub didChangeDifficultyBeatmapEvent: *mut crate::System::Action_1<
        *mut StandardLevelDetailView,
    >,
    pub didFavoriteToggleChangeEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UI::Toggle,
    >,
    pub _beatmapKey_k__BackingField: BeatmapKey,
    pub _beatmapLevel: *mut BeatmapLevel,
    pub _allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
    pub _playerData: *mut PlayerData,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _levelParamsPanelCanvasGroupTween: *mut crate::Tweening::Tween,
    pub _songLength: crate::System::Nullable_1<f32>,
}
#[cfg(feature = "StandardLevelDetailView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardLevelDetailView => ""."StandardLevelDetailView"
);
#[cfg(feature = "StandardLevelDetailView")]
impl std::ops::Deref for StandardLevelDetailView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailView")]
impl std::ops::DerefMut for StandardLevelDetailView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailView")]
impl StandardLevelDetailView {
    #[cfg(feature = "StandardLevelDetailView+__c")]
    pub type __c = crate::GlobalNamespace::StandardLevelDetailView___c;
    #[cfg(feature = "StandardLevelDetailView+_CalculateAndSetContent_d__61")]
    pub type _CalculateAndSetContent_d__61 = crate::GlobalNamespace::StandardLevelDetailView__CalculateAndSetContent_d__61;
    #[cfg(feature = "StandardLevelDetailView+_CheckIfBeatmapLevelDataExists_d__57")]
    pub type _CheckIfBeatmapLevelDataExists_d__57 = crate::GlobalNamespace::StandardLevelDetailView__CheckIfBeatmapLevelDataExists_d__57;
    #[cfg(feature = "StandardLevelDetailView+_SetBeatmapLevelVersions_d__62")]
    pub type _SetBeatmapLevelVersions_d__62 = crate::GlobalNamespace::StandardLevelDetailView__SetBeatmapLevelVersions_d__62;
    #[cfg(feature = "StandardLevelDetailView+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::GlobalNamespace::StandardLevelDetailView___c__DisplayClass45_0;
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_practiceButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_practiceButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearBeatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBeatmapLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn TriggerEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_actionButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_actionButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn _CalculateAndSetContent_b__61_0(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<CalculateAndSetContent>b__61_0", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFavoriteToggleChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didFavoriteToggleChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut StandardLevelDetailView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut StandardLevelDetailView>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ShowContent(
        &mut self,
        contentType: crate::GlobalNamespace::StandardLevelDetailViewController_ContentType,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowContent", (contentType, progress))?;
        Ok(__cordl_ret)
    }
    pub fn ClearContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapDifficultySegmentedControlControllerDidSelectDifficulty(
        &mut self,
        controller: *mut BeatmapDifficultySegmentedControlController,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapDifficultySegmentedControlControllerDidSelectDifficulty",
                (controller, difficulty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckIfBeatmapLevelDataExists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfBeatmapLevelDataExists", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapLevelsModelLevelDownloadingUpdate(
        &mut self,
        levelDownloadingUpdate: crate::GlobalNamespace::BeatmapLevelLoader_LevelDownloadingUpdate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapLevelsModelLevelDownloadingUpdate",
                (levelDownloadingUpdate),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapKey(
        &mut self,
        value: BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContent_BeatmapLevel_BeatmapDifficultyMask_HashSet_1_BeatmapDifficulty_BeatmapCharacteristicSO_PlayerData0(
        &mut self,
        level: *mut BeatmapLevel,
        allowedBeatmapDifficultyMask: BeatmapDifficultyMask,
        notAllowedCharacteristics: *mut crate::System::Collections::Generic::HashSet_1<
            *mut BeatmapCharacteristicSO,
        >,
        defaultDifficulty: BeatmapDifficulty,
        defaultBeatmapCharacteristic: *mut BeatmapCharacteristicSO,
        playerData: *mut PlayerData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetContent",
                (
                    level,
                    allowedBeatmapDifficultyMask,
                    notAllowedCharacteristics,
                    defaultDifficulty,
                    defaultBeatmapCharacteristic,
                    playerData,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetContent_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContentForBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContentForBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hidePracticeButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hidePracticeButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _Awake_b__47_0(
        &mut self,
        _: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__47_0", (_))?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetBeatmapLevelVersions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeatmapLevelVersions", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapCharacteristicSegmentedControlControllerDidSelectBeatmapCharacteristic(
        &mut self,
        controller: *mut BeatmapCharacteristicSegmentedControlController,
        beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCharacteristicSegmentedControlControllerDidSelectBeatmapCharacteristic",
                (controller, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateBeatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("CreateBeatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidPressRefreshButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPressRefreshButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_actionButtonText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_actionButtonText", (value))?;
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
    pub fn CalculateAndSetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateAndSetContent", ())?;
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
#[cfg(feature = "StandardLevelDetailView")]
impl quest_hook::libil2cpp::ObjectType for StandardLevelDetailView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
