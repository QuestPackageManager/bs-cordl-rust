#[cfg(feature = "MissionLevelNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelNoTransitionInstaller {
    __cordl_parent: crate::Zenject::NoTransitionInstaller,
    pub _beatmapLevelSo: *mut BeatmapLevelSO,
    pub _beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub _beatmapDifficulty: BeatmapDifficulty,
    pub _colorScheme: *mut ColorSchemeSO,
    pub _missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjective,
    >,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _playerSpecificSettings: *mut PlayerSpecificSettings,
    pub _backButtonText: *mut crate::System::String,
    pub _scenesTransitionSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionLevelNoTransitionInstaller => ""
    ."MissionLevelNoTransitionInstaller"
);
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl std::ops::Deref for MissionLevelNoTransitionInstaller {
    type Target = crate::Zenject::NoTransitionInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl std::ops::DerefMut for MissionLevelNoTransitionInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl MissionLevelNoTransitionInstaller {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
impl quest_hook::libil2cpp::ObjectType for MissionLevelNoTransitionInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
