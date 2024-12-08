#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncSceneContext {
    __cordl_parent: crate::Zenject::SceneContext,
    pub _asyncPreloaders: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BGLib::AppFlow::Initialization::AsyncPreloader,
    >,
    pub _asyncInstallers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller,
    >,
    pub _state: crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State,
    pub _registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::AsyncSceneContext => "BGLib.AppFlow.Initialization"
    ."AsyncSceneContext"
);
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    type Target = crate::Zenject::SceneContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl std::ops::DerefMut for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    #[cfg(
        feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+_LoadInstallersAsync_d__9"
    )]
    pub type _LoadInstallersAsync_d__9 = crate::BGLib::AppFlow::Initialization::AsyncSceneContext__LoadInstallersAsync_d__9;
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+_RunAsync_d__6")]
    pub type _RunAsync_d__6 = crate::BGLib::AppFlow::Initialization::AsyncSceneContext__RunAsync_d__6;
    #[cfg(
        feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+__c__DisplayClass9_0"
    )]
    pub type __c__DisplayClass9_0 = crate::BGLib::AppFlow::Initialization::AsyncSceneContext___c__DisplayClass9_0;
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+__c")]
    pub type __c = crate::BGLib::AppFlow::Initialization::AsyncSceneContext___c;
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
    pub type State = crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State;
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+_Run_d__5")]
    pub type _Run_d__5 = crate::BGLib::AppFlow::Initialization::AsyncSceneContext__Run_d__5;
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
    pub fn RunAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RunAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateContainerForLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateContainerForLoading", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadInstallersAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        > = __cordl_object.invoke("LoadInstallersAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn __n__0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<>n__0", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry = __cordl_object
            .invoke("CreateRegistry", ())?;
        Ok(__cordl_ret)
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallInstallers", ())?;
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::AsyncSceneContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncSceneContext_State {
    Initialized = 2i32,
    Initializing = 1i32,
    NotInitialized = 0i32,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::AsyncSceneContext_State =>
    "BGLib.AppFlow.Initialization"."AsyncSceneContext/State"
);
