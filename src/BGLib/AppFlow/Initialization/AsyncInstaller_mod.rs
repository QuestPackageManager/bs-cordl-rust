#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstaller {
    __cordl_parent: crate::Zenject::MonoInstaller,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AppFlow::Initialization::AsyncInstaller {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "AsyncInstaller";
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
impl std::ops::Deref for crate::BGLib::AppFlow::Initialization::AsyncInstaller {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
impl std::ops::DerefMut for crate::BGLib::AppFlow::Initialization::AsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
impl crate::BGLib::AppFlow::Initialization::AsyncInstaller {
    #[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
    type IInstallerRegistry = crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry;
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        registry: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (registry, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourcesBeforeInstallAsync(
        &mut self,
        registry: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("LoadResourcesBeforeInstallAsync", (registry, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::AsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstaller_IInstallerRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.AppFlow.Initialization";
    const CLASS_NAME: &'static str = "AsyncInstaller/IInstallerRegistry";
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
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
impl std::ops::Deref
for crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
impl std::ops::DerefMut
for crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
impl crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
    pub fn AddMonoInstaller(
        &mut self,
        newMonoInstaller: quest_hook::libil2cpp::Gc<crate::Zenject::MonoInstaller>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMonoInstaller", (newMonoInstaller))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddScriptableObjectInstaller(
        &mut self,
        newScriptableObjectInstaller: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScriptableObjectInstaller,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScriptableObjectInstaller", (newScriptableObjectInstaller))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BGLib+AppFlow+Initialization+AsyncInstaller+IInstallerRegistry")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
