#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
#[repr(C)]
#[derive(Debug)]
pub struct Addressables {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Addressables {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets";
    const CLASS_NAME: &'static str = "Addressables";
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
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl std::ops::Deref for crate::UnityEngine::AddressableAssets::Addressables {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl std::ops::DerefMut for crate::UnityEngine::AddressableAssets::Addressables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl crate::UnityEngine::AddressableAssets::Addressables {
    pub const kAddressablesRuntimeBuildLogPath: &'static str = "AddressablesRuntimeBuildLog";
    pub const kAddressablesRuntimeDataPath: &'static str = "AddressablesRuntimeDataPath";
    pub const k_AddressablesLogConditional: &'static str = "ADDRESSABLES_LOG_ALL";
    #[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
    pub type MergeMode = crate::UnityEngine::AddressableAssets::Addressables_MergeMode;
    pub fn AddResourceLocator(
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
        localCatalogHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        remoteCatalogLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AddResourceLocator",
                (locator, localCatalogHash, remoteCatalogLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForCatalogUpdates(
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckForCatalogUpdates", (autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanBundleCache(
        catalogsIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CleanBundleCache", (catalogsIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IEnumerable3(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (keys))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IEnumerable__cordl_bool8(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (keys, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IList_1_1(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (locations))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IList_1_2(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (keys))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IList_1__cordl_bool6(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (locations, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_IList_1__cordl_bool7(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (keys, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_Il2CppObject0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_Il2CppObject__cordl_bool5(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (key, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_Il2CppString4(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDependencyCacheAsync_Il2CppString__cordl_bool9(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            bool,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearDependencyCacheAsync", (key, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearResourceLocators() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearResourceLocators", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCatalogLocationWithHashDependencies_IResourceLocation1<T>(
        remoteCatalogLocation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateCatalogLocationWithHashDependencies",
                (remoteCatalogLocation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCatalogLocationWithHashDependencies_Il2CppString0<T>(
        remoteCatalogPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCatalogLocationWithHashDependencies", (remoteCatalogPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCatalogLocationWithHashDependencies_Il2CppString_Il2CppString2<T>(
        remoteCatalogPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        remoteHashPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::ResourceLocationBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateCatalogLocationWithHashDependencies",
                (remoteCatalogPath, remoteHashPath),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadDependencies(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadDependencies", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadDependenciesAsync_IEnumerable_Addressables_MergeMode__cordl_bool3(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadDependenciesAsync", (keys, mode, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadDependenciesAsync_IList_1_Addressables_MergeMode__cordl_bool2(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadDependenciesAsync", (keys, mode, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadDependenciesAsync_IList_1__cordl_bool1(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadDependenciesAsync", (locations, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn DownloadDependenciesAsync_Il2CppObject__cordl_bool0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DownloadDependenciesAsync", (key, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadSize(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDownloadSize", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadSizeAsync_IEnumerable3(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDownloadSizeAsync", (keys))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadSizeAsync_IList_1_2(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDownloadSizeAsync", (keys))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadSizeAsync_Il2CppObject0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDownloadSizeAsync", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadSizeAsync_Il2CppString1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            i64,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDownloadSizeAsync", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocatorInfo_IResourceLocator1(
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocatorInfo", (locator))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocatorInfo_Il2CppString0(
        locatorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocatorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocatorInfo", (locatorId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsync_0() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAsync__cordl_bool1(
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeAsync", (autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_IResourceLocation_InstantiationParameters__cordl_bool5(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (location, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_IResourceLocation_Transform__cordl_bool__cordl_bool0(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateAsync",
                (location, parent, instantiateInWorldSpace, trackHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_IResourceLocation_Vector3_Quaternion_Transform__cordl_bool1(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateAsync",
                (location, position, rotation, parent, trackHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Il2CppObject_InstantiationParameters__cordl_bool4(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (key, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Il2CppObject_Transform__cordl_bool__cordl_bool2(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InstantiateAsync",
                (key, parent, instantiateInWorldSpace, trackHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateAsync_Il2CppObject_Vector3_Quaternion_Transform__cordl_bool3(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateAsync", (key, position, rotation, parent, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_IResourceLocation_InstantiationParameters__cordl_bool5(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (location, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_IResourceLocation_Transform__cordl_bool__cordl_bool0(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Instantiate",
                (location, parent, instantiateInWorldSpace, trackHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_IResourceLocation_Vector3_Quaternion_Transform__cordl_bool1(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (location, position, rotation, parent, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Il2CppObject_InstantiationParameters__cordl_bool4(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (key, instantiateParameters, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Il2CppObject_Transform__cordl_bool__cordl_bool2(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        instantiateInWorldSpace: bool,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (key, parent, instantiateInWorldSpace, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Instantiate_Il2CppObject_Vector3_Quaternion_Transform__cordl_bool3(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        trackHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Instantiate", (key, position, rotation, parent, trackHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSafeSerializationLog(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        logType: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSafeSerializationLog", (msg, logType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSafeSerializationLogFormat(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        logType: crate::UnityEngine::LogType,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalSafeSerializationLogFormat", (format, logType, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAsync_IResourceLocation0<TObject>(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
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
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetAsync", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetAsync_Il2CppObject1<TObject>(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetAsync", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_IResourceLocation0<TObject>(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
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
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAsset", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsset_Il2CppObject1<TObject>(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("LoadAsset", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IEnumerable_Addressables_MergeMode3<TObject>(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetsAsync", (keys, callback, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IEnumerable_Addressables_MergeMode__cordl_bool5<TObject>(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadAssetsAsync",
                (keys, callback, mode, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IList_1_0<TObject>(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetsAsync", (locations, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IList_1_Addressables_MergeMode2<TObject>(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetsAsync", (keys, callback, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IList_1_Addressables_MergeMode__cordl_bool4<TObject>(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadAssetsAsync",
                (keys, callback, mode, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_IList_1__cordl_bool1<TObject>(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadAssetsAsync",
                (locations, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_Il2CppObject6<TObject>(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetsAsync", (key, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssetsAsync_Il2CppObject__cordl_bool7<TObject>(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssetsAsync", (key, callback, releaseDependenciesOnFailure))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssets_IList_1_0<TObject>(
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssets", (locations, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssets_IList_1_Addressables_MergeMode1<TObject>(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssets", (keys, callback, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAssets_Il2CppObject2<TObject>(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TObject>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadAssets", (key, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalog(
        catalogPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadContentCatalog", (catalogPath, providerSuffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalogAsync_Il2CppString0(
        catalogPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadContentCatalogAsync", (catalogPath, providerSuffix))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentCatalogAsync__cordl_bool_Il2CppString1(
        catalogPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        autoReleaseHandle: bool,
        providerSuffix: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadContentCatalogAsync",
                (catalogPath, autoReleaseHandle, providerSuffix),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourceLocationsAsync_IEnumerable_Addressables_MergeMode_Type1(
        keys: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResourceLocationsAsync", (keys, mode, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourceLocationsAsync_IList_1_Addressables_MergeMode_Type0(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResourceLocationsAsync", (keys, mode, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourceLocationsAsync_Il2CppObject_Type2(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResourceLocationsAsync", (key, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourceLocations_IList_1_Addressables_MergeMode_Type0(
        keys: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        mode: crate::UnityEngine::AddressableAssets::Addressables_MergeMode,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResourceLocations", (keys, mode, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadResourceLocations_Il2CppObject_Type1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadResourceLocations", (key, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_IResourceLocation_LoadSceneMode2(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (location, loadMode, activateOnLoad, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_IResourceLocation_LoadSceneParameters3(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadSceneAsync",
                (location, loadSceneParameters, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppObject_LoadSceneMode0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadSceneAsync", (key, loadMode, activateOnLoad, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadSceneAsync_Il2CppObject_LoadSceneParameters1(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loadSceneParameters: crate::UnityEngine::SceneManagement::LoadSceneParameters,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LoadSceneAsync",
                (key, loadSceneParameters, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_IResourceLocation1(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (location, loadMode, activateOnLoad, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadScene_Il2CppObject0(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loadMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
        activateOnLoad: bool,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadScene", (key, loadMode, activateOnLoad, priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogError", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogErrorFormat(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogErrorFormat", (format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogException_AsyncOperationHandle_Exception0(
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogException", (op, ex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogException_Exception1(
        ex: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogException", (ex))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogFormat(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogFormat", (format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarning", (msg))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarningFormat(
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogWarningFormat", (format, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseInstance_AsyncOperationHandle1(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseInstance", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseInstance_AsyncOperationHandle_1_2(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseInstance", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseInstance_GameObject0(
        instance: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseInstance", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release_AsyncOperationHandle2(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release_AsyncOperationHandle_1_1<TObject>(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release_TObject0<TObject>(
        obj: TObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveResourceLocator(
        locator: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveResourceLocator", (locator))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveInternalId(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResolveInternalId", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_AsyncOperationHandle_1__cordl_bool4(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (handle, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_AsyncOperationHandle_UnloadSceneOptions__cordl_bool1(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (handle, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_AsyncOperationHandle__cordl_bool3(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (handle, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_SceneInstance_UnloadSceneOptions__cordl_bool0(
        scene: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        unloadOptions: crate::UnityEngine::SceneManagement::UnloadSceneOptions,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (scene, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadSceneAsync_SceneInstance__cordl_bool2(
        scene: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadSceneAsync", (scene, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadScene_AsyncOperationHandle_1_UnloadSceneOptions__cordl_bool3(
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
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadScene", (handle, unloadOptions, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadScene_AsyncOperationHandle_1__cordl_bool2(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadScene", (handle, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadScene_AsyncOperationHandle__cordl_bool1(
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadScene", (handle, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnloadScene_SceneInstance__cordl_bool0(
        scene: crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnloadScene", (scene, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCatalogs_IEnumerable_1__cordl_bool0(
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateCatalogs", (catalogs, autoReleaseHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCatalogs__cordl_bool_IEnumerable_1__cordl_bool1(
        autoCleanBundleCache: bool,
        catalogs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        autoReleaseHandle: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UpdateCatalogs",
                (autoCleanBundleCache, catalogs, autoReleaseHandle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BuildPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_BuildPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InitializationOperation() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InitializationOperation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstanceProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InstanceProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalIdTransformFunc() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InternalIdTransformFunc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PlayerBuildDataPath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PlayerBuildDataPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceLocators() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ResourceLocators", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ResourceManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RuntimePath() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_RuntimePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StreamingAssetsSubFolder() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_StreamingAssetsSubFolder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WebRequestOverride() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_WebRequestOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_m_Addressables() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AddressablesImpl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_m_Addressables", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InternalIdTransformFunc(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_InternalIdTransformFunc", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WebRequestOverride(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_WebRequestOverride", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Addressables {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Addressables_MergeMode {
    #[default]
    Intersection = 2i32,
    None = 0i32,
    Union = 1i32,
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AddressableAssets::Addressables_MergeMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets";
    const CLASS_NAME: &'static str = "MergeMode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::AddressableAssets::Addressables_MergeMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::AddressableAssets::Addressables_MergeMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::AddressableAssets::Addressables_MergeMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+AddressableAssets+Addressables+MergeMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::AddressableAssets::Addressables_MergeMode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
