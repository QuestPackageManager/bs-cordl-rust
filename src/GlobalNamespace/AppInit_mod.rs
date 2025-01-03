#[cfg(feature = "AppInit")]
#[repr(C)]
#[derive(Debug)]
pub struct AppInit {
    __cordl_parent: crate::Zenject::MonoInstaller,
    pub _cameraGO: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _asyncSceneContext: quest_hook::libil2cpp::Gc<
        crate::BGLib::AppFlow::Initialization::AsyncSceneContext,
    >,
    pub sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AppInitScenesTransitionSetupDataSO_AppInitSceneSetupData,
    >,
    pub _setupData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AppInitSetupData>,
    pub _gameScenesManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameScenesManager,
    >,
}
#[cfg(feature = "AppInit")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AppInit => ""."AppInit"
);
#[cfg(feature = "AppInit")]
impl std::ops::Deref for crate::GlobalNamespace::AppInit {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AppInit")]
impl std::ops::DerefMut for crate::GlobalNamespace::AppInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AppInit")]
impl crate::GlobalNamespace::AppInit {
    #[cfg(feature = "AppInit+AppStartType")]
    pub type AppStartType = crate::GlobalNamespace::AppInit_AppStartType;
    pub fn AppStartAndMultiSceneEditorSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppStartAndMultiSceneEditorSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppStartType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::AppInit_AppStartType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::AppInit_AppStartType = __cordl_object
            .invoke("GetAppStartType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleBeforeDismissingScenes(
        &mut self,
        scenes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBeforeDismissingScenes", (scenes))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("InitializeAsync", ())?;
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
    pub fn PreloadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("PreloadAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepeatableSetupAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RepeatableSetupAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartGameAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("StartGameAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TransitionToNextScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TransitionToNextScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _InitializeAsync_b__13_0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<InitializeAsync>b__13_0", ())?;
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
    pub fn get_gameScenesManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameScenesManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameScenesManager,
        > = __cordl_object.invoke("get_gameScenesManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isTestContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isTestContext", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AppInit")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AppInit {
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
