#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationCallbackInfo_WithSyncContext {
    __cordl_parent: crate::System::Threading::CancellationCallbackInfo,
    pub TargetSyncContext: *mut crate::System::Threading::SynchronizationContext,
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CancellationCallbackInfo_WithSyncContext => "System.Threading"
    ."CancellationCallbackInfo/WithSyncContext"
);
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    type Target = crate::System::Threading::CancellationCallbackInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    pub fn New(
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
        stateForCallback: *mut crate::System::Object,
        targetExecutionContext: *mut crate::System::Threading::ExecutionContext,
        cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
        targetSyncContext: *mut crate::System::Threading::SynchronizationContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    callback,
                    stateForCallback,
                    targetExecutionContext,
                    cancellationTokenSource,
                    targetSyncContext,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
        stateForCallback: *mut crate::System::Object,
        targetExecutionContext: *mut crate::System::Threading::ExecutionContext,
        cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
        targetSyncContext: *mut crate::System::Threading::SynchronizationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    callback,
                    stateForCallback,
                    targetExecutionContext,
                    cancellationTokenSource,
                    targetSyncContext,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
