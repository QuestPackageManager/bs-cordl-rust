#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationCallbackInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Callback: *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
    pub StateForCallback: *mut quest_hook::libil2cpp::Il2CppObject,
    pub TargetExecutionContext: *mut crate::System::Threading::ExecutionContext,
    pub CancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::CancellationCallbackInfo =>
    "System.Threading"."CancellationCallbackInfo"
);
#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
impl std::ops::Deref for crate::System::Threading::CancellationCallbackInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
impl std::ops::DerefMut for crate::System::Threading::CancellationCallbackInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
impl crate::System::Threading::CancellationCallbackInfo {
    #[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
    pub type WithSyncContext = crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext;
    pub fn ExecuteCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        stateForCallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetExecutionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        cancellationTokenSource: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationTokenSource,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        stateForCallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetExecutionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        cancellationTokenSource: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationTokenSource,
        >,
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
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::CancellationCallbackInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
