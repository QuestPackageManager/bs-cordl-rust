#[cfg(feature = "PlayerSettingsPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSettingsPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftHandedToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _reduceDebrisToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _noTextsAndHudsToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _advanceHudToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _autoRestartToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _playerHeightSettingsController: *mut crate::GlobalNamespace::PlayerHeightSettingsController,
    pub _playerHeightSettingsCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _automaticPlayerHeightToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _sfxVolumeSettingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
    pub _saberTrailIntensitySettingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
    pub _noteJumpDurationTypeSettingsDropdown: *mut crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown,
    pub _noteJumpFixedDurationSettingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
    pub _noteJumpFixedDurationSettingsCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _noteJumpStartBeatOffsetDropdown: *mut crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown,
    pub _noteJumpStartBeatOffsetCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _environmentEffectsFilterDefaultPresetDropdown: *mut crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown,
    pub _environmentEffectsFilterExpertPlusPresetDropdown: *mut crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown,
    pub _hideNoteSpawnEffectToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _adaptiveSfxToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _headsetHapticIntensityController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
    pub _arcsVisibilityTypeSettingsDropdown: *mut crate::GlobalNamespace::ArcVisibilityTypeSettingsDropdown,
    pub _arcHapticFeedbackCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _arcsHapticFeedbackToggle: *mut crate::UnityEngine::UI::Toggle,
    pub _singleplayerOnlyCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _arcVisibilityWarning: *mut crate::UnityEngine::GameObject,
    pub didChangePlayerSettingsEvent: *mut crate::System::Action,
    pub _currentArcType: crate::GlobalNamespace::ArcVisibilityType,
    pub _playerData: *mut crate::GlobalNamespace::PlayerData,
    pub _playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    pub _toggleBinder: *mut crate::HMUI::ToggleBinder,
    pub _dirty: bool,
    pub _refreshed: bool,
    pub _eventBinder: *mut crate::GlobalNamespace::EventBinder,
}
#[cfg(feature = "PlayerSettingsPanelController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerSettingsPanelController
    => ""."PlayerSettingsPanelController"
);
#[cfg(feature = "PlayerSettingsPanelController")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerSettingsPanelController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSettingsPanelController")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerSettingsPanelController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSettingsPanelController")]
impl crate::GlobalNamespace::PlayerSettingsPanelController {
    pub const kDisabledSectionAlpha: f32 = 0.2f32;
    #[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
    pub type PlayerSettingsPanelLayout = crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout;
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
    pub fn HandleAdvancedHudToggleChanged(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdvancedHudToggleChanged", (on))?;
        Ok(__cordl_ret)
    }
    pub fn HandleArcVisibilityDropdownDidSelectCellWithIdx(
        &mut self,
        idx: i32,
        arcVisibilityType: crate::GlobalNamespace::ArcVisibilityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleArcVisibilityDropdownDidSelectCellWithIdx",
                (idx, arcVisibilityType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleHeadsetHapticIntensityControllerValueDidChange(
        &mut self,
        settingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleHeadsetHapticIntensityControllerValueDidChange",
                (settingsController, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleLightReductionAmountSelected(
        &mut self,
        obj: i32,
        environmentEffectsFilterPreset: crate::GlobalNamespace::EnvironmentEffectsFilterPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLightReductionAmountSelected",
                (obj, environmentEffectsFilterPreset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoTextsAndHudsToggleChanged(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleNoTextsAndHudsToggleChanged", (on))?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteJumpDurationTypeSettingsDropdownDidSelectCellWithIdx(
        &mut self,
        idx: i32,
        noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteJumpDurationTypeSettingsDropdownDidSelectCellWithIdx",
                (idx, noteJumpDurationTypeSettings),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteJumpFixedDurationSettingsControllerValueDidChange(
        &mut self,
        formattedFloatListSettingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteJumpFixedDurationSettingsControllerValueDidChange",
                (formattedFloatListSettingsController, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleNoteJumpStartBeatOffsetPositionSelected(
        &mut self,
        idx: i32,
        startBeatOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleNoteJumpStartBeatOffsetPositionSelected",
                (idx, startBeatOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerHeightSettingsControllerValueDidChange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerHeightSettingsControllerValueDidChange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSFXVolumeSettingsControllerValueDidChange(
        &mut self,
        settingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSFXVolumeSettingsControllerValueDidChange",
                (settingsController, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleSaberTrailIntensitySettingsControllerValueDidChange(
        &mut self,
        settingsController: *mut crate::GlobalNamespace::FormattedFloatListSettingsController,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSaberTrailIntensitySettingsControllerValueDidChange",
                (settingsController, value),
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
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret)
    }
    pub fn RefreshArcsWarning(
        &mut self,
        arcVisibilityType: crate::GlobalNamespace::ArcVisibilityType,
        forceRebuild: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshArcsWarning", (arcVisibilityType, forceRebuild))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshNoteJumpUI(
        &mut self,
        noteJumpDurationTypeSettings: crate::GlobalNamespace::NoteJumpDurationTypeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshNoteJumpUI", (noteJumpDurationTypeSettings))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        playerData: *mut crate::GlobalNamespace::PlayerData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (playerData))?;
        Ok(__cordl_ret)
    }
    pub fn SetIsDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLayout(
        &mut self,
        layout: crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayout", (layout))?;
        Ok(__cordl_ret)
    }
    pub fn SetSectionDisabled(
        &mut self,
        sectionCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
        disable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSectionDisabled", (sectionCanvasGroup, disable))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeAllUICallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeAllUICallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_0(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_0", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_1(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_1", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_2(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_2", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_3(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_3", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_4(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_4", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_5(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_5", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_6(
        &mut self,
        on: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_6", (on))?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_7", ())?;
        Ok(__cordl_ret)
    }
    pub fn _OnEnable_b__42_8(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_8", ())?;
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
    pub fn add_didChangePlayerSettingsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangePlayerSettingsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerSpecificSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didChangePlayerSettingsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangePlayerSettingsEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerSettingsPanelController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSettingsPanelController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSettingsPanelController_PlayerSettingsPanelLayout {
    All = 0i32,
    Multiplayer = 2i32,
    Singleplayer = 1i32,
}
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout => ""
    ."PlayerSettingsPanelController/PlayerSettingsPanelLayout"
);
