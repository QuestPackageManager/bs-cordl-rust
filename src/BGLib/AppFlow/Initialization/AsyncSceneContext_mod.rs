#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncSceneContext {
    __cordl_parent: crate::Zenject::SceneContext,
    pub _asyncPreloaders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::AppFlow::Initialization::AsyncPreloader,
        >,
    >,
    pub _asyncInstallers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller,
        >,
    >,
    pub _state: crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State,
    pub _registry: quest_hook::libil2cpp::Gc<
        crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
    >,
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
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncSceneContext+State")]
    pub type State = crate::BGLib::AppFlow::Initialization::AsyncSceneContext_State;
    pub fn CreateContainerForLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("CreateContainerForLoading", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateRegistry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
        > = __cordl_object.invoke("CreateRegistry", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallInstallers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallInstallers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadInstallersAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::BGLib::AppFlow::Initialization::AsyncInstallerRegistry,
            >,
        > = __cordl_object.invoke("LoadInstallersAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RunAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __n__0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<>n__0", ())?;
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
