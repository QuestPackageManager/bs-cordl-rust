#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesImpl {
    __cordl_parent: crate::System::Object,
    pub m_ResourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    pub m_InstanceProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
    pub m_CatalogRequestsTimeout: i32,
    pub SceneProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
    pub m_ResourceLocators: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
    >,
    pub m_InitializationOperation: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    >,
    pub m_ActiveCheckUpdateOperation: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    >,
    pub m_ActiveUpdateOperation: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    >,
    pub m_OnHandleCompleteAction: *mut crate::System::Action_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
    pub m_OnSceneHandleCompleteAction: *mut crate::System::Action_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
    pub m_OnHandleDestroyedAction: *mut crate::System::Action_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
    pub m_resultToHandle: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Object,
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
    pub m_SceneInstances: *mut crate::System::Collections::Generic::HashSet_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >,
    pub m_ActiveCleanBundleCacheOperation: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        bool,
    >,
    pub hasStartedInitialization: bool,
}
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AddressableAssets::AddressablesImpl
    => "UnityEngine.AddressableAssets"."AddressablesImpl"
);
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::AddressablesImpl {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::AddressablesImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
impl crate::UnityEngine::AddressableAssets::AddressablesImpl {
    pub const kCacheDataFolder: &'static str = "{UnityEngine.Application.persistentDataPath}/com.unity.addressables/";
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
    )]
    pub type LoadResourceLocationKeyOp = crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
    )]
    pub type LoadResourceLocationKeysOp = crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp;
    #[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c")]
    pub type __c = crate::UnityEngine::AddressableAssets::AddressablesImpl___c;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass100_0"
    )]
    pub type __c__DisplayClass100_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass100_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass103_0"
    )]
    pub type __c__DisplayClass103_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass103_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass106_0"
    )]
    pub type __c__DisplayClass106_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass106_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass110_0"
    )]
    pub type __c__DisplayClass110_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass110_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass112_0"
    )]
    pub type __c__DisplayClass112_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass112_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass117_0_1"
    )]
    pub type __c__DisplayClass117_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass117_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass119_0"
    )]
    pub type __c__DisplayClass119_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass119_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass120_0"
    )]
    pub type __c__DisplayClass120_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass120_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass121_0"
    )]
    pub type __c__DisplayClass121_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass121_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass126_0"
    )]
    pub type __c__DisplayClass126_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass126_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass128_0"
    )]
    pub type __c__DisplayClass128_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass128_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass131_0"
    )]
    pub type __c__DisplayClass131_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass131_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass132_0"
    )]
    pub type __c__DisplayClass132_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass132_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass141_0"
    )]
    pub type __c__DisplayClass141_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass141_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass142_0"
    )]
    pub type __c__DisplayClass142_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass142_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass146_0"
    )]
    pub type __c__DisplayClass146_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass146_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass150_0"
    )]
    pub type __c__DisplayClass150_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass150_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass155_0"
    )]
    pub type __c__DisplayClass155_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass155_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass156_0"
    )]
    pub type __c__DisplayClass156_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass156_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass61_0"
    )]
    pub type __c__DisplayClass61_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass61_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass72_0"
    )]
    pub type __c__DisplayClass72_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass72_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass78_0_1"
    )]
    pub type __c__DisplayClass78_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass78_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass79_0_1"
    )]
    pub type __c__DisplayClass79_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass79_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass83_0"
    )]
    pub type __c__DisplayClass83_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass83_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass85_0"
    )]
    pub type __c__DisplayClass85_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass85_0;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass88_0_1"
    )]
    pub type __c__DisplayClass88_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass88_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass89_0_1"
    )]
    pub type __c__DisplayClass89_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass89_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass91_0_1"
    )]
    pub type __c__DisplayClass91_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass91_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+AddressablesImpl+__c__DisplayClass99_0"
    )]
    pub type __c__DisplayClass99_0 = crate::UnityEngine::AddressableAssets::AddressablesImpl___c__DisplayClass99_0;
    pub fn AddResourceLocator(
        &mut self,
        loc: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        localCatalogHash: *mut crate::System::String,
        remoteCatalogLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddResourceLocator",
                (loc, localCatalogHash, remoteCatalogLocation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AutoReleaseHandleOnCompletion_AsyncOperationHandle0(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoReleaseHandleOnCompletion", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn AutoReleaseHandleOnCompletion_AsyncOperationHandle_1_1<TObject>(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoReleaseHandleOnCompletion", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn AutoReleaseHandleOnCompletion_AsyncOperationHandle_1__cordl_bool2<TObject>(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
        unloadSceneOpExcludeReleaseCallback: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AutoReleaseHandleOnCompletion",
                (handle, unloadSceneOpExcludeReleaseCallback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AutoReleaseHandleOnTypelessCompletion<TObject>(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AutoReleaseHandleOnTypelessCompletion", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForCatalogUpdates(
        &mut self,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
        > = __cordl_object.invoke("CheckForCatalogUpdates", (autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForCatalogUpdatesWithChain(
        &mut self,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
        > = __cordl_object
            .invoke("CheckForCatalogUpdatesWithChain", (autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn CleanBundleCacheWithChain_AsyncOperationHandle_1_0(
        &mut self,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
        forceSingleThreading: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("CleanBundleCacheWithChain", (depOp, forceSingleThreading))?;
        Ok(__cordl_ret)
    }
    pub fn CleanBundleCacheWithChain_IEnumerable_1_1(
        &mut self,
        catalogIds: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
        forceSingleThreading: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("CleanBundleCacheWithChain", (catalogIds, forceSingleThreading))?;
        Ok(__cordl_ret)
    }
    pub fn CleanBundleCache_AsyncOperationHandle_1_1(
        &mut self,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
        forceSingleThreading: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object.invoke("CleanBundleCache", (depOp, forceSingleThreading))?;
        Ok(__cordl_ret)
    }
    pub fn CleanBundleCache_IEnumerable_1_0(
        &mut self,
        catalogIds: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
        forceSingleThreading: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("CleanBundleCache", (catalogIds, forceSingleThreading))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDependencyCacheAsync_IEnumerable2(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("ClearDependencyCacheAsync", (keys, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDependencyCacheAsync_IList_1_1(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("ClearDependencyCacheAsync", (locations, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDependencyCacheAsync_Object0(
        &mut self,
        key: *mut crate::System::Object,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = __cordl_object
            .invoke("ClearDependencyCacheAsync", (key, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDependencyCacheForKey(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ClearDependencyCacheForKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn ClearResourceLocators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearResourceLocators", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearTrackHandles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearTrackHandles", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeCatalogSizeWithChain(
        &mut self,
        catalogLoc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("ComputeCatalogSizeWithChain", (catalogLoc))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCatalogLocationWithHashDependencies_IResourceLocation0<T>(
        &mut self,
        catalogLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase = __cordl_object
            .invoke("CreateCatalogLocationWithHashDependencies", (catalogLocation))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCatalogLocationWithHashDependencies_String1<T>(
        &mut self,
        catalogLocation: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase = __cordl_object
            .invoke("CreateCatalogLocationWithHashDependencies", (catalogLocation))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCatalogLocationWithHashDependencies_String_String2<T>(
        &mut self,
        catalogPath: *mut crate::System::String,
        hashFilePath: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase = __cordl_object
            .invoke(
                "CreateCatalogLocationWithHashDependencies",
                (catalogPath, hashFilePath),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnloadSceneWithChain_AsyncOperationHandle0(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "CreateUnloadSceneWithChain",
                (handle, unloadOptions, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateUnloadSceneWithChain_AsyncOperationHandle_1_1(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "CreateUnloadSceneWithChain",
                (handle, unloadOptions, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsyncWithChain_IEnumerable_Addressables_MergeMode__cordl_bool2(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        keys: *mut crate::System::Collections::IEnumerable,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke(
                "DownloadDependenciesAsyncWithChain",
                (dep, keys, mode, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsyncWithChain_IList_1__cordl_bool1(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke(
                "DownloadDependenciesAsyncWithChain",
                (dep, locations, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsyncWithChain_Object__cordl_bool0(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke(
                "DownloadDependenciesAsyncWithChain",
                (dep, key, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsync_IEnumerable_Addressables_MergeMode__cordl_bool2(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("DownloadDependenciesAsync", (keys, mode, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsync_IList_1__cordl_bool1(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("DownloadDependenciesAsync", (locations, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn DownloadDependenciesAsync_Object__cordl_bool0(
        &mut self,
        key: *mut crate::System::Object,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("DownloadDependenciesAsync", (key, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        x: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        y: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn EvaluateKey(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("EvaluateKey", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadSizeAsync_IEnumerable1(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("GetDownloadSizeAsync", (keys))?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadSizeAsync_Object0(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("GetDownloadSizeAsync", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadSizeWithChain_IEnumerable1(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        keys: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("GetDownloadSizeWithChain", (dep, keys))?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadSizeWithChain_Object0(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("GetDownloadSizeWithChain", (dep, key))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        loc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (loc))?;
        Ok(__cordl_ret)
    }
    pub fn GetLocatorInfo(
        &mut self,
        c: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AddressableAssets::ResourceLocatorInfo = __cordl_object
            .invoke("GetLocatorInfo", (c))?;
        Ok(__cordl_ret)
    }
    pub fn GetRemoteCatalogHeaderSize(
        &mut self,
        catalogLoc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("GetRemoteCatalogHeaderSize", (catalogLoc))?;
        Ok(__cordl_ret)
    }
    pub fn GetResourceLocations_IEnumerable_Addressables_MergeMode_ByRefMut1(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
        _cordl_type: *mut crate::System::Type,
        merge: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        locations: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetResourceLocations", (keys, _cordl_type, merge, locations))?;
        Ok(__cordl_ret)
    }
    pub fn GetResourceLocations_Object_ByRefMut0(
        &mut self,
        key: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
        locations: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetResourceLocations", (key, _cordl_type, locations))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync_1(
        &mut self,
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
        > = __cordl_object.invoke("InitializeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync_String_String__cordl_bool0(
        &mut self,
        runtimeDataPath: *mut crate::System::String,
        providerSuffix: *mut crate::System::String,
        autoReleaseHandle: bool,
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
                "InitializeAsync",
                (runtimeDataPath, providerSuffix, autoReleaseHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAsync__cordl_bool2(
        &mut self,
        autoReleaseHandle: bool,
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
        > = __cordl_object.invoke("InitializeAsync", (autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_IResourceLocation_InstantiationParameters__cordl_bool5(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke("InstantiateAsync", (location, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_IResourceLocation_Transform__cordl_bool__cordl_bool0(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        parent: *mut crate::UnityEngine::Transform,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke(
                "InstantiateAsync",
                (location, parent, instantiateInWorldSpace, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_IResourceLocation_Vector3_Quaternion_Transform__cordl_bool1(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: *mut crate::UnityEngine::Transform,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke(
                "InstantiateAsync",
                (location, position, rotation, parent, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_Object_InstantiationParameters__cordl_bool4(
        &mut self,
        key: *mut crate::System::Object,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke("InstantiateAsync", (key, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_Object_Transform__cordl_bool__cordl_bool2(
        &mut self,
        key: *mut crate::System::Object,
        parent: *mut crate::UnityEngine::Transform,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke(
                "InstantiateAsync",
                (key, parent, instantiateInWorldSpace, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAsync_Object_Vector3_Quaternion_Transform__cordl_bool3(
        &mut self,
        key: *mut crate::System::Object,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: *mut crate::UnityEngine::Transform,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke("InstantiateAsync", (key, position, rotation, parent, trackHandle))?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateWithChain_IResourceLocation1(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke(
                "InstantiateWithChain",
                (dep, location, instantiateParameters, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateWithChain_Object0(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object
            .invoke(
                "InstantiateWithChain",
                (dep, key, instantiateParameters, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InternalUnloadScene(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke("InternalUnloadScene", (handle, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn IsCatalogCached(
        &mut self,
        catalogLoc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        remoteHash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsCatalogCached", (catalogLoc, remoteHash))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetAsync_IResourceLocation0<TObject>(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAssetAsync", (location))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetAsync_Object1<TObject>(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAssetAsync", (key))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetWithChain_IResourceLocation0<TObject>(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        loc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAssetWithChain", (dep, loc))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetWithChain_Object1<TObject>(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("LoadAssetWithChain", (dep, key))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsAsync_IEnumerable_Addressables_MergeMode__cordl_bool1<TObject>(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
        callback: *mut crate::System::Action_1<TObject>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke(
                "LoadAssetsAsync",
                (keys, callback, mode, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsAsync_IList_1__cordl_bool0<TObject>(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        callback: *mut crate::System::Action_1<TObject>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke(
                "LoadAssetsAsync",
                (locations, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsAsync_Object__cordl_bool2<TObject>(
        &mut self,
        key: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<TObject>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke("LoadAssetsAsync", (key, callback, releaseDependenciesOnFailure))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsWithChain_IEnumerable_Addressables_MergeMode__cordl_bool1<TObject>(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        keys: *mut crate::System::Collections::IEnumerable,
        callback: *mut crate::System::Action_1<TObject>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke(
                "LoadAssetsWithChain",
                (dep, keys, callback, mode, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsWithChain_IList_1__cordl_bool0<TObject>(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        callback: *mut crate::System::Action_1<TObject>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke(
                "LoadAssetsWithChain",
                (dep, locations, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadAssetsWithChain_Object__cordl_bool2<TObject>(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<TObject>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<TObject>,
        > = __cordl_object
            .invoke(
                "LoadAssetsWithChain",
                (dep, key, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadContentCatalogAsync(
        &mut self,
        catalogPath: *mut crate::System::String,
        autoReleaseHandle: bool,
        providerSuffix: *mut crate::System::String,
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
                "LoadContentCatalogAsync",
                (catalogPath, autoReleaseHandle, providerSuffix),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourceLocationsAsync_IEnumerable_Addressables_MergeMode_Type0(
        &mut self,
        keys: *mut crate::System::Collections::IEnumerable,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        > = __cordl_object
            .invoke("LoadResourceLocationsAsync", (keys, mode, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourceLocationsAsync_Object_Type1(
        &mut self,
        key: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        > = __cordl_object.invoke("LoadResourceLocationsAsync", (key, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourceLocationsWithChain_IEnumerable_Addressables_MergeMode_Type0(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        keys: *mut crate::System::Collections::IEnumerable,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        > = __cordl_object
            .invoke("LoadResourceLocationsWithChain", (dep, keys, mode, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn LoadResourceLocationsWithChain_Object_Type1(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
        > = __cordl_object
            .invoke("LoadResourceLocationsWithChain", (dep, key, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsync_IResourceLocation2(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object.invoke("LoadSceneAsync", (location))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsync_IResourceLocation_LoadSceneMode__cordl_bool_i32__cordl_bool3(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (location, loadMode, activateOnLoad, priority, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsync_IResourceLocation_LoadSceneParameters__cordl_bool_i32__cordl_bool4(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (location, loadSceneParameters, activateOnLoad, priority, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsync_Object0(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object.invoke("LoadSceneAsync", (key))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneAsync_Object_LoadSceneParameters__cordl_bool_i32__cordl_bool1(
        &mut self,
        key: *mut crate::System::Object,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneAsync",
                (key, loadSceneParameters, activateOnLoad, priority, trackHandle),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneWithChain_IResourceLocation_LoadSceneMode1(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneWithChain",
                (dep, key, loadMode, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadSceneWithChain_Object_LoadSceneParameters0(
        &mut self,
        dep: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        key: *mut crate::System::Object,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke(
                "LoadSceneWithChain",
                (dep, key, loadSceneParameters, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Log(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Log", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn LogError(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogError", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn LogErrorFormat(
        &mut self,
        format: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogErrorFormat", (format, args))?;
        Ok(__cordl_ret)
    }
    pub fn LogException_AsyncOperationHandle_Exception0(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        ex: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (op, ex))?;
        Ok(__cordl_ret)
    }
    pub fn LogException_Exception1(
        &mut self,
        ex: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogException", (ex))?;
        Ok(__cordl_ret)
    }
    pub fn LogFormat(
        &mut self,
        format: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogFormat", (format, args))?;
        Ok(__cordl_ret)
    }
    pub fn LogWarning(
        &mut self,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn LogWarningFormat(
        &mut self,
        format: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarningFormat", (format, args))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        alloc: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alloc))?;
        Ok(__cordl_object)
    }
    pub fn OnHandleCompleted(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHandleCompleted", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn OnHandleDestroyed(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHandleDestroyed", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn OnSceneHandleCompleted(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSceneHandleCompleted", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn OnSceneUnloaded(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSceneUnloaded", (scene))?;
        Ok(__cordl_ret)
    }
    pub fn QueueEditorUpdateIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEditorUpdateIfNeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseInstance(
        &mut self,
        instance: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseInstance", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseSceneManagerOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseSceneManagerOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn Release_AsyncOperationHandle2(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn Release_AsyncOperationHandle_1_1<TObject>(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn Release_TObject0<TObject>(
        &mut self,
        obj: TObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveResourceLocator(
        &mut self,
        loc: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveResourceLocator", (loc))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveInternalId(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ResolveInternalId", (id))?;
        Ok(__cordl_ret)
    }
    pub fn TrackHandle_AsyncOperationHandle2(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("TrackHandle", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn TrackHandle_AsyncOperationHandle_1_0(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object.invoke("TrackHandle", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn TrackHandle_AsyncOperationHandle_1_1<TObject>(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("TrackHandle", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn UnloadSceneAsync_AsyncOperationHandle1(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke("UnloadSceneAsync", (handle, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn UnloadSceneAsync_AsyncOperationHandle_1_2(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke("UnloadSceneAsync", (handle, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn UnloadSceneAsync_SceneInstance0(
        &mut self,
        scene: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = __cordl_object
            .invoke("UnloadSceneAsync", (scene, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCatalogs(
        &mut self,
        catalogIds: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
        autoReleaseHandle: bool,
        autoCleanBundleCache: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = __cordl_object
            .invoke(
                "UpdateCatalogs",
                (catalogIds, autoReleaseHandle, autoCleanBundleCache),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _AutoReleaseHandleOnCompletion_b__115_0(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AutoReleaseHandleOnCompletion>b__115_0", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _AutoReleaseHandleOnCompletion_b__116_0<TObject>(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AutoReleaseHandleOnCompletion>b__116_0", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _AutoReleaseHandleOnTypelessCompletion_b__118_0<TObject>(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AutoReleaseHandleOnTypelessCompletion>b__118_0", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _GetRemoteCatalogHeaderSize_b__102_0(
        &mut self,
        getOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::Networking::UnityWebRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = __cordl_object.invoke("<GetRemoteCatalogHeaderSize>b__102_0", (getOp))?;
        Ok(__cordl_ret)
    }
    pub fn _TrackHandle_b__73_0(
        &mut self,
        sceneHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<TrackHandle>b__73_0", (sceneHandle))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        alloc: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alloc))?;
        Ok(__cordl_ret)
    }
    pub fn get_BuildPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BuildPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CatalogRequestsTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CatalogRequestsTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CatalogsWithAvailableUpdates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_CatalogsWithAvailableUpdates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChainOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("get_ChainOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InstanceProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider = __cordl_object
            .invoke("get_InstanceProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalIdTransformFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            *mut crate::System::String,
        > = __cordl_object.invoke("get_InternalIdTransformFunc", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PlayerBuildDataPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PlayerBuildDataPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResourceLocators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        > = __cordl_object.invoke("get_ResourceLocators", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResourceManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceManager = __cordl_object
            .invoke("get_ResourceManager", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RuntimePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_RuntimePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SceneOperationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SceneOperationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ShouldChainRequest(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ShouldChainRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StreamingAssetsSubFolder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_StreamingAssetsSubFolder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TrackedHandleCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TrackedHandleCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WebRequestOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_1<
            *mut crate::UnityEngine::Networking::UnityWebRequest,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Networking::UnityWebRequest,
        > = __cordl_object.invoke("get_WebRequestOverride", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CatalogRequestsTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CatalogRequestsTimeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InstanceProvider(
        &mut self,
        value: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InstanceProvider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_InternalIdTransformFunc(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalIdTransformFunc", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_WebRequestOverride(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::UnityEngine::Networking::UnityWebRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WebRequestOverride", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+AddressablesImpl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AddressablesImpl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesImpl_LoadResourceLocationKeyOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    >,
    pub m_Keys: *mut crate::System::Object,
    pub m_locations: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_Addressables: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    pub m_ResourceType: *mut crate::System::Type,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp =>
    "UnityEngine.AddressableAssets"."AddressablesImpl/LoadResourceLocationKeyOp"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
impl crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp {
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
    pub fn Init(
        &mut self,
        aa: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
        t: *mut crate::System::Type,
        keys: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (aa, t, keys))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
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
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeyOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeyOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesImpl_LoadResourceLocationKeysOp {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    >,
    pub m_Key: *mut crate::System::Collections::IEnumerable,
    pub m_MergeMode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
    pub m_locations: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub m_Addressables: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
    pub m_ResourceType: *mut crate::System::Type,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp =>
    "UnityEngine.AddressableAssets"."AddressablesImpl/LoadResourceLocationKeysOp"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
impl crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp {
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
    pub fn Init(
        &mut self,
        aa: *mut crate::UnityEngine::AddressableAssets::AddressablesImpl,
        t: *mut crate::System::Type,
        key: *mut crate::System::Collections::IEnumerable,
        mergeMode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (aa, t, key, mergeMode))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
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
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+AddressablesImpl+LoadResourceLocationKeysOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::AddressablesImpl_LoadResourceLocationKeysOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
