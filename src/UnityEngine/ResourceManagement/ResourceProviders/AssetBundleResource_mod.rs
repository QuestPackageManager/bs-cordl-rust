#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleResource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AssetBundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    pub m_RequestOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AsyncOperation,
    >,
    pub m_WebRequestQueueOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
    >,
    pub m_ProvideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
    pub m_Options: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleRequestOptions,
    >,
    pub m_RequestCompletedCallbackCalled: bool,
    pub m_Retries: i32,
    pub m_Source: crate::UnityEngine::ResourceManagement::Util::BundleSource,
    pub m_BytesToDownload: i64,
    pub m_DownloadedBytes: i64,
    pub m_Completed: bool,
    pub m_UnloadOperation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleUnloadOperation,
    >,
    pub m_TransformedInternalId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_PreloadRequest: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AssetBundleRequest,
    >,
    pub m_PreloadCompleted: bool,
    pub m_LastDownloadedByteCount: u64,
    pub m_TimeoutTimer: f32,
    pub m_TimeoutOverFrames: i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "AssetBundleResource";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        webRequestQueueOperation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeginWebRequestHandler", (webRequestQueueOperation))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCallbackInvokeIfDone(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCallbackInvokeIfDone", (operation, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginOperation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginWebRequestOperation(
        &mut self,
        asyncOp: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginWebRequestOperation", (asyncOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteBundleLoad(
        &mut self,
        bundle: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteBundleLoad", (bundle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWebRequest_IResourceLocation0(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = __cordl_object.invoke("CreateWebRequest", (loc))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateWebRequest_Il2CppString1(
        &mut self,
        url: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        > = __cordl_object.invoke("CreateWebRequest", (url))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueWebRequest(
        &mut self,
        internalId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::WebRequestQueueOperation,
        > = __cordl_object.invoke("EnqueueWebRequest", (internalId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle> = __cordl_object
            .invoke("GetAssetBundle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetPreloadRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleRequest,
        > = __cordl_object.invoke("GetAssetPreloadRequest", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetLoadInfo_IResourceLocation_ResourceManager_ByRefMut1(
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        resourceManager: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        loadType: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType,
        >,
        path: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoadInfo", (location, resourceManager, loadType, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoadInfo_ProvideHandle_ByRefMut0(
        handle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
        loadType: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType,
        >,
        path: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoadInfo", (handle, loadType, path))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadLocalBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadLocalBundle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalRequestOperationCompleted(
        &mut self,
        op: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LocalRequestOperationCompleted", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnUnloadOperationComplete(
        &mut self,
        op: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUnloadOperationComplete", (op))?;
        Ok(__cordl_ret.into())
    }
    pub fn PercentComplete(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("PercentComplete", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        provideHandle: crate::UnityEngine::ResourceManagement::ResourceProviders::ProvideHandle,
        unloadOp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleUnloadOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", (provideHandle, unloadOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unload(
        &mut self,
        unloadOp: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundleUnloadOperation>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Unload", (unloadOp))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
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
    pub fn _AddBeginWebRequestHandler_b__39_0(
        &mut self,
        asyncOp: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<AddBeginWebRequestHandler>b__39_0", (asyncOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn _GetAssetPreloadRequest_b__26_0(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<GetAssetPreloadRequest>b__26_0", (operation))?;
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
    pub fn get_BytesToDownload(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_BytesToDownload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasTimedOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasTimedOut", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl AsRef<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn as_ref(&self) -> &crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl AsMut<crate::UnityEngine::ResourceManagement::IUpdateReceiver>
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::IUpdateReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl AsRef<
    crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource,
> for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource")]
impl AsMut<
    crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource,
> for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AssetBundleResource_LoadType {
    #[default]
    Local = 1i32,
    None = 0i32,
    Web = 2i32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "LoadType";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType {
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType {
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceProviders+AssetBundleResource+LoadType"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::ResourceProviders::AssetBundleResource_LoadType {
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
