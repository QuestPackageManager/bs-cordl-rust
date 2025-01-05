#[cfg(feature = "BGLib+Polyglot+LocalizationAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalizationAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
    >,
    pub _mainPolyglotAsset: quest_hook::libil2cpp::Gc<
        crate::BGLib::Polyglot::Localization,
    >,
    pub _inputFiles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationAsset>,
        >,
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
        quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
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
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLocalizationAssetsSync() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationAsset>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationAsset>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadLocalizationAssetsSync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        assets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
            >,
        >,
        registry: quest_hook::libil2cpp::Gc<
            crate::BGLib::AppFlow::Initialization::AsyncInstaller_IInstallerRegistry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadResourcesBeforeInstall", (assets, registry))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalizationContentToAsset(
        content: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::TextAsset>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationAsset>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::BGLib::Polyglot::LocalizationAsset>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalizationContentToAsset", (content))?;
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
    pub fn get_assetLabelRuntimeKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_assetLabelRuntimeKey", ())?;
        Ok(__cordl_ret.into())
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
