#[cfg(feature = "PracticeViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct PracticeViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _songStartSlider: *mut crate::HMUI::TimeSlider,
    pub _speedSlider: *mut crate::HMUI::PercentSlider,
    pub _levelBar: *mut LevelBar,
    pub _playButton: *mut crate::UnityEngine::UI::Button,
    pub _loader: *mut crate::UnityEngine::GameObject,
    pub _value: *mut crate::TMPro::TextMeshProUGUI,
    pub _enabledColor: crate::UnityEngine::Color,
    pub _disabledColor: crate::UnityEngine::Color,
    pub _playerDataModel: *mut PlayerDataModel,
    pub _songPreviewPlayer: *mut SongPreviewPlayer,
    pub _perceivedLoudnessPerLevelModel: *mut PerceivedLoudnessPerLevelModel,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub _beatmapLevelsEntitlementModel: *mut BeatmapLevelsEntitlementModel,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub didPressPlayButtonEvent: *mut crate::System::Action,
    pub _practiceSettings: *mut PracticeSettings,
    pub _currentPlayingStartTime: f32,
    pub _maxStartSongTime: f32,
    pub _songLength: f32,
    pub _getAudioClipTask: *mut crate::System::Threading::Tasks::Task,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _beatmapKey: BeatmapKey,
    pub _beatmapLevel: *mut BeatmapLevel,
    pub _audioClip: *mut crate::UnityEngine::AudioClip,
}
#[cfg(feature = "PracticeViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PracticeViewController => ""."PracticeViewController"
);
#[cfg(feature = "PracticeViewController")]
impl std::ops::Deref for PracticeViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PracticeViewController")]
impl std::ops::DerefMut for PracticeViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PracticeViewController")]
impl PracticeViewController {
    pub const kMinValueChangeToInstantPlayPreview: f32 = 3f32;
    pub const kWaitBeforePlayPreviewAfterPreviewStartValueChanged: f32 = 1f32;
    #[cfg(feature = "PracticeViewController+_LoadSong_d__33")]
    pub type _LoadSong_d__33 = crate::GlobalNamespace::PracticeViewController__LoadSong_d__33;
    pub fn CancelSongLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelSongLoading", ())?;
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
    pub fn GetSongTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetSongTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSongStartSliderValueDidChange(
        &mut self,
        slider: *mut crate::HMUI::RangeValuesTextSlider,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongStartSliderValueDidChange", (slider, value))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (beatmapKey, beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSong(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("LoadSong", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PlayPreview(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayPreview", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLoader(
        &mut self,
        loading: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLoader", (loading))?;
        Ok(__cordl_ret)
    }
    pub fn SetSongLength(
        &mut self,
        songLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSongLength", (songLength))?;
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
    pub fn add_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PracticeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PracticeSettings = __cordl_object
            .invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PracticeViewController")]
impl quest_hook::libil2cpp::ObjectType for PracticeViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
