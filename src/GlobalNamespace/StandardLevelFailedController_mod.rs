#[cfg(feature = "StandardLevelFailedController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelFailedController_InitData {
    __cordl_parent: crate::System::Object,
    pub autoRestart: bool,
}
#[cfg(feature = "StandardLevelFailedController+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelFailedController_InitData => ""
    ."StandardLevelFailedController/InitData"
);
#[cfg(feature = "StandardLevelFailedController+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelFailedController_InitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelFailedController+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelFailedController_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelFailedController+InitData")]
impl crate::GlobalNamespace::StandardLevelFailedController_InitData {
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
#[cfg(feature = "StandardLevelFailedController+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelFailedController_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelFailedController")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelFailedController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _levelFailedTextEffect: *mut crate::GlobalNamespace::LevelFailedTextEffect,
    pub _standardLevelSceneSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    pub _prepareLevelCompletionResults: *mut crate::GlobalNamespace::PrepareLevelCompletionResults,
    pub _initData: *mut crate::GlobalNamespace::StandardLevelFailedController_InitData,
    pub _gameplayManager: *mut crate::GlobalNamespace::ILevelEndActions,
    pub _beatmapObjectSpawnController: *mut crate::GlobalNamespace::BeatmapObjectSpawnController,
    pub _gameSongController: *mut crate::GlobalNamespace::GameSongController,
    pub _environmentSpawnRotation: *mut crate::GlobalNamespace::EnvironmentSpawnRotation,
    pub _beatmapObjectManager: *mut crate::GlobalNamespace::BeatmapObjectManager,
}
#[cfg(feature = "StandardLevelFailedController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardLevelFailedController
    => ""."StandardLevelFailedController"
);
#[cfg(feature = "StandardLevelFailedController")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelFailedController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelFailedController")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelFailedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelFailedController")]
impl crate::GlobalNamespace::StandardLevelFailedController {
    #[cfg(feature = "StandardLevelFailedController+InitData")]
    pub type InitData = crate::GlobalNamespace::StandardLevelFailedController_InitData;
    #[cfg(feature = "StandardLevelFailedController+_LevelFailedCoroutine_d__13")]
    pub type _LevelFailedCoroutine_d__13 = crate::GlobalNamespace::StandardLevelFailedController__LevelFailedCoroutine_d__13;
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
#[cfg(feature = "StandardLevelFailedController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelFailedController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
