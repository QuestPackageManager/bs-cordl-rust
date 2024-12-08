#[cfg(feature = "AppInit")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInit {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _cameraGO: *mut crate::UnityEngine::GameObject,
    pub _asyncSceneContext: *mut crate::BGLib::AppFlow::Initialization::AsyncSceneContext,
    pub sceneSetupData: *mut crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData,
    pub _setupData: *mut AppInitSetupData,
    pub _gameScenesManager: *mut GameScenesManager,
}
#[cfg(feature = "AppInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AppInit => ""."AppInit"
);
#[cfg(feature = "AppInit")]
impl std::ops::Deref for AppInit {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInit")]
impl std::ops::DerefMut for AppInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInit")]
impl AppInit {
    #[cfg(feature = "AppInit+_Awake_d__10")]
    pub type _Awake_d__10 = crate::GlobalNamespace::AppInit__Awake_d__10;
    #[cfg(feature = "AppInit+AppStartType")]
    pub type AppStartType = crate::GlobalNamespace::AppInit_AppStartType;
    #[cfg(feature = "AppInit+_StartGameAsync_d__11")]
    pub type _StartGameAsync_d__11 = crate::GlobalNamespace::AppInit__StartGameAsync_d__11;
    #[cfg(feature = "AppInit+__c")]
    pub type __c = crate::GlobalNamespace::AppInit___c;
    #[cfg(feature = "AppInit+_InitializeAsync_d__13")]
    pub type _InitializeAsync_d__13 = crate::GlobalNamespace::AppInit__InitializeAsync_d__13;
    pub fn get_isTestContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isTestContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn _InitializeAsync_b__13_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<InitializeAsync>b__13_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InitializeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn RepeatableSetupAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RepeatableSetupAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAppStartType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::AppInit_AppStartType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AppInit_AppStartType = __cordl_object
            .invoke("GetAppStartType", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartGameAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("StartGameAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppStartAndMultiSceneEditorSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppStartAndMultiSceneEditorSetup", ())?;
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
    pub fn TransitionToNextScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToNextScene", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameScenesManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameScenesManager> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameScenesManager = __cordl_object
            .invoke("get_gameScenesManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn PreloadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("PreloadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeforeDismissingScenes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeforeDismissingScenes", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "AppInit")]
impl quest_hook::libil2cpp::ObjectType for AppInit {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AppInit+AppStartType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppInit_AppStartType {
    AppRestart = 1i32,
    AppStart = 0i32,
    MultiSceneEditor = 2i32,
}
#[cfg(feature = "AppInit+AppStartType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AppInit_AppStartType => ""
    ."AppInit/AppStartType"
);
