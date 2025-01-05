#[cfg(feature = "AudioLatencyViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioLatencyViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _setupCanvasGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::CanvasGroup>,
    pub _volumeSettingsList: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _ambientSettingsList: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FormattedFloatListSettingsController,
    >,
    pub _overrideAudioLatencyToggle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Toggle,
    >,
    pub _slider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
    pub _visualMetronome: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VisualMetronome,
    >,
    pub _disabledAlpha: f32,
    pub _songPreviewPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _toggleBinder: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleBinder>,
}
#[cfg(feature = "AudioLatencyViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioLatencyViewController =>
    ""."AudioLatencyViewController"
);
#[cfg(feature = "AudioLatencyViewController")]
impl std::ops::Deref for crate::GlobalNamespace::AudioLatencyViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioLatencyViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioLatencyViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioLatencyViewController")]
impl crate::GlobalNamespace::AudioLatencyViewController {
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
    pub fn HandleAmbientVolumeChanged(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
        newValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAmbientVolumeChanged", (_cordl__, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleOverrideAudioLatencyToggleValueChanged(
        &mut self,
        isOn: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOverrideAudioLatencyToggleValueChanged", (isOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVolumeChanged(
        &mut self,
        _cordl__: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FormattedFloatListSettingsController,
        >,
        newValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVolumeChanged", (_cordl__, newValue))?;
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
    pub fn RefreshVisuals(
        &mut self,
        overrideAudioLatencyIsEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", (overrideAudioLatencyIsEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderValueDidChange(
        &mut self,
        slider: quest_hook::libil2cpp::Gc<crate::HMUI::RangeValuesTextSlider>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SliderValueDidChange", (slider, value))?;
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
}
#[cfg(feature = "AudioLatencyViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioLatencyViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
