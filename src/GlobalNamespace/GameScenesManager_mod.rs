#[cfg(feature = "GameScenesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _emptyTransitionSceneInfo: *mut SceneInfo,
    pub _zenjectSceneLoader: *mut crate::Zenject::ZenjectSceneLoader,
    pub transitionDidStartEvent: *mut crate::System::Action_1<f32>,
    pub beforeDismissingScenesEvent: *mut crate::System::Action,
    pub transitionDidFinishEvent: *mut crate::System::Action_2<
        *mut ScenesTransitionSetupDataSO,
        *mut crate::Zenject::DiContainer,
    >,
    pub installEarlyBindingsEvent: *mut crate::System::Action_2<
        *mut ScenesTransitionSetupDataSO,
        *mut crate::Zenject::DiContainer,
    >,
    pub _inTransition: bool,
    pub _scenesStack: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::GameScenesManager_ScenesStackData,
    >,
    pub _neverUnloadScenes: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _sceneNameToSceneOperationHandlesDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    >,
}
#[cfg(feature = "GameScenesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameScenesManager => ""."GameScenesManager"
);
#[cfg(feature = "GameScenesManager")]
impl std::ops::Deref for GameScenesManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager")]
impl std::ops::DerefMut for GameScenesManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameScenesManager")]
impl GameScenesManager {
    pub const kLongTransitionLength: f32 = 1.3f32;
    pub const kRootContainerGOName: &'static str = "RootContainer";
    pub const kShortTransitionLength: f32 = 0.35f32;
    pub const kStandardTransitionLength: f32 = 0.7f32;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass43_0")]
    pub type __c__DisplayClass43_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass43_0;
    #[cfg(feature = "GameScenesManager+ScenePresentType")]
    pub type ScenePresentType = crate::GlobalNamespace::GameScenesManager_ScenePresentType;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass39_0")]
    pub type __c__DisplayClass39_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass39_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass36_0")]
    pub type __c__DisplayClass36_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass36_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass35_0;
    #[cfg(feature = "GameScenesManager+_ScenesTransitionCoroutine_d__42")]
    pub type _ScenesTransitionCoroutine_d__42 = crate::GlobalNamespace::GameScenesManager__ScenesTransitionCoroutine_d__42;
    #[cfg(feature = "GameScenesManager+_WaitUntilTaskCompleted_d__43")]
    pub type _WaitUntilTaskCompleted_d__43 = crate::GlobalNamespace::GameScenesManager__WaitUntilTaskCompleted_d__43;
    #[cfg(feature = "GameScenesManager+SceneDismissType")]
    pub type SceneDismissType = crate::GlobalNamespace::GameScenesManager_SceneDismissType;
    #[cfg(feature = "GameScenesManager+ScenesStackData")]
    pub type ScenesStackData = crate::GlobalNamespace::GameScenesManager_ScenesStackData;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass38_0")]
    pub type __c__DisplayClass38_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass38_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass37_0")]
    pub type __c__DisplayClass37_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass37_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass34_0")]
    pub type __c__DisplayClass34_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass34_0;
    pub fn AppendScenes(
        &mut self,
        scenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AppendScenes",
                (
                    scenesTransitionSetupData,
                    minDuration,
                    afterMinDurationCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AreAllScenesInStack(
        &mut self,
        sceneNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AreAllScenesInStack", (sceneNames))?;
        Ok(__cordl_ret)
    }
    pub fn ClearAndOpenScenes(
        &mut self,
        scenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret)
    }
    pub fn GetCurrentlyLoadedSceneNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetCurrentlyLoadedSceneNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsAnySceneInStack(
        &mut self,
        sceneNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAnySceneInStack", (sceneNames))?;
        Ok(__cordl_ret)
    }
    pub fn IsSceneInStack(
        &mut self,
        searchSceneName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSceneInStack", (searchSceneName))?;
        Ok(__cordl_ret)
    }
    pub fn MarkSceneAsPersistent(
        &mut self,
        sceneName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkSceneAsPersistent", (sceneName))?;
        Ok(__cordl_ret)
    }
    pub fn MoveGameObjectsFromContainerToSceneRoot(
        &mut self,
        sceneName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveGameObjectsFromContainerToSceneRoot", (sceneName))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PopScenes(
        &mut self,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "PopScenes",
                (minDuration, afterMinDurationCallback, finishCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PushScenes(
        &mut self,
        scenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret)
    }
    pub fn RegisterExternallyLoadedScene(
        &mut self,
        sceneName: *mut crate::System::String,
        asyncOperationHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterExternallyLoadedScene", (sceneName, asyncOperationHandle))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveScenes(
        &mut self,
        scenesTransitionSetupDataSo: *mut ScenesTransitionSetupDataSO,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret)
    }
    pub fn ReparentRootGameObjectsToDisabledGameObject(
        &mut self,
        sceneName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReparentRootGameObjectsToDisabledGameObject", (sceneName))?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceScenes(
        &mut self,
        scenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        beforeNewScenesActivateRoutines: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Collections::IEnumerator,
        >,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret)
    }
    pub fn SceneNamesFromSceneInfoArray(
        &mut self,
        sceneInfos: *mut quest_hook::libil2cpp::Il2CppArray<*mut SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("SceneNamesFromSceneInfoArray", (sceneInfos))?;
        Ok(__cordl_ret)
    }
    pub fn ScenesTransitionCoroutine(
        &mut self,
        newScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
        scenesToPresent: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        presentType: crate::GlobalNamespace::GameScenesManager_ScenePresentType,
        scenesToDismiss: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        dismissType: crate::GlobalNamespace::GameScenesManager_SceneDismissType,
        minDuration: f32,
        afterMinDurationCallback: *mut crate::System::Action,
        extraBindingsCallback: *mut crate::System::Action_1<
            *mut crate::Zenject::DiContainer,
        >,
        finishCallback: *mut crate::System::Action_1<*mut crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke(
                "ScenesTransitionCoroutine",
                (
                    newScenesTransitionSetupData,
                    scenesToPresent,
                    presentType,
                    scenesToDismiss,
                    dismissType,
                    minDuration,
                    afterMinDurationCallback,
                    extraBindingsCallback,
                    finishCallback,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetActiveRootObjectsInScenes(
        &mut self,
        sceneNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveRootObjectsInScenes", (sceneNames, value))?;
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
    pub fn _get_waitUntilSceneTransitionFinish_b__22_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<get_waitUntilSceneTransitionFinish>b__22_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_beforeDismissingScenesEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beforeDismissingScenesEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_installEarlyBindingsEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut ScenesTransitionSetupDataSO,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_installEarlyBindingsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_transitionDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut ScenesTransitionSetupDataSO,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_transitionDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_transitionDidStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_transitionDidStartEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentScenesContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("get_currentScenesContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInTransition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sceneNameToSceneOperationHandlesDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
            >,
        > = __cordl_object.invoke("get_sceneNameToSceneOperationHandlesDictionary", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_waitUntilSceneTransitionFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::WaitUntil> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::WaitUntil = __cordl_object
            .invoke("get_waitUntilSceneTransitionFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_beforeDismissingScenesEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beforeDismissingScenesEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_installEarlyBindingsEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut ScenesTransitionSetupDataSO,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_installEarlyBindingsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_transitionDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut ScenesTransitionSetupDataSO,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_transitionDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_transitionDidStartEvent(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_transitionDidStartEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameScenesManager")]
impl quest_hook::libil2cpp::ObjectType for GameScenesManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameScenesManager+SceneDismissType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScenesManager_SceneDismissType {
    Deactivate = 2i32,
    DoNotUnload = 0i32,
    Unload = 1i32,
}
#[cfg(feature = "GameScenesManager+SceneDismissType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameScenesManager_SceneDismissType => ""
    ."GameScenesManager/SceneDismissType"
);
#[cfg(feature = "GameScenesManager+ScenePresentType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScenesManager_ScenePresentType {
    Activate = 2i32,
    DoNotLoad = 0i32,
    Load = 1i32,
}
#[cfg(feature = "GameScenesManager+ScenePresentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameScenesManager_ScenePresentType => ""
    ."GameScenesManager/ScenePresentType"
);
#[cfg(feature = "GameScenesManager+ScenesStackData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager_ScenesStackData {
    __cordl_parent: crate::System::Object,
    pub _sceneNames_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _container_k__BackingField: *mut crate::Zenject::DiContainer,
}
#[cfg(feature = "GameScenesManager+ScenesStackData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameScenesManager_ScenesStackData => ""
    ."GameScenesManager/ScenesStackData"
);
#[cfg(feature = "GameScenesManager+ScenesStackData")]
impl std::ops::Deref for crate::GlobalNamespace::GameScenesManager_ScenesStackData {
    type Target = crate::System::Object;
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
        sceneNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sceneNames))?;
        Ok(__cordl_object)
    }
    pub fn SetDiContainer(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDiContainer", (container))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sceneNames: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sceneNames))?;
        Ok(__cordl_ret)
    }
    pub fn get_container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("get_container", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sceneNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_sceneNames", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_container(
        &mut self,
        value: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_container", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sceneNames(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sceneNames", (value))?;
        Ok(__cordl_ret)
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
