#[cfg(feature = "SongStartHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct SongStartHandler {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
    pub _playersAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    pub _readyPlayers: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _started: bool,
    pub _startTime: i64,
    pub setSongStartSyncTimeEvent: *mut crate::System::Action_1<i64>,
}
#[cfg(feature = "SongStartHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongStartHandler => ""
    ."SongStartHandler"
);
#[cfg(feature = "SongStartHandler")]
impl std::ops::Deref for crate::GlobalNamespace::SongStartHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongStartHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongStartHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongStartHandler")]
impl crate::GlobalNamespace::SongStartHandler {
    pub const kFixedStartDelayMs: i64 = 250i64;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn ForceStart(
        &mut self,
        sessionGameId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceStart", (sessionGameId))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelStartTimeOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetLevelStartTimeOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGetGameplaySongReady(
        &mut self,
        user: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGetGameplaySongReady", (user))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSetGameplaySongReady(
        &mut self,
        user: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetGameplaySongReady", (user))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSetSongStartTime(
        &mut self,
        user: *mut crate::System::String,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetSongStartTime", (user, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        playersAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, playersAtGameStartModel),
            )?;
        Ok(__cordl_object)
    }
    pub fn StartSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
        playersAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, playersAtGameStartModel),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_setSongStartSyncTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_setSongStartSyncTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_songStartSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_songStartSyncTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_setSongStartSyncTimeEvent(
        &mut self,
        value: *mut crate::System::Action_1<i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_setSongStartSyncTimeEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SongStartHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongStartHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
