#[cfg(feature = "PrepareLevelCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct PrepareLevelCompletionResults {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModelSO: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersModelSO,
    >,
    pub _saberActivityCounter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SaberActivityCounter,
    >,
    pub _beatmapObjectExecutionRatingsRecorder: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectExecutionRatingsRecorder,
    >,
    pub _scoreController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IScoreController,
    >,
    pub _gameEnergyCounter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameEnergyCounter,
    >,
    pub _beatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyBeatmapData,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _comboController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ComboController,
    >,
}
#[cfg(feature = "PrepareLevelCompletionResults")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PrepareLevelCompletionResults {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PrepareLevelCompletionResults";
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
#[cfg(feature = "PrepareLevelCompletionResults")]
impl std::ops::Deref for crate::GlobalNamespace::PrepareLevelCompletionResults {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PrepareLevelCompletionResults")]
impl std::ops::DerefMut for crate::GlobalNamespace::PrepareLevelCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PrepareLevelCompletionResults")]
impl crate::GlobalNamespace::PrepareLevelCompletionResults {
    pub fn FillLevelCompletionResults(
        &mut self,
        levelEndStateType: crate::GlobalNamespace::LevelCompletionResults_LevelEndStateType,
        levelEndAction: crate::GlobalNamespace::LevelCompletionResults_LevelEndAction,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LevelCompletionResults>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        > = __cordl_object
            .invoke("FillLevelCompletionResults", (levelEndStateType, levelEndAction))?;
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
#[cfg(feature = "PrepareLevelCompletionResults")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PrepareLevelCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
