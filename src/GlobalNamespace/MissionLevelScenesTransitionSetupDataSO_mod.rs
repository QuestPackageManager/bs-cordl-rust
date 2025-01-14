#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
    pub _missionGameplaySceneInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SceneInfo,
    >,
    pub _gameCoreSceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionCompletionResults>,
        >,
    >,
    pub _missionId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionLevelScenesTransitionSetupDataSO";
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
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref
for crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO {
    pub fn Finish(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionCompletionResults,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Finish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Finish", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (levelCompletionResults))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init_ByRefMut_BeatmapLevel_Il2CppArray_ColorScheme_GameplayModifiers_PlayerSpecificSettings_EnvironmentsListModel_BeatmapLevelsModel1(
        &mut self,
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        missionObjectives: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
            >,
        >,
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionObjective,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentsListModel,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapLevelsModel,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::AudioClipAsyncLoader,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataLoader>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                13usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 13usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        missionId,
                        beatmapKey,
                        beatmapLevel,
                        missionObjectives,
                        overrideColorScheme,
                        gameplayModifiers,
                        playerSpecificSettings,
                        environmentsListModel,
                        beatmapLevelsModel,
                        audioClipAsyncLoader,
                        settingsManager,
                        beatmapDataLoader,
                        backButtonText,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init_IBeatmapLevelData_ByRefMut_BeatmapLevel_Il2CppArray_ColorScheme_GameplayModifiers_PlayerSpecificSettings_EnvironmentsListModel0(
        &mut self,
        missionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        missionObjectives: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjective>,
            >,
        >,
        overrideColorScheme: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        >,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SettingsManager,
        >,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataLoader,
        >,
        backButtonText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
                    quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MissionObjective,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlayerSpecificSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::EnvironmentsListModel,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::AudioClipAsyncLoader,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataLoader>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                13usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 13usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        missionId,
                        beatmapLevelData,
                        beatmapKey,
                        beatmapLevel,
                        missionObjectives,
                        overrideColorScheme,
                        gameplayModifiers,
                        playerSpecificSettings,
                        environmentsListModel,
                        audioClipAsyncLoader,
                        settingsManager,
                        beatmapDataLoader,
                        backButtonText,
                    ),
                )
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MissionCompletionResults,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_missionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_missionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_missionId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MissionCompletionResults,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MissionCompletionResults,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_didFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_didFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_missionId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_missionId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_missionId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
