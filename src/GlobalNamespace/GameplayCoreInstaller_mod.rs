#[cfg(feature = "GameplayCoreInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayCoreInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _beatLineManagerPrefab: *mut BeatLineManager,
    pub _songTimeTweeningManager: *mut crate::Tweening::SongTimeTweeningManager,
    pub _audioManager: *mut AudioManagerSO,
    pub _playerHeightDetectorPrefab: *mut PlayerHeightDetector,
    pub _noteCutScoreSpawnerPrefab: *mut NoteCutScoreSpawner,
    pub _badNoteCutEffectSpawnerPrefab: *mut BadNoteCutEffectSpawner,
    pub _missedNoteEffectSpawnerPrefab: *mut MissedNoteEffectSpawner,
    pub _effectPoolsManualInstaller: *mut EffectPoolsManualInstaller,
    pub _arcAndObstacleHapticManagerEffectPrefab: *mut ArcAndObstacleHapticEffectManager,
    pub _songProfilingControllerPrefab: *mut SongProfilingController,
    pub _sceneSetupData: *mut GameplayCoreSceneSetupData,
    pub _perceivedLoudnessPerLevelModel: *mut PerceivedLoudnessPerLevelModel,
    pub _commandLineArguments: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
}
#[cfg(feature = "GameplayCoreInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameplayCoreInstaller => ""."GameplayCoreInstaller"
);
#[cfg(feature = "GameplayCoreInstaller")]
impl std::ops::Deref for GameplayCoreInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl std::ops::DerefMut for GameplayCoreInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl GameplayCoreInstaller {
    pub fn InstallRecordingTool(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallRecordingTool", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl quest_hook::libil2cpp::ObjectType for GameplayCoreInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
