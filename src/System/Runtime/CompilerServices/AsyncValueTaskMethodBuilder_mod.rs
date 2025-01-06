#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AsyncValueTaskMethodBuilder {
    pub _methodBuilder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub _haveResult: bool,
    pub _useBuilder: bool,
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder =>
    "System.Runtime.CompilerServices"."AsyncValueTaskMethodBuilder"
);
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+AsyncValueTaskMethodBuilder")]
impl crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder {
    pub fn AwaitUnsafeOnCompleted<TAwaiter, TStateMachine>(
        &mut self,
        awaiter: quest_hook::libil2cpp::ByRefMut<TAwaiter>,
        stateMachine: quest_hook::libil2cpp::ByRefMut<TStateMachine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
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
        Ok(__cordl_ret.into())
    }
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::AsyncValueTaskMethodBuilder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetException(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetException",
            (exception),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetResult",
            (),
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
    pub fn Start<TStateMachine>(
        &mut self,
        stateMachine: quest_hook::libil2cpp::ByRefMut<TStateMachine>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TStateMachine: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Start",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Task(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Threading::Tasks::ValueTask> {
        let __cordl_ret: crate::System::Threading::Tasks::ValueTask = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Task",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
