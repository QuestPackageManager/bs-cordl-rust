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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ActivatePresentedSceneRootObjects")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ActivatePresentedSceneRootObjects", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scenesToPresent))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ActivateScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ActivateScenes", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    bool,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("AppendScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AppendScenes", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        activateScenes,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                bool,
                1usize,
            >("AreAllScenesInStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AreAllScenesInStack", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (sceneNames)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::EventSystems::EventSystem,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BackupToListAndDisableCurrentEventSystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BackupToListAndDisableCurrentEventSystem", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ClearAndOpenScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearAndOpenScenes", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                        unloadAllScenes,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("DeactivateScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DeactivateScenes", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                0usize,
            >("GetCurrentlyLoadedSceneNames")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCurrentlyLoadedSceneNames", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                bool,
                1usize,
            >("IsAnySceneInStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAnySceneInStack", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (sceneNames)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneActiveOrLoading(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>),
                bool,
                1usize,
            >("IsSceneActiveOrLoading")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsSceneActiveOrLoading", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (sceneInfo)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsSceneInStack(
        &mut self,
        searchSceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsSceneInStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsSceneInStack", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (searchSceneName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadOneScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("LoadOneScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadOneScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (sceneName)) };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSingleScene(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LoadSingleScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadSingleScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Log")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Log", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkSceneAsPersistent(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("MarkSceneAsPersistent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MarkSceneAsPersistent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneName))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveGameObjectsFromContainerToSceneRoot(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("MoveGameObjectsFromContainerToSceneRoot")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MoveGameObjectsFromContainerToSceneRoot", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneName))
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("PopScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopScenes", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (minDuration, afterMinDurationCallback, finishCallback),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("PushScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PushScenes", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterExternallyLoadedScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        asyncOperationHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                        crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterExternallyLoadedScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterExternallyLoadedScene", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneName, asyncOperationHandle))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSceneFromStack(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveSceneFromStack")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveSceneFromStack", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneName))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("RemoveScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveScenes", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupDataSo,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReparentRootGameObjectsToDisabledGameObject(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ReparentRootGameObjectsToDisabledGameObject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReparentRootGameObjectsToDisabledGameObject", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneName))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::IEnumerator,
                            >,
                        >,
                    >,
                    f32,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("ReplaceScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReplaceScenes", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        scenesTransitionSetupData,
                        beforeNewScenesActivateRoutines,
                        minDuration,
                        afterMinDurationCallback,
                        finishCallback,
                    ),
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                1usize,
            >("SceneNamesFromSceneInfoArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SceneNamesFromSceneInfoArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, (sceneInfos)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    crate::GlobalNamespace::GameScenesManager_ScenePresentType,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    crate::GlobalNamespace::GameScenesManager_SceneDismissType,
                    f32,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::System::Action>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                10usize,
            >("ScenesTransitionCoroutine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScenesTransitionCoroutine", 10usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetActiveRootObjectsInScenes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetActiveRootObjectsInScenes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneNames, value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                bool,
                1usize,
            >("ShouldUnloadUnusedAssets")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShouldUnloadUnusedAssets", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (scenesToDismiss))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadOneScene(
        &mut self,
        sceneName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("UnloadOneScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnloadOneScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, (sceneName)) };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSingleScene(
        &mut self,
        sceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnloadSingleScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnloadSingleScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneInfo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitUntilTaskCompleted(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                1usize,
            >("WaitUntilTaskCompleted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WaitUntilTaskCompleted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked((), (task)) };
        Ok(__cordl_ret.into())
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
    pub fn _get_waitUntilSceneTransitionFinish_b__24_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                bool,
                0usize,
            >("<get_waitUntilSceneTransitionFinish>b__24_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<get_waitUntilSceneTransitionFinish>b__24_0", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_beforeDismissingScenesEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_beforeDismissingScenesEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_installEarlyBindingsEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_installEarlyBindingsEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_3<
                        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_transitionDidFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_transitionDidFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                        f32,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_transitionDidStartEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_transitionDidStartEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentSceneTransitionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                0usize,
            >("get_currentSceneTransitionType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_currentSceneTransitionType", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::GameScenesManager_SceneTransitionType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentScenesContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                0usize,
            >("get_currentScenesContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_currentScenesContainer", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isInTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isInTransition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_isInTransition", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::Dictionary_2<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                        >,
                    >,
                >,
                0usize,
            >("get_sceneNameToSceneOperationHandlesDictionary")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sceneNameToSceneOperationHandlesDictionary", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_waitUntilSceneTransitionFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil>,
                0usize,
            >("get_waitUntilSceneTransitionFinish")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_waitUntilSceneTransitionFinish", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::WaitUntil> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_beforeDismissingScenesEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_beforeDismissingScenesEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_installEarlyBindingsEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_installEarlyBindingsEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_3<
                        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
                        >,
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_transitionDidFinishEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_transitionDidFinishEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        crate::GlobalNamespace::GameScenesManager_SceneTransitionType,
                        f32,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_transitionDidStartEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_transitionDidStartEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
    const CLASS_NAME: &'static str = "GameScenesManager/SceneDismissType";
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
#[cfg(feature = "GameScenesManager+SceneDismissType")]
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
#[cfg(feature = "GameScenesManager+SceneDismissType")]
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
#[cfg(feature = "GameScenesManager+SceneDismissType")]
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
#[cfg(feature = "GameScenesManager+SceneDismissType")]
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
    const CLASS_NAME: &'static str = "GameScenesManager/ScenePresentType";
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
#[cfg(feature = "GameScenesManager+ScenePresentType")]
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
#[cfg(feature = "GameScenesManager+ScenePresentType")]
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
#[cfg(feature = "GameScenesManager+ScenePresentType")]
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
#[cfg(feature = "GameScenesManager+ScenePresentType")]
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
    const CLASS_NAME: &'static str = "GameScenesManager/SceneTransitionType";
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
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
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
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
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
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
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
#[cfg(feature = "GameScenesManager+SceneTransitionType")]
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
    const CLASS_NAME: &'static str = "GameScenesManager/ScenesStackData";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetDiContainer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDiContainer", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sceneNames))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                0usize,
            >("get_container")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_container", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                0usize,
            >("get_sceneNames")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_sceneNames", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_container(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_container")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_container", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_sceneNames")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_sceneNames", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
