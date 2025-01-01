#[cfg(feature = "StandardLevelDetailView")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelDetailView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _actionButton: *mut crate::UnityEngine::UI::Button,
    pub _actionButtonText: *mut crate::TMPro::TextMeshProUGUI,
    pub _practiceButton: *mut crate::UnityEngine::UI::Button,
    pub _levelBar: *mut crate::GlobalNamespace::LevelBar,
    pub _levelParamsPanel: *mut crate::GlobalNamespace::LevelParamsPanel,
    pub _levelParamsPanelCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _beatmapDifficultySegmentedControlController: *mut crate::GlobalNamespace::BeatmapDifficultySegmentedControlController,
    pub _beatmapCharacteristicSegmentedControlController: *mut crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController,
    pub _favoriteToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _buttonsWrapper: *mut crate::UnityEngine::GameObject,
    pub _loadingControl: *mut crate::GlobalNamespace::LoadingControl,
    pub _beatmapLevelVersionsWrapper: *mut crate::UnityEngine::GameObject,
    pub _beatmapLevelVersionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _beatmapLevelDataVersionText: *mut crate::TMPro::TextMeshProUGUI,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _beatmapDataLoader: *mut crate::GlobalNamespace::BeatmapDataLoader,
    pub _audioClipAsyncLoader: *mut crate::GlobalNamespace::AudioClipAsyncLoader,
    pub _tweeningManager: *mut crate::Tweening::TimeTweeningManager,
    pub _settingsManager: *mut crate::GlobalNamespace::SettingsManager,
    pub _beatmapLevelsEntitlementModel: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    pub didChangeDifficultyBeatmapEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::StandardLevelDetailView,
    >,
    pub didFavoriteToggleChangeEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::UI::Toggle,
    >,
    pub _beatmapKey_k__BackingField: crate::GlobalNamespace::BeatmapKey,
    pub _beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub _allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub _playerData: *mut crate::GlobalNamespace::PlayerData,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _levelParamsPanelCanvasGroupTween: *mut crate::Tweening::Tween,
}
#[cfg(feature = "StandardLevelDetailView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardLevelDetailView => ""
    ."StandardLevelDetailView"
);
#[cfg(feature = "StandardLevelDetailView")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelDetailView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailView")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelDetailView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelDetailView")]
impl crate::GlobalNamespace::StandardLevelDetailView {
    #[cfg(feature = "StandardLevelDetailView+_CalculateAndSetContentAsync_d__61")]
    pub type _CalculateAndSetContentAsync_d__61 = crate::GlobalNamespace::StandardLevelDetailView__CalculateAndSetContentAsync_d__61;
    #[cfg(feature = "StandardLevelDetailView+_CheckIfBeatmapLevelDataExists_d__56")]
    pub type _CheckIfBeatmapLevelDataExists_d__56 = crate::GlobalNamespace::StandardLevelDetailView__CheckIfBeatmapLevelDataExists_d__56;
    #[cfg(feature = "StandardLevelDetailView+_SetBeatmapLevelVersions_d__62")]
    pub type _SetBeatmapLevelVersions_d__62 = crate::GlobalNamespace::StandardLevelDetailView__SetBeatmapLevelVersions_d__62;
    #[cfg(feature = "StandardLevelDetailView+__c")]
    pub type __c = crate::GlobalNamespace::StandardLevelDetailView___c;
    #[cfg(feature = "StandardLevelDetailView+__c__DisplayClass44_0")]
    pub type __c__DisplayClass44_0 = crate::GlobalNamespace::StandardLevelDetailView___c__DisplayClass44_0;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateAndSetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateAndSetContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateAndSetContentAsync(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("CalculateAndSetContentAsync", (beatmapKey, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIfBeatmapLevelDataExists(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIfBeatmapLevelDataExists", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBeatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBeatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBeatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("CreateBeatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapCharacteristicSegmentedControlControllerDidSelectBeatmapCharacteristic(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSegmentedControlController,
        >,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCharacteristicSegmentedControlControllerDidSelectBeatmapCharacteristic",
                (controller, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeatmapDifficultySegmentedControlControllerDidSelectDifficulty(
        &mut self,
        controller: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDifficultySegmentedControlController,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapDifficultySegmentedControlControllerDidSelectDifficulty",
                (controller, difficulty),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidPressRefreshButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidPressRefreshButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBeatmapLevelVersions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeatmapLevelVersions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContentForBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContentForBeatmapData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContent_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContent_BeatmapLevel_BeatmapDifficultyMask_HashSet_1_BeatmapDifficulty_BeatmapCharacteristicSO_PlayerData0(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        allowedBeatmapDifficultyMask: crate::GlobalNamespace::BeatmapDifficultyMask,
        notAllowedCharacteristics: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        >,
        defaultDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        defaultBeatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        notesCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
        songLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (notesCount, obstaclesCount, bombsCount, songLength))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TriggerEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__46_0(
        &mut self,
        _cordl__: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Awake>b__46_0", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn _CalculateAndSetContentAsync_b__61_0(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<CalculateAndSetContentAsync>b__61_0", (value))?;
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
            crate::System::Action_1<*mut crate::GlobalNamespace::StandardLevelDetailView>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didFavoriteToggleChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_actionButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button> = __cordl_object
            .invoke("get_actionButton", ())?;
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
    pub fn get_practiceButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button> = __cordl_object
            .invoke("get_practiceButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeDifficultyBeatmapEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::StandardLevelDetailView>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeDifficultyBeatmapEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFavoriteToggleChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::UI::Toggle>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFavoriteToggleChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_actionButtonText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_actionButtonText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapKey(
        &mut self,
        value: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelDetailView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelDetailView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
