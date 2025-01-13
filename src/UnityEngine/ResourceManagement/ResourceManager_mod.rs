#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub postProfilerEvents: bool,
    pub _InternalIdTransformFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_2<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
            >,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _WebRequestOverride_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::UnityWebRequest>,
        >,
    >,
    pub CallbackHooksEnabled: bool,
    pub m_ResourceProviders: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ListWithEvents_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
            >,
        >,
    >,
    pub m_allocator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
    >,
    pub m_UpdateReceivers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ListWithEvents_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::IUpdateReceiver,
            >,
        >,
    >,
    pub m_UpdateReceiversToRemove: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::IUpdateReceiver,
            >,
        >,
    >,
    pub m_UpdatingReceivers: bool,
    pub m_InsideUpdateMethod: bool,
    pub m_providerMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
            >,
        >,
    >,
    pub m_AssetOperationCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
            >,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub m_TrackedInstanceOperations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation,
            >,
        >,
    >,
    pub m_UpdateCallbacks: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DelegateList_1<f32>,
    >,
    pub m_DeferredCompleteCallbacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub m_AssetBundleProviders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
            >,
        >,
    >,
    pub m_InsideExecuteDeferredCallbacksMethod: bool,
    pub m_DeferredCallbacksToRegister: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest,
        >,
    >,
    pub m_obsoleteDiagnosticsHandler: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub m_diagnosticsHandler: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
        >,
    >,
    pub m_ReleaseOpNonCached: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub m_ReleaseOpCached: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub m_ReleaseInstanceOp: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub _CertificateHandlerInstance_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::CertificateHandler,
    >,
    pub m_RegisteredForCallbacks: bool,
    pub m_ProviderOperationTypeCache: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl std::ops::Deref for crate::UnityEngine::ResourceManagement::ResourceManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
    )]
    pub type DeferredCallbackRegisterRequest = crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext"
    )]
    pub type DiagnosticEventContext = crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext;
    #[cfg(
        feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType"
    )]
    pub type DiagnosticEventType = crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType;
    #[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
    pub type InstanceOperation = crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation;
    pub fn Acquire(
        &mut self,
        handle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Acquire", (handle))?;
        Ok(__cordl_ret.into())
    }
    pub fn AcquireGroupOpFromCache(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::GroupOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::GroupOperation,
        > = __cordl_object.invoke("AcquireGroupOpFromCache", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddOperationToCache(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
        operation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOperationToCache", (key, operation))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddUpdateReceiver(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::IUpdateReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUpdateReceiver", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn CachedOperationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CachedOperationCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateLocationsHash(
        &mut self,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CalculateLocationsHash", (locations, t))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ClearDiagnosticCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDiagnosticCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearDiagnosticsCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDiagnosticsCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCacheKeyForLocation(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        > = __cordl_object
            .invoke("CreateCacheKeyForLocation", (provider, location, desiredType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateChainOperation_AsyncOperationHandle1<TObject>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateChainOperation_AsyncOperationHandle_1_0<TObject, TObjectDependency>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObjectDependency,
        >,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObjectDependency,
                >,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateChainOperation_AsyncOperationHandle_1__cordl_bool2<
        TObject,
        TObjectDependency,
    >(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObjectDependency,
        >,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObjectDependency,
                >,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateChainOperation_AsyncOperationHandle__cordl_bool3<TObject>(
        &mut self,
        dependentOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateCompletedOperation<TObject>(
        &mut self,
        result: TObject,
        errorMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateCompletedOperationInternal<TObject>(
        &mut self,
        result: TObject,
        success: bool,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateCompletedOperationWithException<TObject>(
        &mut self,
        result: TObject,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CreateGenericGroupOperation(
        &mut self,
        operations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
        releasedCachedOpOnComplete: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        > = __cordl_object
            .invoke(
                "CreateGenericGroupOperation",
                (operations, releasedCachedOpOnComplete),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGroupOperation_IList_1_0<T>(
        &mut self,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
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
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        > = __cordl_object.invoke("CreateGroupOperation", (locations))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGroupOperation__cordl_bool1<T>(
        &mut self,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        allowFailedDependencies: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
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
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        > = __cordl_object
            .invoke("CreateGroupOperation", (locations, allowFailedDependencies))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOperation<T>(
        &mut self,
        actualType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeHash: i32,
        cacheKey: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
        onDestroyAction: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
                >,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDeferredCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDeferredCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultTypeForLocation(
        &mut self,
        loc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetDefaultTypeForLocation", (loc))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOperationFromCache(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        > = __cordl_object.invoke("GetOperationFromCache", (location, desiredType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResourceProvider(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        > = __cordl_object.invoke("GetResourceProvider", (t, location))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOperationCached(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsOperationCached", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        alloc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alloc))?;
        Ok(__cordl_object.into())
    }
    pub fn OnInstanceOperationDestroy(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnInstanceOperationDestroy", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnObjectAdded(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnObjectAdded", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnObjectRemoved(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnObjectRemoved", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnOperationDestroyCached(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOperationDestroyCached", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnOperationDestroyNonCached(
        &mut self,
        o: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOperationDestroyNonCached", (o))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ProvideInstance(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        instantiateParameters: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        > = __cordl_object
            .invoke("ProvideInstance", (provider, location, instantiateParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideResourceGroupCached(
        &mut self,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        groupHash: i32,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
        releaseDependenciesOnFailure: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                >,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProvideResource_IResourceLocation1<TObject>(
        &mut self,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = __cordl_object.invoke("ProvideResource", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideResource_Type__cordl_bool0(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        desiredType: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProvideResources_Action_1_0<TObject>(
        &mut self,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = __cordl_object.invoke("ProvideResources", (locations, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideResources__cordl_bool_Action_1_1<TObject>(
        &mut self,
        locations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
            >,
        >,
        releaseDependenciesOnFailure: bool,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IList_1<TObject>,
            >,
        > = __cordl_object
            .invoke(
                "ProvideResources",
                (locations, releaseDependenciesOnFailure, callback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvideScene_LoadSceneMode0(
        &mut self,
        sceneProvider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn ProvideScene_LoadSceneParameters1(
        &mut self,
        sceneProvider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDiagnosticCallback_Action_1_1(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDiagnosticCallback", (func))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDiagnosticCallback_Action_4_0(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Action_4<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
                i32,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDiagnosticCallback", (func))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterForCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForCallbacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterForDeferredCallback(
        &mut self,
        op: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
        incrementRefCount: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterForDeferredCallback", (op, incrementRefCount))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseScene(
        &mut self,
        sceneProvider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::ISceneProvider,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn RemoveOperationFromCache(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemoveOperationFromCache", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveUpdateReciever(
        &mut self,
        receiver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::IUpdateReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveUpdateReciever", (receiver))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartOperation_AsyncOperationBase_1_0<TObject>(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
                TObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn StartOperation_IAsyncOperation1(
        &mut self,
        operation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
        >,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = __cordl_object
            .invoke("StartOperation", (operation, dependency))?;
        Ok(__cordl_ret.into())
    }
    pub fn TransformInternalId(
        &mut self,
        location: quest_hook::libil2cpp::Gc<
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
        > = __cordl_object.invoke("TransformInternalId", (location))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDiagnosticCallback(
        &mut self,
        func: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDiagnosticCallback", (func))?;
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
    pub fn __ctor_b__54_0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::IUpdateReceiver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__54_0", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        alloc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alloc))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Allocator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
        > = __cordl_object.invoke("get_Allocator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateHandlerInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Networking::CertificateHandler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::CertificateHandler,
        > = __cordl_object.invoke("get_CertificateHandlerInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExceptionHandler() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ExceptionHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstanceOperationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InstanceOperationCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalIdTransformFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("get_InternalIdTransformFunc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OperationCacheCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OperationCacheCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceProviders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
                >,
            >,
        > = __cordl_object.invoke("get_ResourceProviders", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WebRequestOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        > = __cordl_object.invoke("get_WebRequestOverride", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Allocator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IAllocationStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Allocator", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CertificateHandlerInstance(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::CertificateHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateHandlerInstance", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ExceptionHandler(
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                quest_hook::libil2cpp::Gc<crate::System::Exception>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ExceptionHandler", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InternalIdTransformFunc(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InternalIdTransformFunc", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_WebRequestOverride(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::UnityWebRequest,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WebRequestOverride", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::ResourceManagement::ResourceManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::ResourceManagement::ResourceManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_CompletedOperation_1<TObject: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        TObject,
    >,
    pub m_Success: bool,
    pub m_Exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    pub m_ReleaseDependenciesOnFailure: bool,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+CompletedOperation_1")]
unsafe impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager_CompletedOperation_1<
    TObject,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager/CompletedOperation`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.ResourceManagement",
                        "ResourceManager/CompletedOperation`1",
                    )
                    .unwrap()
                    .make_generic::<(TObject)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
        Ok(__cordl_ret.into())
    }
    pub fn Init_Exception1(
        &mut self,
        result: TObject,
        success: bool,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Init_Il2CppString0(
        &mut self,
        result: TObject,
        success: bool,
        errorMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DebugName", ())?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResourceManager_DeferredCallbackRegisterRequest {
    pub operation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
    >,
    pub incrementRefCount: bool,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager/DeferredCallbackRegisterRequest";
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
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
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
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
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
    feature = "UnityEngine+ResourceManagement+ResourceManager+DeferredCallbackRegisterRequest"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::ResourceManager_DeferredCallbackRegisterRequest {
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResourceManager_DiagnosticEventContext {
    pub _OperationHandle_k__BackingField: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    pub _Type_k__BackingField: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
    pub _EventValue_k__BackingField: i32,
    pub _Location_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    >,
    pub _Context_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _Error_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager/DiagnosticEventContext";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventContext")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventContext {
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
    pub fn _ctor(
        &mut self,
        op: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        _cordl_type: crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType,
        eventValue: i32,
        error: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (op, _cordl_type, eventValue, error, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Context", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Error", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EventValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_EventValue",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Location", ())?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResourceManager_DiagnosticEventType {
    #[default]
    AsyncOperationComplete = 3i32,
    AsyncOperationCreate = 1i32,
    AsyncOperationDestroy = 5i32,
    AsyncOperationFail = 0i32,
    AsyncOperationPercentComplete = 2i32,
    AsyncOperationReferenceCount = 4i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager/DiagnosticEventType";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType {
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType {
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+DiagnosticEventType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::ResourceManagement::ResourceManager_DiagnosticEventType {
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManager_InstanceOperation {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    >,
    pub m_dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    >,
    pub m_instantiationParams: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
    pub m_instanceProvider: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
    >,
    pub m_instance: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub m_scene: crate::UnityEngine::SceneManagement::Scene,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement";
    const CLASS_NAME: &'static str = "ResourceManager/InstanceOperation";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceManager+InstanceOperation")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceManager_InstanceOperation {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
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
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
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
    pub fn GetDependencies(
        &mut self,
        deps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (deps))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadStatus(
        &mut self,
        visited: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = __cordl_object
            .invoke("GetDownloadStatus", (visited))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        instanceProvider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IInstanceProvider,
        >,
        instantiationParams: crate::UnityEngine::ResourceManagement::ResourceProviders::InstantiationParameters,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (rm, instanceProvider, instantiationParams, dependency))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstanceScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = __cordl_object
            .invoke("InstanceScene", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InvokeWaitForCompletion", ())?;
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
