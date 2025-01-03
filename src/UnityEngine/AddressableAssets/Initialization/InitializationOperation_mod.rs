#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct InitializationOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    >,
    pub m_rtdOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::AddressableAssets::Initialization::ResourceManagerRuntimeData,
    >,
    pub m_loadCatalogOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    >,
    pub m_ProviderSuffix: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Addressables: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    pub m_Diagnostics: *mut crate::UnityEngine::AddressableAssets::Utility::ResourceManagerDiagnostics,
    pub m_InitGroupOps: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::InitalizationObjectsOperation,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Initialization::InitializationOperation =>
    "UnityEngine.AddressableAssets.Initialization"."InitializationOperation"
);
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    pub fn CreateInitializationOperation(
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        playerSettingsLocation: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateInitializationOperation",
                (aa, playerSettingsLocation, providerSuffix),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalogInternal(
        &mut self,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
        index: i32,
        locMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = __cordl_object
            .invoke(
                "LoadContentCatalogInternal",
                (catalogs, index, locMap, remoteHashLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalog_AddressablesImpl_IResourceLocation_Il2CppString_IResourceLocation0(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadContentCatalog",
                (addressables, loc, providerSuffix, remoteHashLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalog_IResourceLocation_Il2CppString_IResourceLocation1(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = __cordl_object
            .invoke("LoadContentCatalog", (loc, providerSuffix, remoteHashLocation))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOpComplete(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
        locMap: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        >,
        index: i32,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadOpComplete",
                (op, catalogs, locMap, index, remoteHashLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadProvider(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        providerData: crate::UnityEngine::ResourceManagement::Util::ObjectInitializationData,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadProvider", (addressables, providerData, providerSuffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aa))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCatalogDataLoaded(
        addressables: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteHashLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnCatalogDataLoaded",
                (addressables, op, providerSuffix, remoteHashLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        aa: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aa))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DebugName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
