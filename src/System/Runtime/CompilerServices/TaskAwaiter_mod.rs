#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TaskAwaiter {
    pub m_task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::TaskAwaiter
    => "System.Runtime.CompilerServices"."TaskAwaiter"
);
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl crate::System::Runtime::CompilerServices::TaskAwaiter {
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResult",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleNonSuccessAndDebuggerNotification(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HandleNonSuccessAndDebuggerNotification", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnCompleted",
            (continuation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCompletedInternal(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
        continueOnCapturedContext: bool,
        flowExecutionContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "OnCompletedInternal",
                (task, continuation, continueOnCapturedContext, flowExecutionContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn OutputWaitEtwEvents(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Action>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Action> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OutputWaitEtwEvents", (task, continuation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowForNonSuccess(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowForNonSuccess", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeOnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnsafeOnCompleted",
            (continuation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateEnd(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateEnd", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (task),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
> for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        todo!()
    }
}
