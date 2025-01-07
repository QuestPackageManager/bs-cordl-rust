#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TaskAwaiter {
    pub m_task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Runtime.CompilerServices";
    const CLASS_NAME: &'static str = "TaskAwaiter";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
impl AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TaskAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::TaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
