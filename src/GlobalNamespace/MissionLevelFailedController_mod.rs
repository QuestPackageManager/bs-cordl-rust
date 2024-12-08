#[cfg(feature = "MissionLevelFailedController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelFailedController_InitData {
    __cordl_parent: crate::System::Object,
    pub autoRestart: bool,
}
#[cfg(feature = "MissionLevelFailedController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelFailedController_InitData => ""
    ."MissionLevelFailedController/InitData"
);
#[cfg(feature = "MissionLevelFailedController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelFailedController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelFailedController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MissionLevelFailedController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelFailedController+InitData")]
impl crate::GlobalNamespace::MissionLevelFailedController_InitData {
    pub fn New(autoRestart: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (autoRestart))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        autoRestart: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (autoRestart))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionLevelFailedController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelFailedController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MissionLevelFailedController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelFailedController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _prepareLevelCompletionResults: *mut PrepareLevelCompletionResults,
    pub _levelFailedTextEffect: *mut LevelFailedTextEffect,
    pub _missionLevelSceneSetupData: *mut MissionLevelScenesTransitionSetupDataSO,
    pub _missionObjectiveCheckersManager: *mut MissionObjectiveCheckersManager,
    pub _initData: *mut crate::GlobalNamespace::MissionLevelFailedController_InitData,
    pub _beatmapObjectSpawnController: *mut BeatmapObjectSpawnController,
    pub _gameSongController: *mut GameSongController,
    pub _gameplayManager: *mut ILevelEndActions,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
}
#[cfg(feature = "MissionLevelFailedController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionLevelFailedController => ""
    ."MissionLevelFailedController"
);
#[cfg(feature = "MissionLevelFailedController")]
impl std::ops::Deref for MissionLevelFailedController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelFailedController")]
impl std::ops::DerefMut for MissionLevelFailedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelFailedController")]
impl MissionLevelFailedController {
    #[cfg(feature = "MissionLevelFailedController+_LevelFailedCoroutine_d__13")]
    pub type _LevelFailedCoroutine_d__13 = crate::GlobalNamespace::MissionLevelFailedController__LevelFailedCoroutine_d__13;
    #[cfg(feature = "MissionLevelFailedController+InitData")]
    pub type InitData = crate::GlobalNamespace::MissionLevelFailedController_InitData;
    pub fn HandleLevelFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn LevelFailedCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("LevelFailedCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "MissionLevelFailedController")]
impl quest_hook::libil2cpp::ObjectType for MissionLevelFailedController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
