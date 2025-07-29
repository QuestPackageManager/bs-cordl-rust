#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncOperationBase_1<TObject: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Result_k__BackingField: TObject,
    pub m_referenceCount: i32,
    pub m_Status: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus,
    pub m_Error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    pub m_RM: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ResourceManagement::ResourceManager,
    >,
    pub m_Version: i32,
    pub m_DestroyedAction: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DelegateList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
    pub m_CompletedActionT: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DelegateList_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                TObject,
            >,
        >,
    >,
    pub m_OnDestroyAction: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
            >,
        >,
    >,
    pub m_dependencyCompleteAction: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        >,
    >,
    pub HasExecuted: bool,
    pub Executed: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _IsRunning_k__BackingField: bool,
    pub m_taskCompletionSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskCompletionSource_1<TObject>,
    >,
    pub m_taskCompletionSourceTypeless: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::TaskCompletionSource_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
    pub m_InDeferredCallbackQueue: bool,
    pub m_UpdateCallbacks: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DelegateList_1<f32>,
    >,
    pub m_UpdateCallback: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1"
)]
unsafe impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.AsyncOperations";
    const CLASS_NAME: &'static str = "AsyncOperationBase`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.ResourceManagement.AsyncOperations",
                        "AsyncOperationBase`1",
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
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1")]
impl<TObject: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    pub fn Complete_Exception__cordl_bool2(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TObject,
                            bool,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Complete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Complete", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (result, success, exception, releaseDependenciesOnFailure),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Il2CppString0(
        &mut self,
        result: TObject,
        success: bool,
        errorMsg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TObject,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Complete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Complete", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result, success, errorMsg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Complete_Il2CppString__cordl_bool1(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TObject,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Complete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Complete", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (result, success, errorMsg, releaseDependenciesOnFailure),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecrementReferenceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("DecrementReferenceCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecrementReferenceCount", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Destroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Destroy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDependencies(
        &mut self,
        dependencies: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetDependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDependencies", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (dependencies))?
        };
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
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
                        1usize,
                    >("GetDownloadStatus")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDownloadStatus", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = unsafe {
            cordl_method_info.invoke_unchecked(self, (visited))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IncrementReferenceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("IncrementReferenceCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IncrementReferenceCount", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeCompletionEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InvokeCompletionEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeCompletionEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeExecute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InvokeExecute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeExecute", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeWaitForCompletion(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("InvokeWaitForCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeWaitForCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn RegisterForDeferredCallbackEvent(
        &mut self,
        incrementReferenceCount: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterForDeferredCallbackEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterForDeferredCallbackEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (incrementReferenceCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseDependencies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ReleaseDependencies")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReleaseDependencies", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShortenPath(
        p: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        keepExtension: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ShortenPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ShortenPath", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (p, keepExtension))? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        updateCallbacks: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DelegateList_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::DelegateList_1<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Start",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rm, dependency, updateCallbacks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_DecrementReferenceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.DecrementReferenceCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.DecrementReferenceCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_GetDependencies(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDependencies",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDependencies",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (deps))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_GetDownloadStatus(
        &mut self,
        visited: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::HashSet_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDownloadStatus",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDownloadStatus",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::DownloadStatus = unsafe {
            cordl_method_info.invoke_unchecked(self, (visited))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_GetResultAsObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetResultAsObject",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetResultAsObject",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_IncrementReferenceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.IncrementReferenceCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.IncrementReferenceCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_InvokeCompletionEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.InvokeCompletionEvent",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.InvokeCompletionEvent",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_Start(
        &mut self,
        rm: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::ResourceManager,
        >,
        dependency: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
        updateCallbacks: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DelegateList_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ResourceManagement::ResourceManager,
                            >,
                            crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::DelegateList_1<f32>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.Start",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.Start",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rm, dependency, updateCallbacks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_add_CompletedTypeless(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_CompletedTypeless",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_CompletedTypeless",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_add_Destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_Destroyed",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_Destroyed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_DebugName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_DebugName",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_DebugName",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Handle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Handle",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_IsDone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        bool,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_IsDone",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_IsDone",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_OperationException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_OperationException",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_OperationException",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_PercentComplete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        f32,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_PercentComplete",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_PercentComplete",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_ReferenceCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        i32,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ReferenceCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ReferenceCount",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_ResultType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ResultType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ResultType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Status",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Status",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_Task(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        >,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Task",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Task",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        i32,
                        0usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Version",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Version",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_remove_CompletedTypeless(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_CompletedTypeless",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_CompletedTypeless",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_remove_Destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_Destroyed",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_Destroyed",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_ResourceManagement_AsyncOperations_IAsyncOperation_set_OnDestroy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.set_OnDestroy",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.set_OnDestroy",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCallback(
        &mut self,
        unscaledDeltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (unscaledDeltaTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitForCompletion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("WaitForCompletion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WaitForCompletion", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__38_0(
        &mut self,
        o: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("<.ctor>b__38_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<.ctor>b__38_0", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (o))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_Completed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                    TObject,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_Completed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_Completed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_CompletedTypeless(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_CompletedTypeless")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_CompletedTypeless", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_Destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_Destroyed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_Destroyed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_Executed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_Executed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_Executed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_CompletedEventHasListeners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_CompletedEventHasListeners")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CompletedEventHasListeners", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_DebugName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DebugName", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_DestroyedEventHasListeners(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_DestroyedEventHasListeners")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_DestroyedEventHasListeners", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Handle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        >,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                            TObject,
                        >,
                        0usize,
                    >("get_Handle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Handle", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            TObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDone(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsDone")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsDone", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsRunning(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsRunning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsRunning", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OperationException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        0usize,
                    >("get_OperationException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_OperationException", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PercentComplete(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_PercentComplete")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_PercentComplete", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Progress(&mut self) -> quest_hook::libil2cpp::Result<f32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_Progress")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Progress", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ReferenceCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_ReferenceCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ReferenceCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(&mut self) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TObject, 0usize>("get_Result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Result", 0usize
                        )
                    })
            });
        let __cordl_ret: TObject = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Status(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus,
                        0usize,
                    >("get_Status")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Status", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Task(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TObject>>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TObject>,
                        >,
                        0usize,
                    >("get_Task")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Task", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TObject>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Version")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Version", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_Completed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                    TObject,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
                                    TObject,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_Completed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_Completed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_CompletedTypeless(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_CompletedTypeless")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_CompletedTypeless", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_Destroyed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_Destroyed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_Destroyed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_Executed(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Action>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_Executed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_Executed", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsRunning(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_IsRunning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_IsRunning", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnDestroy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_OnDestroy", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_OperationException(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_OperationException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_OperationException", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Result(
        &mut self,
        value: TObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TObject),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Result", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1"
)]
impl<TObject: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation>
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationBase_1")]
impl<
    TObject: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation>
for crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationBase_1<
    TObject,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ResourceManagement::AsyncOperations::IAsyncOperation {
        unsafe { std::mem::transmute(self) }
    }
}
