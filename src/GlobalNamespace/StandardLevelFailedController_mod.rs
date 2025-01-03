#[cfg(feature = "StandardLevelFailedController")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelFailedController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _levelFailedTextEffect: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelFailedTextEffect,
    >,
    pub _standardLevelSceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
    >,
    pub _prepareLevelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PrepareLevelCompletionResults,
    >,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::StandardLevelFailedController_InitData,
    >,
    pub _gameplayManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILevelEndActions,
    >,
    pub _beatmapObjectSpawnController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectSpawnController,
    >,
    pub _gameSongController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameSongController,
    >,
    pub _environmentSpawnRotation: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentSpawnRotation,
    >,
    pub _beatmapObjectManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectManager,
    >,
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
    pub fn HandleLevelFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLevelFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LevelFailedCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("LevelFailedCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "StandardLevelFailedController+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelFailedController_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        autoRestart: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (autoRestart))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
