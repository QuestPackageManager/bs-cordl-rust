#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueTaskAwaiter {
    pub _value: crate::System::Threading::Tasks::ValueTask,
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ValueTaskAwaiter =>
    "System.Runtime.CompilerServices"."ValueTaskAwaiter"
);
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
impl crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
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
    pub fn _ctor(
        &mut self,
        value: crate::System::Threading::Tasks::ValueTask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
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
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
impl AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
impl AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ValueTaskAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ValueTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
