#[cfg(feature = "cordl_class_System+Threading+CancellationCallbackInfo+WithSyncContext")]
#[repr(C)]
#[derive(Debug)]
pub struct CancellationCallbackInfo_WithSyncContext {
    __cordl_parent: crate::System::Threading::CancellationCallbackInfo,
    pub TargetSyncContext: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SynchronizationContext,
    >,
}
#[cfg(feature = "cordl_class_System+Threading+CancellationCallbackInfo+WithSyncContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "CancellationCallbackInfo/WithSyncContext";
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
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl std::ops::Deref
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    type Target = crate::System::Threading::CancellationCallbackInfo;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl std::ops::DerefMut
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    pub fn New(
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        stateForCallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetExecutionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        cancellationTokenSource: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationTokenSource,
        >,
        targetSyncContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
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
                    targetSyncContext,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        stateForCallback: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        targetExecutionContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::ExecutionContext,
        >,
        cancellationTokenSource: quest_hook::libil2cpp::Gc<
            crate::System::Threading::CancellationTokenSource,
        >,
        targetSyncContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_1<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppObject,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::ExecutionContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::CancellationTokenSource,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::SynchronizationContext,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        callback,
                        stateForCallback,
                        targetExecutionContext,
                        cancellationTokenSource,
                        targetSyncContext,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+CancellationCallbackInfo+WithSyncContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CancellationCallbackInfo_WithSyncContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
