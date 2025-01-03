#[cfg(feature = "GameplayCoreInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayCoreInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _beatLineManagerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatLineManager,
    >,
    pub _songTimeTweeningManager: quest_hook::libil2cpp::Gc<
        crate::Tweening::SongTimeTweeningManager,
    >,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _playerHeightDetectorPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerHeightDetector,
    >,
    pub _noteCutScoreSpawnerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NoteCutScoreSpawner,
    >,
    pub _badNoteCutEffectSpawnerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BadNoteCutEffectSpawner,
    >,
    pub _missedNoteEffectSpawnerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissedNoteEffectSpawner,
    >,
    pub _effectPoolsManualInstaller: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EffectPoolsManualInstaller,
    >,
    pub _arcAndObstacleHapticManagerEffectPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ArcAndObstacleHapticEffectManager,
    >,
    pub _songProfilingControllerPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongProfilingController,
    >,
    pub _sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
    pub _perceivedLoudnessPerLevelModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PerceivedLoudnessPerLevelModel,
    >,
    pub _commandLineArguments: crate::BGLib::DotnetExtension::CommandLine::CommandLineParserResult,
    pub _performanceToolConfig: crate::System::Nullable_1<
        crate::GlobalNamespace::PerformanceToolLauncher_OverrideConfig,
    >,
}
#[cfg(feature = "GameplayCoreInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameplayCoreInstaller => ""
    ."GameplayCoreInstaller"
);
#[cfg(feature = "GameplayCoreInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::GameplayCoreInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameplayCoreInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl crate::GlobalNamespace::GameplayCoreInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameplayCoreInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayCoreInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
