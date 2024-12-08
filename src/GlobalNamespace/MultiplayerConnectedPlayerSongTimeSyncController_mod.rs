#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerSongTimeSyncController_InitData {
    __cordl_parent: crate::System::Object,
    pub startSongTime: f32,
    pub songTimeOffset: f32,
    pub timeScale: f32,
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData => ""
    ."MultiplayerConnectedPlayerSongTimeSyncController/InitData"
);
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData {
    pub fn New(
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startSongTime, songTimeOffset, timeScale))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        startSongTime: f32,
        songTimeOffset: f32,
        timeScale: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startSongTime, songTimeOffset, timeScale))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerSongTimeSyncController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioSyncLerpSpeed: f32,
    pub _forcedSyncDeltaTime: f32,
    pub _startSyncDeltaTime: f32,
    pub _stopSyncDeltaTime: f32,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _initData: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _songTime_k__BackingField: f32,
    pub _lastFrameDeltaSongTime_k__BackingField: f32,
    pub _timeScale: f32,
    pub _startSongSyncTime: i64,
    pub _fixingAudioSyncError: bool,
    pub _isReady: bool,
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController => ""
    ."MultiplayerConnectedPlayerSongTimeSyncController"
);
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController {
    #[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController+InitData")]
    pub type InitData = crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController_InitData;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetConnectedPlayerSongTime(
        &mut self,
        syncTime: i64,
        songTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetConnectedPlayerSongTime", (syncTime, songTime))?;
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
        songStartSyncTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", (songStartSyncTime))?;
        Ok(__cordl_ret)
    }
    pub fn StopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSong", ())?;
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
    pub fn get_isReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastFrameDeltaSongTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lastFrameDeltaSongTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songEndTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songLength(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_timeScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_timeScale", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_lastFrameDeltaSongTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lastFrameDeltaSongTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_songTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_songTime", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSongTimeSyncController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
