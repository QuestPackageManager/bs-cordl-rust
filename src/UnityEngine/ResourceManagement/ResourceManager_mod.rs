#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_CompletedOperation_1<TObject: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        TObject,
    >,
    pub m_Success: bool,
    pub m_Exception: *mut crate::System::Exception,
    pub m_ReleaseDependenciesOnFailure: bool,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1 < TObject >
    => "UnityEngine.ResourceManagement"."ResourceManager/CompletedOperation`1" < TObject
    >
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<
    TObject,
> {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        TObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<
    TObject,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<TObject> {
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init_String0(
        &mut self,
        result: TObject,
        success: bool,
        errorMsg: *mut crate::System::String,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (result, success, errorMsg, releaseDependenciesOnFailure))?;
        Ok(__cordl_ret)
    }
    pub fn Init_Exception1(
        &mut self,
        result: TObject,
        success: bool,
        exception: *mut crate::System::Exception,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (result, success, exception, releaseDependenciesOnFailure))?;
        Ok(__cordl_ret)
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DebugName", ())?;
        Ok(__cordl_ret)
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<
    TObject,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ResourceManager_DeferredCallbackRegisterRequest {
    pub operation: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    pub incrementRefCount: bool,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest =>
    "UnityEngine.ResourceManagement"."ResourceManager/DeferredCallbackRegisterRequest"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
impl crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ResourceManager_DiagnosticEventContext {
    pub _OperationHandle_k__BackingField: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    pub _Type_k__BackingField: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
    pub _EventValue_k__BackingField: i32,
    pub _Location_k__BackingField: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    pub _Context_k__BackingField: *mut crate::System::Object,
    pub _Error_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext =>
    "UnityEngine.ResourceManagement"."ResourceManager/DiagnosticEventContext"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
impl crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
    pub fn get_OperationHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OperationHandle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
    > {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Type",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_EventValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EventValue",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    > {
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Location",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Error",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Context",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        _cordl_type: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
        eventValue: i32,
        error: *mut crate::System::String,
        context: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (op, _cordl_type, eventValue, error, context),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceManager_DiagnosticEventType {
    AsyncOperationComplete = 3i32,
    AsyncOperationCreate = 1i32,
    AsyncOperationDestroy = 5i32,
    AsyncOperationFail = 0i32,
    AsyncOperationPercentComplete = 2i32,
    AsyncOperationReferenceCount = 4i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType =>
    "UnityEngine.ResourceManagement"."ResourceManager/DiagnosticEventType"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_InstanceOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub m_dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub m_instantiationParams: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
    pub m_instanceProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
    pub m_instance: *mut crate::UnityEngine::GameObject,
    pub m_scene: crate::UnityEngine::SceneManagement::Scene,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation =>
    "UnityEngine.ResourceManagement"."ResourceManager/InstanceOperation"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        *mut crate::UnityEngine::GameObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
impl crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    pub fn GetDependencies(
        &mut self,
        deps: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (deps))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        rm: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
        instanceProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        instantiationParams: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::UnityEngine::GameObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (rm, instanceProvider, instantiationParams, dependency))?;
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
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDownloadStatus(
        &mut self,
        visited: *mut crate::System::Collections::Generic::HashSet_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = __cordl_object
            .invoke("GetDownloadStatus", (visited))?;
        Ok(__cordl_ret)
    }
    pub fn InstanceScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = __cordl_object
            .invoke("InstanceScene", ())?;
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
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager {
    __cordl_parent: crate::System::Object,
    pub postProfilerEvents: bool,
    pub _InternalIdTransformFunc_k__BackingField: *mut crate::System::Func_2<
        *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        *mut crate::System::String,
    >,
    pub _WebRequestOverride_k__BackingField: *mut crate::System::Action_1<
        *mut crate::UnityEngine::Networking::UnityWebRequest,
    >,
    pub CallbackHooksEnabled: bool,
    pub m_ResourceProviders: *mut ListWithEvents_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
    >,
    pub m_allocator: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    pub m_UpdateReceivers: *mut ListWithEvents_1<
        *mut crate::UnityEngine::ResourceManagement::IUpdateReceiver,
    >,
    pub m_UpdateReceiversToRemove: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ResourceManagement::IUpdateReceiver,
    >,
    pub m_UpdatingReceivers: bool,
    pub m_InsideUpdateMethod: bool,
    pub m_providerMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
    >,
    pub m_AssetOperationCache: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub m_TrackedInstanceOperations: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation,
    >,
    pub m_UpdateCallbacks: *mut DelegateList_1<f32>,
    pub m_DeferredCompleteCallbacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub m_AssetBundleProviders: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
    >,
    pub m_InsideExecuteDeferredCallbacksMethod: bool,
    pub m_DeferredCallbacksToRegister: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest,
    >,
    pub m_obsoleteDiagnosticsHandler: *mut crate::System::Action_4<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
        i32,
        *mut crate::System::Object,
    >,
    pub m_diagnosticsHandler: *mut crate::System::Action_1<
        crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
    >,
    pub m_ReleaseOpNonCached: *mut crate::System::Action_1<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub m_ReleaseOpCached: *mut crate::System::Action_1<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub m_ReleaseInstanceOp: *mut crate::System::Action_1<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub _CertificateHandlerInstance_k__BackingField: *mut crate::UnityEngine::Networking::CertificateHandler,
    pub m_RegisteredForCallbacks: bool,
    pub m_ProviderOperationTypeCache: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::Type,
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ResourceManagement::ResourceManager
    => "UnityEngine.ResourceManagement"."ResourceManager"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::ResourceManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl std::ops::DerefMut for crate::UnityEngine::ResourceManagement::ResourceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl crate::UnityEngine::ResourceManagement::ResourceManager {
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1"
    )]
    pub type CompletedOperation_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<
        TObject,
    >;
    #[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
    pub type InstanceOperation = crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType"
    )]
    pub type DiagnosticEventType = crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+__c__DisplayClass95_0_1"
    )]
    pub type __c__DisplayClass95_0_1<TObject: quest_hook::libil2cpp::Type> = crate::UnityEngine::ResourceManagement::ResourceManager___c__DisplayClass95_0_1<
        TObject,
    >;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext"
    )]
    pub type DiagnosticEventContext = crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
    )]
    pub type DeferredCallbackRegisterRequest = crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest;
    pub fn GetDefaultTypeForLocation(
        &mut self,
        loc: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetDefaultTypeForLocation", (loc))?;
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
    pub fn OnObjectAdded(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnObjectAdded", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ProvideScene_LoadSceneMode0(
        &mut self,
        sceneProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        loadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
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
                "ProvideScene",
                (sceneProvider, location, loadSceneMode, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProvideScene_LoadSceneParameters1(
        &mut self,
        sceneProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
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
                "ProvideScene",
                (sceneProvider, location, loadSceneParameters, activateOnLoad, priority),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseScene(
        &mut self,
        sceneProvider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        sceneLoadHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
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
        > = __cordl_object.invoke("ReleaseScene", (sceneProvider, sceneLoadHandle))?;
        Ok(__cordl_ret)
    }
    pub fn set_Allocator(
        &mut self,
        value: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Allocator", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnOperationDestroyNonCached(
        &mut self,
        o: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOperationDestroyNonCached", (o))?;
        Ok(__cordl_ret)
    }
    pub fn get_OperationCacheCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OperationCacheCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterDiagnosticCallback(
        &mut self,
        func: *mut crate::System::Action_1<
            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDiagnosticCallback", (func))?;
        Ok(__cordl_ret)
    }
    pub fn TransformInternalId(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("TransformInternalId", (location))?;
        Ok(__cordl_ret)
    }
    pub fn get_Allocator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy = __cordl_object
            .invoke("get_Allocator", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProvideResourceGroupCached(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        groupHash: i32,
        desiredType: *mut crate::System::Type,
        callback: *mut crate::System::Action_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        > = __cordl_object
            .invoke(
                "ProvideResourceGroupCached",
                (
                    locations,
                    groupHash,
                    desiredType,
                    callback,
                    releaseDependenciesOnFailure,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PostDiagnosticEvent(
        &mut self,
        context: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostDiagnosticEvent", (context))?;
        Ok(__cordl_ret)
    }
    pub fn AddOperationToCache(
        &mut self,
        key: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        operation: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOperationToCache", (key, operation))?;
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
    pub fn IsOperationCached(
        &mut self,
        key: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsOperationCached", (key))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDiagnosticCallback_Action_4_0(
        &mut self,
        func: *mut crate::System::Action_4<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
            i32,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDiagnosticCallback", (func))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDiagnosticCallback_Action_1_1(
        &mut self,
        func: *mut crate::System::Action_1<
            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDiagnosticCallback", (func))?;
        Ok(__cordl_ret)
    }
    pub fn CreateChainOperation_AsyncOperationHandle_1_0<TObject, TObjectDependency>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObjectDependency,
        >,
        callback: *mut crate::System::Func_2<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObjectDependency,
            >,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TObjectDependency: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("CreateChainOperation", (dependentOp, callback))?;
        Ok(__cordl_ret)
    }
    pub fn CreateChainOperation_AsyncOperationHandle1<TObject>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        callback: *mut crate::System::Func_2<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObject,
            >,
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
        > = __cordl_object.invoke("CreateChainOperation", (dependentOp, callback))?;
        Ok(__cordl_ret)
    }
    pub fn CreateChainOperation_AsyncOperationHandle_1__cordl_bool2<
        TObject,
        TObjectDependency,
    >(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObjectDependency,
        >,
        callback: *mut crate::System::Func_2<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObjectDependency,
            >,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObject,
            >,
        >,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TObjectDependency: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object
            .invoke(
                "CreateChainOperation",
                (dependentOp, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateChainOperation_AsyncOperationHandle__cordl_bool3<TObject>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        callback: *mut crate::System::Func_2<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObject,
            >,
        >,
        releaseDependenciesOnFailure: bool,
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
        > = __cordl_object
            .invoke(
                "CreateChainOperation",
                (dependentOp, callback, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteDeferredCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDeferredCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveOperationFromCache(
        &mut self,
        key: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveOperationFromCache", (key))?;
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
    pub fn ClearDiagnosticCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDiagnosticCallbacks", ())?;
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
    pub fn ProvideResources_Action_1_0<TObject>(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        callback: *mut crate::System::Action_1<TObject>,
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
        > = __cordl_object.invoke("ProvideResources", (locations, callback))?;
        Ok(__cordl_ret)
    }
    pub fn ProvideResources__cordl_bool_Action_1_1<TObject>(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        releaseDependenciesOnFailure: bool,
        callback: *mut crate::System::Action_1<TObject>,
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
                "ProvideResources",
                (locations, releaseDependenciesOnFailure, callback),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CalculateLocationsHash(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        t: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateLocationsHash", (locations, t))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveUpdateReciever(
        &mut self,
        receiver: *mut crate::UnityEngine::ResourceManagement::IUpdateReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveUpdateReciever", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn CachedOperationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CachedOperationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartOperation_AsyncOperationBase_1_0<TObject>(
        &mut self,
        operation: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
            TObject,
        >,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
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
        > = __cordl_object.invoke("StartOperation", (operation, dependency))?;
        Ok(__cordl_ret)
    }
    pub fn StartOperation_IAsyncOperation1(
        &mut self,
        operation: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("StartOperation", (operation, dependency))?;
        Ok(__cordl_ret)
    }
    pub fn GetResourceProvider(
        &mut self,
        t: *mut crate::System::Type,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider = __cordl_object
            .invoke("GetResourceProvider", (t, location))?;
        Ok(__cordl_ret)
    }
    pub fn ClearDiagnosticsCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDiagnosticsCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProvideResource_Type__cordl_bool0(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        desiredType: *mut crate::System::Type,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke(
                "ProvideResource",
                (location, desiredType, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProvideResource_IResourceLocation1<TObject>(
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
        > = __cordl_object.invoke("ProvideResource", (location))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCompletedOperation<TObject>(
        &mut self,
        result: TObject,
        errorMsg: *mut crate::System::String,
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
        > = __cordl_object.invoke("CreateCompletedOperation", (result, errorMsg))?;
        Ok(__cordl_ret)
    }
    pub fn AddUpdateReceiver(
        &mut self,
        receiver: *mut crate::UnityEngine::ResourceManagement::IUpdateReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUpdateReceiver", (receiver))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGenericGroupOperation(
        &mut self,
        operations: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
        releasedCachedOpOnComplete: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        > = __cordl_object
            .invoke(
                "CreateGenericGroupOperation",
                (operations, releasedCachedOpOnComplete),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateCacheKeyForLocation(
        &mut self,
        provider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        desiredType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey = __cordl_object
            .invoke("CreateCacheKeyForLocation", (provider, location, desiredType))?;
        Ok(__cordl_ret)
    }
    pub fn get_ResourceProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        > = __cordl_object.invoke("get_ResourceProviders", ())?;
        Ok(__cordl_ret)
    }
    pub fn RegisterForDeferredCallback(
        &mut self,
        op: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        incrementRefCount: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForDeferredCallback", (op, incrementRefCount))?;
        Ok(__cordl_ret)
    }
    pub fn OnInstanceOperationDestroy(
        &mut self,
        o: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInstanceOperationDestroy", (o))?;
        Ok(__cordl_ret)
    }
    pub fn get_InstanceOperationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InstanceOperationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOperationFromCache(
        &mut self,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        desiredType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation = __cordl_object
            .invoke("GetOperationFromCache", (location, desiredType))?;
        Ok(__cordl_ret)
    }
    pub fn ProvideInstance(
        &mut self,
        provider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
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
            .invoke("ProvideInstance", (provider, location, instantiateParameters))?;
        Ok(__cordl_ret)
    }
    pub fn Acquire(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Acquire", (handle))?;
        Ok(__cordl_ret)
    }
    pub fn OnObjectRemoved(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnObjectRemoved", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Release(
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
    pub fn CleanupSceneInstances(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanupSceneInstances", (scene))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateHandlerInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Networking::CertificateHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Networking::CertificateHandler = __cordl_object
            .invoke("get_CertificateHandlerInstance", ())?;
        Ok(__cordl_ret)
    }
    pub fn AcquireGroupOpFromCache(
        &mut self,
        key: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::ResourceManagement::AsyncOperations::GroupOperation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::GroupOperation = __cordl_object
            .invoke("AcquireGroupOpFromCache", (key))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCompletedOperationInternal<TObject>(
        &mut self,
        result: TObject,
        success: bool,
        exception: *mut crate::System::Exception,
        releaseDependenciesOnFailure: bool,
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
        > = __cordl_object
            .invoke(
                "CreateCompletedOperationInternal",
                (result, success, exception, releaseDependenciesOnFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateCompletedOperationWithException<TObject>(
        &mut self,
        result: TObject,
        exception: *mut crate::System::Exception,
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
        > = __cordl_object
            .invoke("CreateCompletedOperationWithException", (result, exception))?;
        Ok(__cordl_ret)
    }
    pub fn set_CertificateHandlerInstance(
        &mut self,
        value: *mut crate::UnityEngine::Networking::CertificateHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateHandlerInstance", (value))?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__54_0(
        &mut self,
        x: *mut crate::UnityEngine::ResourceManagement::IUpdateReceiver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__54_0", (x))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterForCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnOperationDestroyCached(
        &mut self,
        o: *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOperationDestroyCached", (o))?;
        Ok(__cordl_ret)
    }
    pub fn CreateOperation<T>(
        &mut self,
        actualType: *mut crate::System::Type,
        typeHash: i32,
        cacheKey: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        onDestroyAction: *mut crate::System::Action_1<
            *mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke(
                "CreateOperation",
                (actualType, typeHash, cacheKey, onDestroyAction),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateGroupOperation_IList_1_0<T>(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        > = __cordl_object.invoke("CreateGroupOperation", (locations))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGroupOperation__cordl_bool1<T>(
        &mut self,
        locations: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        allowFailedDependencies: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        > = __cordl_object
            .invoke("CreateGroupOperation", (locations, allowFailedDependencies))?;
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
    pub fn New(
        alloc: *mut crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alloc))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
