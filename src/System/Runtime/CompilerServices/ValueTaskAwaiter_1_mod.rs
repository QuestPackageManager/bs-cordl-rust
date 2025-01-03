#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ValueTaskAwaiter_1<TResult: quest_hook::libil2cpp::Type> {
    pub _value: crate::System::Threading::Tasks::ValueTask_1<TResult>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ValueTaskAwaiter_1 < TResult > =>
    "System.Runtime.CompilerServices"."ValueTaskAwaiter`1<TResult>" < TResult >
);
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
unsafe impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    pub fn GetResult(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TResult = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResult",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnCompleted",
            (continuation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeOnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnsafeOnCompleted",
            (continuation),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        value: crate::System::Threading::Tasks::ValueTask_1<TResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCompleted(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsCompleted",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter_1<TResult> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
