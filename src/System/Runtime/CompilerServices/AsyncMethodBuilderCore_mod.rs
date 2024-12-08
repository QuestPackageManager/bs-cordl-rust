#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AsyncMethodBuilderCore {
    pub m_stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    pub m_defaultContextAction: *mut crate::System::Action,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncMethodBuilderCore =>
    "System.Runtime.CompilerServices"."AsyncMethodBuilderCore"
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore")]
impl crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore {
    #[cfg(
        feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
    )]
    pub type ContinuationWrapper = crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper;
    #[cfg(
        feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner"
    )]
    pub type MoveNextRunner = crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner;
    #[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+__c")]
    pub type __c = crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore___c;
    #[cfg(
        feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore___c__DisplayClass5_0;
    pub fn GetCompletionAction(
        &mut self,
        taskForTracing: *mut crate::System::Threading::Tasks::Task,
        runnerToInitialize: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Action> {
        let __cordl_ret: *mut crate::System::Action = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCompletionAction",
            (taskForTracing, runnerToInitialize),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OutputAsyncCausalityEvents(
        &mut self,
        innerTask: *mut crate::System::Threading::Tasks::Task,
        continuation: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Action> {
        let __cordl_ret: *mut crate::System::Action = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OutputAsyncCausalityEvents",
            (innerTask, continuation),
        )?;
        Ok(__cordl_ret)
    }
    pub fn PostBoxInitialization(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        runner: *mut crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner,
        builtTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PostBoxInitialization",
            (stateMachine, runner, builtTask),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncMethodBuilderCore_ContinuationWrapper {
    __cordl_parent: crate::System::Object,
    pub m_continuation: *mut crate::System::Action,
    pub m_invokeAction: *mut crate::System::Action,
    pub m_innerTask: *mut crate::System::Threading::Tasks::Task,
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper =>
    "System.Runtime.CompilerServices"."AsyncMethodBuilderCore/ContinuationWrapper"
);
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
impl crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        continuation: *mut crate::System::Action,
        invokeAction: *mut crate::System::Action,
        innerTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (continuation, invokeAction, innerTask))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        continuation: *mut crate::System::Action,
        invokeAction: *mut crate::System::Action,
        innerTask: *mut crate::System::Threading::Tasks::Task,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (continuation, invokeAction, innerTask))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_ContinuationWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncMethodBuilderCore_MoveNextRunner {
    __cordl_parent: crate::System::Object,
    pub m_context: *mut crate::System::Threading::ExecutionContext,
    pub m_stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner =>
    "System.Runtime.CompilerServices"."AsyncMethodBuilderCore/MoveNextRunner"
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
impl crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner {
    pub fn New(
        context: *mut crate::System::Threading::ExecutionContext,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, stateMachine))?;
        Ok(__cordl_object)
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        context: *mut crate::System::Threading::ExecutionContext,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, stateMachine))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+MoveNextRunner")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
