#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut crate::UnityEngine::TextAsset,
    >,
    pub _mainPolyglotAsset: *mut crate::BGLib::Polyglot::Localization,
    pub _inputFiles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BGLib::Polyglot::LocalizationAsset,
    >,
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::LocalizationAsyncInstaller =>
    "BGLib.Polyglot"."LocalizationAsyncInstaller"
);
#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
impl std::ops::Deref for crate::BGLib::Polyglot::LocalizationAsyncInstaller {
    type Target = crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut crate::UnityEngine::TextAsset,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
impl std::ops::DerefMut for crate::BGLib::Polyglot::LocalizationAsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
impl crate::BGLib::Polyglot::LocalizationAsyncInstaller {
    pub const kLocalizationContentLabel: &'static str = "LocalizationContent";
    #[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller+__c")]
    pub type __c = crate::BGLib::Polyglot::LocalizationAsyncInstaller___c;
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
        assets: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::TextAsset,
        >,
        registry: *mut crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (assets, registry))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::Polyglot::LocalizationAsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
