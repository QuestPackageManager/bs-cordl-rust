#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderOperation_1<TObject: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        TObject,
    >,
    pub m_ReleaseDependenciesOnFailure: bool,
    pub m_CompletionCallback: *mut crate::System::Action_4<
        i32,
        *mut quest_hook::libil2cpp::Il2CppObject,
        bool,
        *mut crate::System::Exception,
    >,
    pub m_GetDepCallback: *mut crate::System::Action_2<
        i32,
        *mut crate::System::Collections::Generic::IList_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
    pub m_GetProgressCallback: *mut crate::System::Func_1<f32>,
    pub m_GetDownloadProgressCallback: *mut crate::System::Func_1<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    >,
    pub m_WaitForCompletionCallback: *mut crate::System::Func_1<bool>,
    pub m_ProviderCompletedCalled: bool,
    pub m_DownloadStatus: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    pub m_Provider: *mut crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
    pub m_DepOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
        *mut crate::System::Collections::Generic::IList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
    pub m_Location: *mut crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
    pub m_ProvideHandleVersion: i32,
    pub m_NeedsRelease: bool,
    pub _UnityEngine_ResourceManagement_AsyncOperations_ICachable_Key_k__BackingField: *mut crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
    pub m_ResourceManager: *mut crate::UnityEngine::ResourceManagement::ResourceManager,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1 < TObject >
    => "UnityEngine.ResourceManagement.AsyncOperations"."ProviderOperation`1" < TObject >
);
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    type Target = crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
        TObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<TObject> {
    pub const kInvalidHandleMsg: &'static str = "The ProvideHandle is invalid. After the handle has been completed, it can no longer be used";
    pub const k_OperationWaitingToCompletePercentComplete: f32 = 0.99f32;
    pub fn Destroy(
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
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies_IList_1_1(
        &mut self,
        dstList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (dstList))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies_List_1_0(
        &mut self,
        deps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDependencies", (deps))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDependency<TDepObject>(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<TDepObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDepObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TDepObject = __cordl_object.invoke("GetDependency", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDownloadStatus(
        &mut self,
        visited: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = __cordl_object
            .invoke("GetDownloadStatus", (visited))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ResourceManager_IResourceProvider_IResourceLocation_AsyncOperationHandle_1_0(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        provider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (rm, provider, location, depOp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init__cordl_bool1(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        provider: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceProviders::IResourceProvider,
        >,
        location: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
        depOp: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            *mut crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
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
            .invoke(
                "Init",
                (rm, provider, location, depOp, releaseDependenciesOnFailure),
            )?;
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
    pub fn ProviderCompleted<T>(
        &mut self,
        result: T,
        status: bool,
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProviderCompleted", (result, status, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseDependencies(
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
            .invoke("ReleaseDependencies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDownloadProgressCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDownloadProgressCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProgressCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Func_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetProgressCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetWaitForCompletionCallback(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Func_1<bool>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWaitForCompletionCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_ICachable_get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        > = __cordl_object
            .invoke(
                "UnityEngine.ResourceManagement.AsyncOperations.ICachable.get_Key",
                (),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_ICachable_set_Key(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Util::IOperationCacheKey,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.ResourceManagement.AsyncOperations.ICachable.set_Key",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletionHandler(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("WaitForCompletionHandler", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_DependencyCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DependencyCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceLocations::IResourceLocation,
        > = __cordl_object.invoke("get_Location", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Progress", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ProvideHandleVersion(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ProvideHandleVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_RequestedType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::ResourceManagement::AsyncOperations::ICachable>
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::AsyncOperations::ICachable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::ResourceManagement::AsyncOperations::ICachable>
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::AsyncOperations::ICachable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsRef<
    crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
>
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+ProviderOperation_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsMut<
    crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation,
>
for crate::UnityEngine::ResourceManagement::AsyncOperations::ProviderOperation_1<
    TObject,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::AsyncOperations::IGenericProviderOperation {
        unsafe { std::mem::transmute(self) }
    }
}
