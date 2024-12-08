#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorSchemesAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut ColorSchemeSO,
    >,
    pub _colorSchemes: *mut crate::System::Collections::Generic::List_1<
        *mut ColorSchemeSO,
    >,
}
#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Environments::Definitions::ColorSchemesAsyncInstaller => "Environments.Definitions"
    ."ColorSchemesAsyncInstaller"
);
#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
impl std::ops::Deref for crate::Environments::Definitions::ColorSchemesAsyncInstaller {
    type Target = crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut ColorSchemeSO,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
impl std::ops::DerefMut
for crate::Environments::Definitions::ColorSchemesAsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
impl crate::Environments::Definitions::ColorSchemesAsyncInstaller {
    #[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller+__c")]
    pub type __c = crate::Environments::Definitions::ColorSchemesAsyncInstaller___c;
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        colorSchemes: *mut crate::System::Collections::Generic::IList_1<
            *mut ColorSchemeSO,
        >,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (colorSchemes, registry))?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "Environments+Definitions+ColorSchemesAsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Environments::Definitions::ColorSchemesAsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
