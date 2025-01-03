#[cfg(feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct AsyncMethodBuilderCore {
    pub m_stateMachine: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    >,
    pub m_defaultContextAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
    pub fn CreateContinuationWrapper(
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
        invokeAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
        innerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateContinuationWrapper",
                (continuation, invokeAction, innerTask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCompletionAction(
        &mut self,
        taskForTracing: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        runnerToInitialize: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCompletionAction",
            (taskForTracing, runnerToInitialize),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OutputAsyncCausalityEvents(
        &mut self,
        innerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OutputAsyncCausalityEvents",
            (innerTask, continuation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PostBoxInitialization(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
        runner: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::AsyncMethodBuilderCore_MoveNextRunner,
        >,
        builtTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PostBoxInitialization",
            (stateMachine, runner, builtTask),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAsync(
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        targetContext: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SynchronizationContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAsync", (exception, targetContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetContinuationTask(
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetContinuationTask", (action))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+AsyncMethodBuilderCore+ContinuationWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncMethodBuilderCore_ContinuationWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_invokeAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_innerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
        invokeAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
        innerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (continuation, invokeAction, innerTask))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
        invokeAction: quest_hook::libil2cpp::Gc<crate::System::Action>,
        innerTask: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (continuation, invokeAction, innerTask))?;
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_context: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
    pub m_stateMachine: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn InvokeMoveNext(
        stateMachine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeMoveNext", (stateMachine))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context, stateMachine))?;
        Ok(__cordl_object.into())
    }
    pub fn Run(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Run", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::System::Threading::ExecutionContext>,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context, stateMachine))?;
        Ok(__cordl_ret.into())
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
