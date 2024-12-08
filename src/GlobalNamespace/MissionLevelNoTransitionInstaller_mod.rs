#[cfg(feature = "MissionLevelNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelNoTransitionInstaller {
    __cordl_parent: crate::Zenject::NoTransitionInstaller,
    pub _beatmapLevelSo: *mut crate::GlobalNamespace::BeatmapLevelSO,
    pub _beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _colorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    pub _missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MissionObjective,
    >,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub _playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    pub _backButtonText: *mut crate::System::String,
    pub _scenesTransitionSetupData: *mut crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelNoTransitionInstaller => ""
    ."MissionLevelNoTransitionInstaller"
);
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelNoTransitionInstaller {
    type Target = crate::Zenject::NoTransitionInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelNoTransitionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl crate::GlobalNamespace::MissionLevelNoTransitionInstaller {
    pub fn InstallBindings(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", (container))?;
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
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelNoTransitionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
