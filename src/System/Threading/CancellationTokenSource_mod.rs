#[cfg(feature = "System+Threading+CancellationTokenSource")]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationTokenSource {
    __cordl_parent: crate::System::Object,
    pub _kernelEvent: *mut crate::System::Threading::ManualResetEvent,
    pub _registeredCallbacksLists: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Threading::SparselyPopulatedArray_1<
            *mut crate::System::Threading::CancellationCallbackInfo,
        >,
    >,
    pub _state: i32,
    pub _threadIDExecutingCallbacks: i32,
    pub _disposed: bool,
    pub _executingCallback: *mut crate::System::Threading::CancellationCallbackInfo,
    pub _timer: *mut crate::System::Threading::Timer,
}
#[cfg(feature = "System+Threading+CancellationTokenSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::CancellationTokenSource =>
    "System.Threading"."CancellationTokenSource"
);
#[cfg(feature = "System+Threading+CancellationTokenSource")]
impl std::ops::Deref for crate::System::Threading::CancellationTokenSource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationTokenSource")]
impl std::ops::DerefMut for crate::System::Threading::CancellationTokenSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationTokenSource")]
impl crate::System::Threading::CancellationTokenSource {
    pub const CannotBeCanceled: i32 = 0i32;
    pub const NotCanceledState: i32 = 1i32;
    pub const NotifyingCompleteState: i32 = 3i32;
    pub const NotifyingState: i32 = 2i32;
    #[cfg(
        feature = "System+Threading+CancellationTokenSource+Linked1CancellationTokenSource"
    )]
    pub type Linked1CancellationTokenSource = crate::GlobalNamespace::CancellationTokenSource_Linked1CancellationTokenSource;
    #[cfg(
        feature = "System+Threading+CancellationTokenSource+Linked2CancellationTokenSource"
    )]
    pub type Linked2CancellationTokenSource = crate::GlobalNamespace::CancellationTokenSource_Linked2CancellationTokenSource;
    #[cfg(
        feature = "System+Threading+CancellationTokenSource+LinkedNCancellationTokenSource"
    )]
    pub type LinkedNCancellationTokenSource = crate::GlobalNamespace::CancellationTokenSource_LinkedNCancellationTokenSource;
    pub fn CancelAfter_TimeSpan0(
        &mut self,
        delay: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAfter", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn CancelAfter_i32_1(
        &mut self,
        millisecondsDelay: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAfter", (millisecondsDelay))?;
        Ok(__cordl_ret)
    }
    pub fn Cancel_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn Cancel__cordl_bool1(
        &mut self,
        throwOnFirstException: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Cancel", (throwOnFirstException))?;
        Ok(__cordl_ret)
    }
    pub fn CancellationCallbackCoreWork(
        &mut self,
        args: crate::System::Threading::CancellationCallbackCoreWorkArguments,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancellationCallbackCoreWork", (args))?;
        Ok(__cordl_ret)
    }
    pub fn CancellationCallbackCoreWork_OnSyncContext(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancellationCallbackCoreWork_OnSyncContext", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteCallbackHandlers(
        &mut self,
        throwOnFirstException: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteCallbackHandlers", (throwOnFirstException))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeWithTimer(
        &mut self,
        millisecondsDelay: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeWithTimer", (millisecondsDelay))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRegister(
        &mut self,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
        stateForCallback: *mut crate::System::Object,
        targetSyncContext: *mut crate::System::Threading::SynchronizationContext,
        executionContext: *mut crate::System::Threading::ExecutionContext,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::CancellationTokenRegistration,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationTokenRegistration = __cordl_object
            .invoke(
                "InternalRegister",
                (callback, stateForCallback, targetSyncContext, executionContext),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_TimeSpan1(
        delay: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (delay))?;
        Ok(__cordl_object)
    }
    pub fn NotifyCancellation(
        &mut self,
        throwOnFirstException: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NotifyCancellation", (throwOnFirstException))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowIfDisposed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowIfDisposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn WaitForCallbackToComplete(
        &mut self,
        callbackInfo: *mut crate::System::Threading::CancellationCallbackInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WaitForCallbackToComplete", (callbackInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TimeSpan1(
        &mut self,
        delay: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn get_ExecutingCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::CancellationCallbackInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::CancellationCallbackInfo = __cordl_object
            .invoke("get_ExecutingCallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCancellationCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationCompleted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCancellationRequested(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsCancellationRequested", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDisposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDisposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThreadIDExecutingCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_ThreadIDExecutingCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Token(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::CancellationToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::CancellationToken = __cordl_object
            .invoke("get_Token", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ThreadIDExecutingCallbacks(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ThreadIDExecutingCallbacks", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+CancellationTokenSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::CancellationTokenSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
