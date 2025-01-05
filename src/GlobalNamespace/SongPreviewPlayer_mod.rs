#[cfg(feature = "SongPreviewPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPreviewPlayer {
    __cordl_parent: crate::GlobalNamespace::AudioPlayerBase,
    pub _channelsCount: i32,
    pub _audioSourcePrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _defaultAudioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    pub _volume: f32,
    pub _crossFadeToAnotherSongSpeed: f32,
    pub _crossFadeToDefaultSpeed: f32,
    pub _fadeInSpeed: f32,
    pub _defaultAudioSourceParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams,
    >,
    pub _othersAudioSourceParams: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer_InitData,
    >,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _audioSourceControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController,
        >,
    >,
    pub _activeChannel: i32,
    pub _timeToDefaultAudioTransition: f32,
    pub _transitionAfterDelay: bool,
    pub _volumeScale: f32,
    pub _fadeSpeed: f32,
    pub _ambientVolumeScale: f32,
    pub _isActiveChannelPaused: bool,
    pub _channelToFadeOutCallbackDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
    >,
}
#[cfg(feature = "SongPreviewPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongPreviewPlayer => ""
    ."SongPreviewPlayer"
);
#[cfg(feature = "SongPreviewPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::SongPreviewPlayer {
    type Target = crate::GlobalNamespace::AudioPlayerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongPreviewPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer")]
impl crate::GlobalNamespace::SongPreviewPlayer {
    #[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
    pub type AudioSourceParams = crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams;
    #[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
    pub type AudioSourceVolumeController = crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController;
    #[cfg(feature = "SongPreviewPlayer+InitData")]
    pub type InitData = crate::GlobalNamespace::SongPreviewPlayer_InitData;
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
    pub fn CrossFadeAfterDelayCoroutine(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("CrossFadeAfterDelayCoroutine", (delay))?;
        Ok(__cordl_ret.into())
    }
    pub fn CrossfadeToDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossfadeToDefault", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CrossfadeToNewDefault(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CrossfadeToNewDefault", (audioClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CrossfadeTo_Action0(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        musicVolume: f32,
        startTime: f32,
        duration: f32,
        onFadeOutCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossfadeTo",
                (audioClip, musicVolume, startTime, duration, onFadeOutCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CrossfadeTo__cordl_bool_Action1(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
        musicVolume: f32,
        startTime: f32,
        duration: f32,
        isDefault: bool,
        onFadeOutCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CrossfadeTo",
                (
                    audioClip,
                    musicVolume,
                    startTime,
                    duration,
                    isDefault,
                    onFadeOutCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FadeOut(
        &mut self,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FadeOut", (duration))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn PauseCurrentChannel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseCurrentChannel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportChannelDidFadeOut(
        &mut self,
        channel: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportChannelDidFadeOut", (channel))?;
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
    pub fn UnPauseCurrentChannel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnPauseCurrentChannel", ())?;
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
    pub fn get_activeAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = __cordl_object
            .invoke("get_activeAudioClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = __cordl_object
            .invoke("get_defaultAudioClip", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPreviewPlayer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongPreviewPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPreviewPlayer_AudioSourceParams {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _position: crate::UnityEngine::Vector3,
    pub _reverbZoneMix: f32,
    pub _spatialBlend: f32,
    pub _spread: f32,
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SongPreviewPlayer_AudioSourceParams => ""
    ."SongPreviewPlayer/AudioSourceParams"
);
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
impl std::ops::Deref for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
impl crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_reverbZoneMix(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_reverbZoneMix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spatialBlend(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spatialBlend", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spread(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_spread", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPreviewPlayer_AudioSourceVolumeController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    pub _maxVolume_k__BackingField: f32,
    pub _volume: f32,
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController => ""
    ."SongPreviewPlayer/AudioSourceVolumeController"
);
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
impl std::ops::Deref
for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
impl crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController {
    pub fn New(
        audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioSource))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        audioSource: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioSource>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioSource))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxVolume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxVolume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_volume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxVolume(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxVolume", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_volume(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_volume", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPreviewPlayer+AudioSourceVolumeController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongPreviewPlayer_AudioSourceVolumeController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongPreviewPlayer+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPreviewPlayer_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ambientVolumeScale: f32,
}
#[cfg(feature = "SongPreviewPlayer+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongPreviewPlayer_InitData =>
    ""."SongPreviewPlayer/InitData"
);
#[cfg(feature = "SongPreviewPlayer+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::SongPreviewPlayer_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongPreviewPlayer_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPreviewPlayer+InitData")]
impl crate::GlobalNamespace::SongPreviewPlayer_InitData {
    pub fn New(
        ambientVolumeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ambientVolumeScale))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        ambientVolumeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ambientVolumeScale))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPreviewPlayer+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongPreviewPlayer_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
