#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _connectedPlayerAudioTimeSyncControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerSongTimeSyncController,
    >,
    pub _connectedPlayerBeatmapObjectEventManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerBeatmapObjectEventManager,
    >,
    pub _multiplayerGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _multiplayerBurstSliderHeadGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _multiplayerBurstSliderGameNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerGameNoteController,
    >,
    pub _multiplayerBombNoteControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerBombNoteController,
    >,
    pub _multiplayerObstacleControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerConnectedPlayerObstacleController,
    >,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _localPlayerStartState: crate::GlobalNamespace::MultiplayerPlayerStartState,
    pub _sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
    pub _playersSpecificSettingsAtGameStartModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerConnectedPlayerInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerConnectedPlayerInstaller";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InstallBindings", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
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
