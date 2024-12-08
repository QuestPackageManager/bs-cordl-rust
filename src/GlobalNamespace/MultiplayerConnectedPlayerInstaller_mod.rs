#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _connectedPlayerAudioTimeSyncControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController,
    pub _connectedPlayerBeatmapObjectEventManagerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager,
    pub _multiplayerGameNoteControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    pub _multiplayerBurstSliderHeadGameNoteControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    pub _multiplayerBurstSliderGameNoteControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    pub _multiplayerBombNoteControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController,
    pub _multiplayerObstacleControllerPrefab: *mut crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController,
    pub _connectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _localPlayerStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    pub _sceneSetupData: *mut crate::GlobalNamespace::GameplayCoreSceneSetupData,
    pub _playersSpecificSettingsAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
}
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerConnectedPlayerInstaller => ""
    ."MultiplayerConnectedPlayerInstaller"
);
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerConnectedPlayerInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerConnectedPlayerInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
