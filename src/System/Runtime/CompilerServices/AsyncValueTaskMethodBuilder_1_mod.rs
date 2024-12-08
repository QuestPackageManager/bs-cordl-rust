#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AsyncValueTaskMethodBuilder_1<TResult: quest_hook::libil2cpp::Type> {
    pub _methodBuilder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        TResult,
    >,
    pub _result: TResult,
    pub _haveResult: bool,
    pub _useBuilder: bool,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder_1 < TResult > =>
    "System.Runtime.CompilerServices"."AsyncValueTaskMethodBuilder`1<TResult>" < TResult
    >
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder_1<TResult> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder_1<TResult> {
    pub fn AwaitUnsafeOnCompleted<TAwaiter, TStateMachine>(
        &mut self,
        awaiter: quest_hook::libil2cpp::ByRefMut<TAwaiter>,
        stateMachine: quest_hook::libil2cpp::ByRefMut<TStateMachine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TAwaiter: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TStateMachine: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AwaitUnsafeOnCompleted",
            (awaiter, stateMachine),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetException(
        &mut self,
        exception: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetException",
            (exception),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetResult(
        &mut self,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetResult",
            (result),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Start<TStateMachine>(
        &mut self,
        stateMachine: quest_hook::libil2cpp::ByRefMut<TStateMachine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStateMachine: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Start",
            (stateMachine),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Task(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ValueTask_1<TResult>,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask_1<TResult> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Task",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
