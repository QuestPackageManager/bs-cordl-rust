#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsManager {
    __cordl_parent: crate::System::Object,
    pub _accessToken: *mut crate::System::String,
    pub _platform: *mut crate::System::String,
    pub _initializationCancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _initializationTask: *mut crate::System::Threading::Tasks::Task_1<bool>,
    pub _updateCatalogTask: *mut crate::System::Threading::Tasks::Task,
    pub _appId: *mut crate::System::String,
    pub _platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
    pub _remoteCatalogLoader: *mut crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader,
    pub didCatalogLoadOrUpdateEvent: *mut crate::System::Action,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager
    => "BGLib.MetaRemoteAssets"."MetaRemoteAssetsManager"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
impl std::ops::DerefMut for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
impl crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager {
    pub const MetaServerHost: &'static str = "https://oculus.com";
    pub const kMetaServerCatalogPath: &'static str = "beat-saber/remote-assets/download/catalog.json";
    pub const kPlatformInjectId: &'static str = "MetaRemoteAssetsManager_platform_injectId";
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
    )]
    pub type AddResourceLocatorInput = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput;
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_FetchTokenAsync_d__22"
    )]
    pub type _FetchTokenAsync_d__22 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__FetchTokenAsync_d__22;
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_InitializeInternalAsync_d__20"
    )]
    pub type _InitializeInternalAsync_d__20 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__InitializeInternalAsync_d__20;
    #[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_Initialize_d__18")]
    pub type _Initialize_d__18 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__Initialize_d__18;
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_UpdateCatalogsAsync_d__23"
    )]
    pub type _UpdateCatalogsAsync_d__23 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__UpdateCatalogsAsync_d__23;
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_UpdateCatalogsInternalAsync_d__24"
    )]
    pub type _UpdateCatalogsInternalAsync_d__24 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__UpdateCatalogsInternalAsync_d__24;
    #[cfg(
        feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+_WaitInitAsync_d__19"
    )]
    pub type _WaitInitAsync_d__19 = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager__WaitInitAsync_d__19;
    #[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+__c")]
    pub type __c = crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager___c;
    pub fn ApplyAddressablesOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyAddressablesOverrides", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn FetchTokenAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("FetchTokenAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeInternalAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("InitializeInternalAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIdTransformFunc(
        &mut self,
        resourceLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("InternalIdTransformFunc", (resourceLocation))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        networkConfig: *mut crate::GlobalNamespace::INetworkConfig,
        platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
        remoteCatalogLoader: *mut crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader,
        platform: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (networkConfig, platformUserModel, remoteCatalogLoader, platform),
            )?;
        Ok(__cordl_object)
    }
    pub fn UpdateCatalogsAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdateCatalogsAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateCatalogsInternalAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("UpdateCatalogsInternalAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn WaitInitAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("WaitInitAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn WebRequestOverride(
        &mut self,
        request: *mut crate::UnityEngine::Networking::UnityWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WebRequestOverride", (request))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        networkConfig: *mut crate::GlobalNamespace::INetworkConfig,
        platformUserModel: *mut crate::GlobalNamespace::IPlatformUserModel,
        remoteCatalogLoader: *mut crate::BGLib::MetaRemoteAssets::IRemoteCatalogLoader,
        platform: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (networkConfig, platformUserModel, remoteCatalogLoader, platform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn add_didCatalogLoadOrUpdateEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCatalogLoadOrUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didCatalogLoadOrUpdateEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCatalogLoadOrUpdateEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MetaRemoteAssetsManager_AddResourceLocatorInput {
    __cordl_parent: crate::System::Object,
    pub ResourceLocator: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
    pub LocalHash: *mut crate::System::String,
    pub CatalogLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
}
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput =>
    "BGLib.MetaRemoteAssets"."MetaRemoteAssetsManager/AddResourceLocatorInput"
);
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
impl std::ops::Deref
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
impl crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput {
    pub fn New(
        resourceLocator: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        localHash: *mut crate::System::String,
        catalogLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourceLocator, localHash, catalogLocation))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        resourceLocator: *mut crate::UnityEngine::AddressableAssets::ResourceLocators::IResourceLocator,
        localHash: *mut crate::System::String,
        catalogLocation: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourceLocator, localHash, catalogLocation))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BGLib+MetaRemoteAssets+MetaRemoteAssetsManager+AddResourceLocatorInput"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::MetaRemoteAssetsManager_AddResourceLocatorInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
