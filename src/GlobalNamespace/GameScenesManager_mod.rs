#[cfg(feature = "GameScenesManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _emptyTransitionSceneInfo: *mut crate::GlobalNamespace::SceneInfo,
    pub _zenjectSceneLoader: *mut crate::Zenject::ZenjectSceneLoader,
    pub transitionDidStartEvent: *mut crate::System::Action_2<
        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
        f32,
    >,
    pub beforeDismissingScenesEvent: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    >,
    pub transitionDidFinishEvent: *mut crate::System::Action_3<
        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
        *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        *mut crate::Zenject::DiContainer,
    >,
    pub installEarlyBindingsEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        *mut crate::Zenject::DiContainer,
    >,
    pub _currentSceneTransitionType: crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    pub _scenesStack: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::GameScenesManager_ScenesStackData,
    >,
    pub _neverUnloadScenes: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _sceneNameToSceneOperationHandlesDictionary: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    >,
}
#[cfg(feature = "GameScenesManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameScenesManager => ""
    ."GameScenesManager"
);
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
    #[cfg(feature = "GameScenesManager+_LoadOneScene_d__50")]
    pub type _LoadOneScene_d__50 = crate::GlobalNamespace::GameScenesManager__LoadOneScene_d__50;
    #[cfg(feature = "GameScenesManager+_ScenesTransitionCoroutine_d__52")]
    pub type _ScenesTransitionCoroutine_d__52 = crate::GlobalNamespace::GameScenesManager__ScenesTransitionCoroutine_d__52;
    #[cfg(feature = "GameScenesManager+_UnloadOneScene_d__51")]
    pub type _UnloadOneScene_d__51 = crate::GlobalNamespace::GameScenesManager__UnloadOneScene_d__51;
    #[cfg(feature = "GameScenesManager+_WaitUntilTaskCompleted_d__55")]
    pub type _WaitUntilTaskCompleted_d__55 = crate::GlobalNamespace::GameScenesManager__WaitUntilTaskCompleted_d__55;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass40_0")]
    pub type __c__DisplayClass40_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass40_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass41_0")]
    pub type __c__DisplayClass41_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass41_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass42_0")]
    pub type __c__DisplayClass42_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass42_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass43_0")]
    pub type __c__DisplayClass43_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass43_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass44_0")]
    pub type __c__DisplayClass44_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass44_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass45_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass46_0")]
    pub type __c__DisplayClass46_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass46_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass47_0")]
    pub type __c__DisplayClass47_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass47_0;
    #[cfg(feature = "GameScenesManager+__c__DisplayClass55_0")]
    pub type __c__DisplayClass55_0 = crate::GlobalNamespace::GameScenesManager___c__DisplayClass55_0;
    pub fn ActivatePresentedSceneRootObjects(
        scenesToPresent: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::EventSystems::EventSystem,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetCurrentlyLoadedSceneNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAnySceneInStack(
        &mut self,
        sceneNames: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
                *mut crate::System::Collections::IEnumerator,
            >,
        >,
        minDuration: f32,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
            quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::SceneInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        presentType: crate::GlobalNamespace::GameScenesManager_ScenePresentType,
        scenesToDismiss: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        dismissType: crate::GlobalNamespace::GameScenesManager_SceneDismissType,
        minDuration: f32,
        canTriggerGarbageCollector: bool,
        afterMinDurationCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        extraBindingsCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
        >,
        finishCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Zenject::DiContainer>,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut crate::System::Collections::Generic::List_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                *mut crate::Zenject::DiContainer,
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
                *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                *mut crate::Zenject::DiContainer,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut crate::System::Collections::Generic::List_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                *mut crate::Zenject::DiContainer,
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
                *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                *mut crate::Zenject::DiContainer,
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
    Activate = 3i32,
    DoNotLoad = 0i32,
    Load = 1i32,
    LoadAndDoNotActivate = 2i32,
}
#[cfg(feature = "GameScenesManager+ScenePresentType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameScenesManager_ScenePresentType => ""
    ."GameScenesManager/ScenePresentType"
);
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScenesManager_SceneTransitionType {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameScenesManager_SceneTransitionType => ""
    ."GameScenesManager/SceneTransitionType"
);
#[cfg(feature = "GameScenesManager+ScenesStackData")]
#[repr(C)]
#[derive(Debug)]
pub struct GameScenesManager_ScenesStackData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sceneNames_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
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
                *mut quest_hook::libil2cpp::Il2CppString,
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
