#[cfg(feature = "MissionGameplaySceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionGameplaySceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MissionObjective,
    >,
    pub autoRestart: bool,
    pub beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
    pub gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub backButtonText: *mut crate::System::String,
}
#[cfg(feature = "MissionGameplaySceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionGameplaySceneSetupData
    => ""."MissionGameplaySceneSetupData"
);
#[cfg(feature = "MissionGameplaySceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::MissionGameplaySceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionGameplaySceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionGameplaySceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionGameplaySceneSetupData")]
impl crate::GlobalNamespace::MissionGameplaySceneSetupData {
    pub fn New(
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MissionObjective,
        >,
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        backButtonText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    missionObjectives,
                    autoRestart,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    backButtonText,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MissionObjective,
        >,
        autoRestart: bool,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
        backButtonText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    missionObjectives,
                    autoRestart,
                    beatmapKey,
                    beatmapLevel,
                    gameplayModifiers,
                    backButtonText,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionGameplaySceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionGameplaySceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
