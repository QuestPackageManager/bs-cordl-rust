#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ContentCatalogProvider {
    __cordl_parent: crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase,
    pub DisableCatalogUpdateOnStart: bool,
    pub IsLocalCatalogInBundle: bool,
    pub m_LocationToCatalogLoadOpMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            *mut crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp,
        >,
    >,
    pub m_RM: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceManager,
    >,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider =>
    "UnityEngine.AddressableAssets.ResourceProviders"."ContentCatalogProvider"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider {
    type Target = crate::UnityEngine::ResourceManagement::ResourceProviders::ResourceProviderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
impl crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider {
    #[cfg(
        feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+DependencyHashIndex"
    )]
    pub type DependencyHashIndex = crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_DependencyHashIndex;
    #[cfg(
        feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
    )]
    pub type InternalOp = crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp;
    pub fn New(
        resourceManagerInstance: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (resourceManagerInstance))?;
        Ok(__cordl_object.into())
    }
    pub fn Provide(
        &mut self,
        providerInterface: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Provide", (providerInterface))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (location, obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        resourceManagerInstance: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (resourceManagerInstance))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+DependencyHashIndex"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentCatalogProvider_DependencyHashIndex {
    Cache = 1i32,
    Count = 2i32,
    Remote = 0i32,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+DependencyHashIndex"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_DependencyHashIndex
    => "UnityEngine.AddressableAssets.ResourceProviders"
    ."ContentCatalogProvider/DependencyHashIndex"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ContentCatalogProvider_InternalOp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_LocalDataPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_RemoteHashValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_LocalHashValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_ProviderInterface: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub m_ContentCatalogData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
    >,
    pub m_ContentCatalogDataLoadOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
    >,
    pub m_BundledCatalog: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog,
    >,
    pub m_Retried: bool,
    pub m_DisableCatalogUpdateOnStart: bool,
    pub m_IsLocalCatalogInBundle: bool,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp
    => "UnityEngine.AddressableAssets.ResourceProviders"
    ."ContentCatalogProvider/InternalOp"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
impl crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp {
    pub const kCatalogExt: &'static str = ".json";
    #[cfg(
        feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
    )]
    pub type BundledCatalog = crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog;
    pub fn CanLoadCatalogFromBundle(
        &mut self,
        idToLoad: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CanLoadCatalogFromBundle", (idToLoad, location))?;
        Ok(__cordl_ret.into())
    }
    pub fn CatalogLoadOpCompleteCallback(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CatalogLoadOpCompleteCallback", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetermineIdToLoad(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        dependencyObjects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
        disableCatalogUpdateOnStart: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke(
                "DetermineIdToLoad",
                (location, dependencyObjects, disableCatalogUpdateOnStart),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformedInternalId(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetTransformedInternalId", (loc))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadCatalog(
        &mut self,
        idToLoad: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        loadCatalogFromLocalBundle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadCatalog", (idToLoad, loadCatalogFromLocalBundle))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCatalogLoaded(
        &mut self,
        ccd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCatalogLoaded", (ccd))?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        providerInterface: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
        disableCatalogUpdateOnStart: bool,
        isLocalCatalogInBundle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Start",
                (providerInterface, disableCatalogUpdateOnStart, isLocalCatalogInBundle),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletionCallback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _LoadCatalog_b__14_0(
        &mut self,
        ccd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<LoadCatalog>b__14_0", (ccd))?;
        Ok(__cordl_ret.into())
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
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceProviders::ContentCatalogProvider_InternalOp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
#[repr(C)]
#[derive(Debug)]
pub struct InternalOp_ContentCatalogProvider_BundledCatalog {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_BundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_OpInProgress: bool,
    pub m_LoadBundleRequest: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleCreateRequest,
    >,
    pub m_CatalogAssetBundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    pub m_LoadTextAssetRequest: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_CatalogData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
    >,
    pub m_WebRequestQueueOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    >,
    pub m_RequestOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AsyncOperation,
    >,
    pub m_WebRequestTimeout: i32,
    pub OnLoaded: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog
    => "UnityEngine.AddressableAssets.ResourceProviders"
    ."ContentCatalogProvider/InternalOp/BundledCatalog"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
impl crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadCatalogFromBundleAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadCatalogFromBundleAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadTextAssetRequestComplete(
        &mut self,
        op: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadTextAssetRequestComplete", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        webRequestTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bundlePath, webRequestTimeout))?;
        Ok(__cordl_object.into())
    }
    pub fn Unload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WebRequestOperationCompleted(
        &mut self,
        op: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WebRequestOperationCompleted", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn _LoadCatalogFromBundleAsync_b__19_0(
        &mut self,
        loadOp: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<LoadCatalogFromBundleAsync>b__19_0", (loadOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _LoadCatalogFromBundleAsync_b__19_1(
        &mut self,
        asyncOp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<LoadCatalogFromBundleAsync>b__19_1", (asyncOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bundlePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        webRequestTimeout: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bundlePath, webRequestTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_OnLoaded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_OnLoaded", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OpInProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OpInProgress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OpIsSuccess(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OpIsSuccess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_OnLoaded(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::AddressableAssets::ResourceLocators::ContentCatalogData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_OnLoaded", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+ResourceProviders+ContentCatalogProvider+InternalOp+BundledCatalog"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::ResourceProviders::InternalOp_ContentCatalogProvider_BundledCatalog {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
