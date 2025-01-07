#[cfg(feature = "GameScenesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _emptyTransitionSceneInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SceneInfo,
    >,
    pub _zenjectSceneLoader: quest_hook::libil2cpp::Gc<
        crate::Zenject::ZenjectSceneLoader,
    >,
    pub transitionDidStartEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
            f32,
        >,
    >,
    pub beforeDismissingScenesEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    >,
    pub transitionDidFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        >,
    >,
    pub installEarlyBindingsEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::ScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        >,
    >,
    pub _currentSceneTransitionType: crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    pub _scenesStack: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::GameScenesManager_ScenesStackData,
            >,
        >,
    >,
    pub _neverUnloadScenes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _sceneNameToSceneOperationHandlesDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
            >,
        >,
    >,
}
#[cfg(feature = "GameScenesManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::GameScenesManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GameScenesManager";
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
#[cfg(feature = "GameScenesManager")]
impl std::ops::Deref for crate::GlobalNamespace::GameScenesManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameScenesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager")]
impl crate::GlobalNamespace::GameScenesManager {
    pub const kLongTransitionLength: f32 = 1.3f32;
    pub const kRootContainerGOName: &'static str = "RootContainer";
    pub const kShortTransitionLength: f32 = 0.35f32;
    pub const kStandardTransitionLength: f32 = 0.7f32;
    #[cfg(feature = "GameScenesManager+SceneDismissType")]
    pub type SceneDismissType = crate::GlobalNamespace::GameScenesManager_SceneDismissType;
    #[cfg(feature = "GameScenesManager+ScenePresentType")]
    pub type ScenePresentType = crate::GlobalNamespace::GameScenesManager_ScenePresentType;
    #[cfg(feature = "GameScenesManager+SceneTransitionType")]
    pub type SceneTransitionType = crate::GlobalNamespace::GameScenesManager_SceneTransitionType;
    #[cfg(feature = "GameScenesManager+ScenesStackData")]
    pub type ScenesStackData = crate::GlobalNamespace::GameScenesManager_ScenesStackData;
    pub fn ActivatePresentedSceneRootObjects(
        scenesToPresent: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ActivatePresentedSceneRootObjects", (scenesToPresent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ActivateScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ActivateScenes",
                (
                    scenesTransitionSetupData,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        activateScenes: bool,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AppendScenes",
                (
                    scenesTransitionSetupData,
                    activateScenes,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AreAllScenesInStack(
        &mut self,
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AreAllScenesInStack", (sceneNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn BackupToListAndDisableCurrentEventSystem(
        &mut self,
        list: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::EventSystem,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackupToListAndDisableCurrentEventSystem", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAndOpenScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        unloadAllScenes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ClearAndOpenScenes",
                (
                    scenesTransitionSetupData,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                    unloadAllScenes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeactivateScenes",
                (
                    scenesTransitionSetupData,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentlyLoadedSceneNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("GetCurrentlyLoadedSceneNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAnySceneInStack(
        &mut self,
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAnySceneInStack", (sceneNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneActiveOrLoading(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSceneActiveOrLoading", (sceneInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneInStack(
        &mut self,
        searchSceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSceneInStack", (searchSceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOneScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("LoadOneScene", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSingleScene(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadSingleScene", (sceneInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkSceneAsPersistent(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkSceneAsPersistent", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectsFromContainerToSceneRoot(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveGameObjectsFromContainerToSceneRoot", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PopScenes(
        &mut self,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopScenes",
                (minDuration, afterMinDurationCallback, finishCallback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PushScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PushScenes",
                (
                    scenesTransitionSetupData,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterExternallyLoadedScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        asyncOperationHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterExternallyLoadedScene", (sceneName, asyncOperationHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSceneFromStack(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSceneFromStack", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveScenes(
        &mut self,
        scenesTransitionSetupDataSo: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RemoveScenes",
                (
                    scenesTransitionSetupDataSo,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReparentRootGameObjectsToDisabledGameObject(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReparentRootGameObjectsToDisabledGameObject", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceScenes(
        &mut self,
        scenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        beforeNewScenesActivateRoutines: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
            >,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceScenes",
                (
                    scenesTransitionSetupData,
                    beforeNewScenesActivateRoutines,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SceneNamesFromSceneInfoArray(
        &mut self,
        sceneInfos: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("SceneNamesFromSceneInfoArray", (sceneInfos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScenesTransitionCoroutine(
        &mut self,
        newScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
        scenesToPresent: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        presentType: crate::GlobalNamespace::GameScenesManager_ScenePresentType,
        scenesToDismiss: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        dismissType: crate::GlobalNamespace::GameScenesManager_SceneDismissType,
        minDuration: f32,
        canTriggerGarbageCollector: bool,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        extraBindingsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object
            .invoke(
                "ScenesTransitionCoroutine",
                (
                    newScenesTransitionSetupData,
                    scenesToPresent,
                    presentType,
                    scenesToDismiss,
                    dismissType,
                    minDuration,
                    canTriggerGarbageCollector,
                    afterMinDurationCallback,
                    extraBindingsCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveRootObjectsInScenes(
        &mut self,
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveRootObjectsInScenes", (sceneNames, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldUnloadUnusedAssets(
        &mut self,
        scenesToDismiss: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldUnloadUnusedAssets", (scenesToDismiss))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadOneScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("UnloadOneScene", (sceneName))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSingleScene(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnloadSingleScene", (sceneInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilTaskCompleted(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WaitUntilTaskCompleted", (task))?;
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
    pub fn _get_waitUntilSceneTransitionFinish_b__24_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<get_waitUntilSceneTransitionFinish>b__24_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_beforeDismissingScenesEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beforeDismissingScenesEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_installEarlyBindingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_installEarlyBindingsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_transitionDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_transitionDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_transitionDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_transitionDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentSceneTransitionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GameScenesManager_SceneTransitionType = __cordl_object
            .invoke("get_currentSceneTransitionType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentScenesContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_currentScenesContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInTransition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneNameToSceneOperationHandlesDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                >,
            >,
        > = __cordl_object.invoke("get_sceneNameToSceneOperationHandlesDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_waitUntilSceneTransitionFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil> = __cordl_object
            .invoke("get_waitUntilSceneTransitionFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beforeDismissingScenesEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beforeDismissingScenesEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_installEarlyBindingsEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_installEarlyBindingsEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_transitionDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_transitionDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_transitionDidStartEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_transitionDidStartEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameScenesManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameScenesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameScenesManager+SceneDismissType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameScenesManager_SceneDismissType {
    #[default]
    Deactivate = 2i32,
    DoNotUnload = 0i32,
    Unload = 1i32,
}
#[cfg(feature = "GameScenesManager+SceneDismissType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameScenesManager_SceneDismissType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SceneDismissType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameScenesManager_SceneDismissType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameScenesManager_SceneDismissType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameScenesManager_SceneDismissType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameScenesManager_SceneDismissType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "GameScenesManager+ScenePresentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameScenesManager_ScenePresentType {
    #[default]
    Activate = 3i32,
    DoNotLoad = 0i32,
    Load = 1i32,
    LoadAndDoNotActivate = 2i32,
}
#[cfg(feature = "GameScenesManager+ScenePresentType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameScenesManager_ScenePresentType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScenePresentType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameScenesManager_ScenePresentType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameScenesManager_ScenePresentType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameScenesManager_ScenePresentType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameScenesManager_ScenePresentType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameScenesManager_SceneTransitionType {
    #[default]
    Activate = 6i32,
    Append = 5i32,
    ClearAndOpen = 4i32,
    Deactivate = 8i32,
    None = 0i32,
    Pop = 2i32,
    Push = 1i32,
    Remove = 7i32,
    Replace = 3i32,
}
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameScenesManager_SceneTransitionType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SceneTransitionType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GameScenesManager_SceneTransitionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GameScenesManager_SceneTransitionType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GameScenesManager_SceneTransitionType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GameScenesManager_SceneTransitionType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager_ScenesStackData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sceneNames_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _container_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Zenject::DiContainer,
    >,
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ScenesStackData";
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
#[cfg(feature = "GameScenesManager+ScenesStackData")]
impl std::ops::Deref for crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
impl crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    pub fn New(
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneNames))?;
        Ok(__cordl_object.into())
    }
    pub fn SetDiContainer(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDiContainer", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sceneNames))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_container", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sceneNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("get_sceneNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_container(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_container", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sceneNames(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sceneNames", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
