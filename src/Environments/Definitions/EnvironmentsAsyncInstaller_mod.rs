#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentsAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut EnvironmentInfoSO,
    >,
    pub _environmentInfos: *mut crate::System::Collections::Generic::List_1<
        *mut EnvironmentInfoSO,
    >,
}
#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Environments::Definitions::EnvironmentsAsyncInstaller => "Environments.Definitions"
    ."EnvironmentsAsyncInstaller"
);
#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
impl std::ops::Deref for crate::Environments::Definitions::EnvironmentsAsyncInstaller {
    type Target = crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut EnvironmentInfoSO,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
impl std::ops::DerefMut
for crate::Environments::Definitions::EnvironmentsAsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
impl crate::Environments::Definitions::EnvironmentsAsyncInstaller {
    #[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller+__c")]
    pub type __c = crate::Environments::Definitions::EnvironmentsAsyncInstaller___c;
    pub fn get_assetLabelRuntimeKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_assetLabelRuntimeKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        environmentInfos: *mut crate::System::Collections::Generic::IList_1<
            *mut EnvironmentInfoSO,
        >,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (environmentInfos, registry))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Environments+Definitions+EnvironmentsAsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Environments::Definitions::EnvironmentsAsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
