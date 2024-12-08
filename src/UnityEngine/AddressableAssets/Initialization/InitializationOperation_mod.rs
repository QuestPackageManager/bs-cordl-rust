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
    pub m_ProviderSuffix: *mut crate::System::String,
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
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation+__c__DisplayClass16_0"
    )]
    pub type __c__DisplayClass16_0 = crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation___c__DisplayClass16_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation+__c__DisplayClass18_0"
    )]
    pub type __c__DisplayClass18_0 = crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation___c__DisplayClass18_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+Initialization+InitializationOperation+__c"
    )]
    pub type __c = crate::UnityEngine::AddressableAssets::Initialization::InitializationOperation___c;
    pub fn _ctor(
        &mut self,
        aa: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (aa))?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadContentCatalogInternal(
        &mut self,
        catalogs: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        index: i32,
        locMap: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        remoteHashLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
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
        Ok(__cordl_ret)
    }
    pub fn LoadContentCatalog(
        &mut self,
        loc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        providerSuffix: *mut crate::System::String,
        remoteHashLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
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
        Ok(__cordl_ret)
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadOpComplete(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        catalogs: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        locMap: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ResourceLocationMap,
        index: i32,
        remoteHashLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "LoadOpComplete",
                (op, catalogs, locMap, index, remoteHashLocation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DebugName", ())?;
        Ok(__cordl_ret)
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        aa: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (aa))?;
        Ok(__cordl_object)
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
