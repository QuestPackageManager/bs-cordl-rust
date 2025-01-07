#[cfg(feature = "PlayerSettingsPanelController")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSettingsPanelController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftHandedToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _reduceDebrisToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _noTextsAndHudsToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _advanceHudToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _playerHeightSettingsController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerHeightSettingsController,
    >,
    pub _playerHeightSettingsCanvasGroup: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CanvasGroup,
    >,
    pub _automaticPlayerHeightToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _sfxVolumeSettingsController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _saberTrailIntensitySettingsController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _noteJumpDurationTypeSettingsDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteJumpDurationTypeSettingsDropdown,
    >,
    pub _noteJumpFixedDurationSettingsController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _noteJumpFixedDurationSettingsCanvasGroup: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CanvasGroup,
    >,
    pub _noteJumpStartBeatOffsetDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteJumpStartBeatOffsetDropdown,
    >,
    pub _noteJumpStartBeatOffsetCanvasGroup: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CanvasGroup,
    >,
    pub _environmentEffectsFilterDefaultPresetDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown,
    >,
    pub _environmentEffectsFilterExpertPlusPresetDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentEffectsFilterPresetDropdown,
    >,
    pub _hideNoteSpawnEffectToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _adaptiveSfxToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _autoRestartToggle: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Toggle>,
    pub _headsetHapticIntensityController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _arcsVisibilityTypeSettingsDropdown: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ArcVisibilityTypeSettingsDropdown,
    >,
    pub _arcHapticFeedbackCanvasGroup: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CanvasGroup,
    >,
    pub _arcsHapticFeedbackToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _singleplayerOnlyCanvasGroup: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::CanvasGroup,
    >,
    pub _arcVisibilityWarning: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub didChangePlayerSettingsEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _currentArcType: crate::GlobalNamespace::ArcVisibilityType,
    pub _playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    pub _playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSpecificSettings,
    >,
    pub _toggleBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleBinder>,
    pub _dirty: bool,
    pub _refreshed: bool,
    pub _eventBinder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EventBinder>,
}
#[cfg(feature = "PlayerSettingsPanelController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSettingsPanelController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSettingsPanelController";
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleHeadsetHapticIntensityControllerValueDidChange(
        &mut self,
        settingsController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleNoteJumpFixedDurationSettingsControllerValueDidChange(
        &mut self,
        formattedFloatListSettingsController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSFXVolumeSettingsControllerValueDidChange(
        &mut self,
        settingsController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleSaberTrailIntensitySettingsControllerValueDidChange(
        &mut self,
        settingsController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
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
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (playerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIsDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsDirty", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetSectionDisabled(
        &mut self,
        sectionCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
        disable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSectionDisabled", (sectionCanvasGroup, disable))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeAllUICallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeAllUICallbacks", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__42_7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_7", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _OnEnable_b__42_8(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<OnEnable>b__42_8", ())?;
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
    pub fn add_didChangePlayerSettingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangePlayerSettingsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        > = __cordl_object.invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangePlayerSettingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangePlayerSettingsEvent", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "PlayerSettingsPanelController")]
impl AsRef<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::PlayerSettingsPanelController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerSettingsPanelController")]
impl AsMut<crate::GlobalNamespace::IRefreshable>
for crate::GlobalNamespace::PlayerSettingsPanelController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IRefreshable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerSettingsPanelController_PlayerSettingsPanelLayout {
    #[default]
    All = 0i32,
    Multiplayer = 2i32,
    Singleplayer = 1i32,
}
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerSettingsPanelLayout";
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
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout {
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
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout {
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
#[cfg(feature = "PlayerSettingsPanelController+PlayerSettingsPanelLayout")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlayerSettingsPanelController_PlayerSettingsPanelLayout {
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
