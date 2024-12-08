#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectReferenceAsyncInstaller_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AsyncInstaller,
    pub _operationHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        T,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1 < T > =>
    "BGLib.AppFlow.Initialization"."ScriptableObjectReferenceAsyncInstaller`1" < T >
);
#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1<T> {
    type Target = crate::BGLib::AppFlow::Initialization::AsyncInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1<T> {
    #[cfg(
        feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1+_LoadResourcesBeforeInstallAsync_d__4"
    )]
    pub type _LoadResourcesBeforeInstallAsync_d__4 = crate::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1__LoadResourcesBeforeInstallAsync_d__4<
        T,
    >;
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourcesBeforeInstallAsync(
        &mut self,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        _: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("LoadResourcesBeforeInstallAsync", (registry, _))?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        _: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (registry, _))?;
        Ok(__cordl_ret)
    }
    pub fn get_assetRuntimeKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_assetRuntimeKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
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
#[cfg(
    feature = "BGLib+AppFlow+Initialization+ScriptableObjectReferenceAsyncInstaller_1"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::ScriptableObjectReferenceAsyncInstaller_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
