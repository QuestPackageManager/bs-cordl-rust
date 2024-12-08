#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerSongSyncController_InitData {
    __cordl_parent: crate::System::Object,
    pub audioClip: *mut crate::UnityEngine::AudioClip,
    pub startSongTime: f32,
    pub songTimeOffset: f32,
    pub timeScale: f32,
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData => ""
    ."MultiplayerLocalInactivePlayerSongSyncController/InitData"
);
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
impl crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData {
    pub fn New(
        audioClip: *mut crate::UnityEngine::AudioClip,
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (audioClip, startSongTime, songTimeOffset, timeScale),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioClip, startSongTime, songTimeOffset, timeScale))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalInactivePlayerSongSyncController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSource: *mut CrossFadeAudioSource,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _initData: *mut crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _vrPlatformHelper: *mut IVRPlatformHelper,
    pub _timeScale: f32,
    pub _startSongTime: f32,
    pub _songTimeOffset: f32,
    pub _songTime: f32,
    pub _audioStarted: bool,
    pub _currentObservableIsFailed: bool,
    pub _observable: *mut IMultiplayerObservable,
    pub _lastLatencyOffsetTime: i64,
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalInactivePlayerSongSyncController => ""
    ."MultiplayerLocalInactivePlayerSongSyncController"
);
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
impl std::ops::Deref for MultiplayerLocalInactivePlayerSongSyncController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
impl std::ops::DerefMut for MultiplayerLocalInactivePlayerSongSyncController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
impl MultiplayerLocalInactivePlayerSongSyncController {
    pub const kReSyncThresholdMs: i64 = 50i64;
    #[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController+InitData")]
    pub type InitData = crate::GlobalNamespace::MultiplayerLocalInactivePlayerSongSyncController_InitData;
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
    pub fn FollowOffsetSyncTime(
        &mut self,
        observable: *mut IMultiplayerObservable,
        crossFade: bool,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FollowOffsetSyncTime", (observable, crossFade, forceUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn HandleVrFocusWasReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVrFocusWasReleased", ())?;
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
    pub fn SeekTo__cordl_bool_f32_1(
        &mut self,
        offsetTime: f32,
        crossFade: bool,
        toVolume: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SeekTo", (offsetTime, crossFade, toVolume))?;
        Ok(__cordl_ret)
    }
    pub fn SeekTo_f32_0(
        &mut self,
        offsetTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SeekTo", (offsetTime))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartSong(
        &mut self,
        offsetTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", (offsetTime))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateOffsetSyncTime(
        &mut self,
        offsetSyncTime: i64,
        crossFade: bool,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateOffsetSyncTime", (offsetSyncTime, crossFade, forceUpdate))?;
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
    pub fn _get_waitUntilIsReadyToStartTheSong_b__8_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<get_waitUntilIsReadyToStartTheSong>b__8_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isAudioLoaded(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isAudioLoaded", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_waitUntilIsReadyToStartTheSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::WaitUntil> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::WaitUntil = __cordl_object
            .invoke("get_waitUntilIsReadyToStartTheSong", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLocalInactivePlayerSongSyncController")]
impl quest_hook::libil2cpp::ObjectType
for MultiplayerLocalInactivePlayerSongSyncController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
