#[cfg(feature = "PackDefinitionAsyncInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct PackDefinitionAsyncInstaller {
    __cordl_parent: crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut crate::GlobalNamespace::PackDefinitionSO,
    >,
    pub _packDefinitions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::PackDefinitionSO,
    >,
}
#[cfg(feature = "PackDefinitionAsyncInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackDefinitionAsyncInstaller =>
    ""."PackDefinitionAsyncInstaller"
);
#[cfg(feature = "PackDefinitionAsyncInstaller")]
impl std::ops::Deref for crate::GlobalNamespace::PackDefinitionAsyncInstaller {
    type Target = crate::BGLib::AppFlow::Initialization::AddressablesAsyncInstaller_1<
        *mut crate::GlobalNamespace::PackDefinitionSO,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionAsyncInstaller")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackDefinitionAsyncInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackDefinitionAsyncInstaller")]
impl crate::GlobalNamespace::PackDefinitionAsyncInstaller {
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
    pub fn LoadPackDefinitionListAsync() -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                *mut crate::System::Collections::Generic::IList_1<
                    *mut crate::GlobalNamespace::PackDefinitionSO,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                *mut crate::System::Collections::Generic::IList_1<
                    *mut crate::GlobalNamespace::PackDefinitionSO,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadPackDefinitionListAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourcesBeforeInstall(
        &mut self,
        assets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
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
#[cfg(feature = "PackDefinitionAsyncInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackDefinitionAsyncInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
