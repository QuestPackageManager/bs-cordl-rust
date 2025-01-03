#[cfg(feature = "AudioTimeSyncController")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTimeSyncController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSyncLerpSpeed: f32,
    pub _forcedSyncDeltaTime: f32,
    pub _startSyncDeltaTime: f32,
    pub _stopSyncDeltaTime: f32,
    pub _audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController_InitData,
    >,
    pub _dspTimeProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IDspTimeProvider,
    >,
    pub forcedNoAudioSync: bool,
    pub stateChangedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _audioLatency: f32,
    pub _fixingAudioSyncError: bool,
    pub _audioStartTimeOffsetSinceStart: f32,
    pub _playbackLoopIndex: i32,
    pub _prevAudioSamplePos: i32,
    pub _startSongTime: f32,
    pub _songTimeOffset: f32,
    pub _audioStarted: bool,
    pub _timeScale: f32,
    pub _songTime: f32,
    pub _dspTimeOffset: f64,
    pub _state: crate::GlobalNamespace::AudioTimeSyncController_State,
    pub _songLoadingStarted: bool,
    pub _canStartSong: bool,
    pub _isReady: bool,
    pub _lastFrameDeltaSongTime: f32,
    pub _forceNoAudioSyncOrAudioSyncErrorFixing: bool,
    pub _inBetweenDSPBufferingTimeEstimate: f32,
}
#[cfg(feature = "AudioTimeSyncController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioTimeSyncController => ""
    ."AudioTimeSyncController"
);
#[cfg(feature = "AudioTimeSyncController")]
impl std::ops::Deref for crate::GlobalNamespace::AudioTimeSyncController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTimeSyncController")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioTimeSyncController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTimeSyncController")]
impl crate::GlobalNamespace::AudioTimeSyncController {
    #[cfg(feature = "AudioTimeSyncController+InitData")]
    pub type InitData = crate::GlobalNamespace::AudioTimeSyncController_InitData;
    #[cfg(feature = "AudioTimeSyncController+State")]
    pub type State = crate::GlobalNamespace::AudioTimeSyncController_State;
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
    pub fn HandleAudioConfigurationChanged(
        &mut self,
        deviceWasChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAudioConfigurationChanged", (deviceWasChanged))?;
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
    pub fn Pause(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Pause", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resume(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SeekTo(
        &mut self,
        startTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SeekTo", (startTimeOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartSong(
        &mut self,
        startTimeOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", (startTimeOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn StopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSong", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn _get_waitUntilAudioIsLoaded_b__32_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<get_waitUntilAudioIsLoaded>b__32_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_stateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_stateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dspTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_dspTimeOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forceNoAudioSyncOrAudioSyncErrorFixing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_forceNoAudioSyncOrAudioSyncErrorFixing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isAudioLoaded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isAudioLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastFrameDeltaSongTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lastFrameDeltaSongTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songEndTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songEndTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songFailedToLoad(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_songFailedToLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTimeOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_startSongTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_startSongTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::AudioTimeSyncController_State,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AudioTimeSyncController_State = __cordl_object
            .invoke("get_state", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_timeSinceStart(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeSinceStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_waitUntilAudioIsLoaded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil> = __cordl_object
            .invoke("get_waitUntilAudioIsLoaded", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_stateChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_stateChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_forceNoAudioSyncOrAudioSyncErrorFixing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_forceNoAudioSyncOrAudioSyncErrorFixing", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioTimeSyncController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioTimeSyncController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AudioTimeSyncController")]
impl AsRef<crate::GlobalNamespace::IAudioTimeSource>
for crate::GlobalNamespace::AudioTimeSyncController {
    fn as_ref(&self) -> &crate::GlobalNamespace::IAudioTimeSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AudioTimeSyncController")]
impl AsMut<crate::GlobalNamespace::IAudioTimeSource>
for crate::GlobalNamespace::AudioTimeSyncController {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IAudioTimeSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AudioTimeSyncController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioTimeSyncController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub startSongTime: f32,
    pub songTimeOffset: f32,
    pub timeScale: f32,
}
#[cfg(feature = "AudioTimeSyncController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AudioTimeSyncController_InitData => ""
    ."AudioTimeSyncController/InitData"
);
#[cfg(feature = "AudioTimeSyncController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::AudioTimeSyncController_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTimeSyncController+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::AudioTimeSyncController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AudioTimeSyncController+InitData")]
impl crate::GlobalNamespace::AudioTimeSyncController_InitData {
    pub fn New(
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (audioClip, startSongTime, songTimeOffset, timeScale),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioClip, startSongTime, songTimeOffset, timeScale))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AudioTimeSyncController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AudioTimeSyncController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AudioTimeSyncController+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioTimeSyncController_State {
    Paused = 1i32,
    Playing = 0i32,
    Stopped = 2i32,
}
#[cfg(feature = "AudioTimeSyncController+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AudioTimeSyncController_State
    => ""."AudioTimeSyncController/State"
);
