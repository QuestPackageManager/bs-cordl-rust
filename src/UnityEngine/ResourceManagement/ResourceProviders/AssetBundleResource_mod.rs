#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleResource {
    __cordl_parent: crate::System::Object,
    pub m_AssetBundle: *mut crate::UnityEngine::AssetBundle,
    pub m_RequestOperation: *mut crate::UnityEngine::AsyncOperation,
    pub m_WebRequestQueueOperation: *mut crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    pub m_ProvideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub m_Options: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleRequestOptions,
    pub m_RequestCompletedCallbackCalled: bool,
    pub m_Retries: i32,
    pub m_Source: crate::UnityEngine::ResourceManagement::Util::BundleSource,
    pub m_BytesToDownload: i64,
    pub m_DownloadedBytes: i64,
    pub m_Completed: bool,
    pub m_UnloadOperation: *mut crate::UnityEngine::AssetBundleUnloadOperation,
    pub m_TransformedInternalId: *mut crate::System::String,
    pub m_PreloadRequest: *mut crate::UnityEngine::AssetBundleRequest,
    pub m_PreloadCompleted: bool,
    pub m_LastDownloadedByteCount: u64,
    pub m_TimeoutTimer: f32,
    pub m_TimeoutOverFrames: i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource =>
    "UnityEngine.ResourceManagement.ResourceProviders"."AssetBundleResource"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    pub const k_WaitForWebRequestMainThreadSleep: i32 = 1i32;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
    )]
    pub type LoadType = crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType;
    pub fn AddBeginWebRequestHandler(
        &mut self,
        webRequestQueueOperation: *mut crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeginWebRequestHandler", (webRequestQueueOperation))?;
        Ok(__cordl_ret)
    }
    pub fn AddCallbackInvokeIfDone(
        &mut self,
        operation: *mut crate::UnityEngine::AsyncOperation,
        callback: *mut crate::System::Action_1<*mut crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallbackInvokeIfDone", (operation, callback))?;
        Ok(__cordl_ret)
    }
    pub fn BeginOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn BeginWebRequestOperation(
        &mut self,
        asyncOp: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginWebRequestOperation", (asyncOp))?;
        Ok(__cordl_ret)
    }
    pub fn CompleteBundleLoad(
        &mut self,
        bundle: *mut crate::UnityEngine::AssetBundle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteBundleLoad", (bundle))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWebRequest_IResourceLocation0(
        &mut self,
        loc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UnityWebRequest = __cordl_object
            .invoke("CreateWebRequest", (loc))?;
        Ok(__cordl_ret)
    }
    pub fn CreateWebRequest_String1(
        &mut self,
        url: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::UnityWebRequest = __cordl_object
            .invoke("CreateWebRequest", (url))?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueWebRequest(
        &mut self,
        internalId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::WebRequestQueueOperation = __cordl_object
            .invoke("EnqueueWebRequest", (internalId))?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AssetBundle> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AssetBundle = __cordl_object
            .invoke("GetAssetBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetPreloadRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AssetBundleRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AssetBundleRequest = __cordl_object
            .invoke("GetAssetPreloadRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = __cordl_object
            .invoke("GetDownloadStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadLocalBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLocalBundle", ())?;
        Ok(__cordl_ret)
    }
    pub fn LocalRequestOperationCompleted(
        &mut self,
        op: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalRequestOperationCompleted", (op))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnUnloadOperationComplete(
        &mut self,
        op: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnloadOperationComplete", (op))?;
        Ok(__cordl_ret)
    }
    pub fn PercentComplete(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("PercentComplete", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
        unloadOp: *mut crate::UnityEngine::AssetBundleUnloadOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (provideHandle, unloadOp))?;
        Ok(__cordl_ret)
    }
    pub fn Unload(
        &mut self,
        unloadOp: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::AssetBundleUnloadOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unload", (unloadOp))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        unscaledDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (unscaledDeltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
        Ok(__cordl_ret)
    }
    pub fn WebRequestOperationCompleted(
        &mut self,
        op: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WebRequestOperationCompleted", (op))?;
        Ok(__cordl_ret)
    }
    pub fn _AddBeginWebRequestHandler_b__39_0(
        &mut self,
        asyncOp: *mut crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AddBeginWebRequestHandler>b__39_0", (asyncOp))?;
        Ok(__cordl_ret)
    }
    pub fn _GetAssetPreloadRequest_b__26_0(
        &mut self,
        operation: *mut crate::UnityEngine::AsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<GetAssetPreloadRequest>b__26_0", (operation))?;
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
    pub fn get_BytesToDownload(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_BytesToDownload", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasTimedOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasTimedOut", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetBundleResource_LoadType {
    Local = 1i32,
    None = 0i32,
    Web = 2i32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType =>
    "UnityEngine.ResourceManagement.ResourceProviders"."AssetBundleResource/LoadType"
);
