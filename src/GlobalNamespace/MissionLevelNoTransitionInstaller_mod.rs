#[cfg(feature = "MissionLevelNoTransitionInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelNoTransitionInstaller {
    __cordl_parent: crate::Zenject::NoTransitionInstaller,
    pub _beatmapLevelSo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelSO,
    >,
    pub _beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _colorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSchemeSO>,
    pub _missionObjectives: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
        >,
    >,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _playerSpecificSettings: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerSpecificSettings,
    >,
    pub _backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
    >,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
}
#[cfg(feature = "MissionLevelNoTransitionInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MissionLevelNoTransitionInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionLevelNoTransitionInstaller";
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
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InstallBindings")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InstallBindings", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
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
