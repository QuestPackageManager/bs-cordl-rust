#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForceAsyncAwaiter {
    pub _task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
}
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::ForceAsyncAwaiter =>
    "System.Threading.Tasks"."ForceAsyncAwaiter"
);
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Tasks::ForceAsyncAwaiter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
impl crate::System::Threading::Tasks::ForceAsyncAwaiter {
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::ForceAsyncAwaiter,
    > {
        let __cordl_ret: crate::System::Threading::Tasks::ForceAsyncAwaiter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAwaiter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
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
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OnCompleted",
            (action),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeOnCompleted(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnsafeOnCompleted",
            (action),
        )?;
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
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
impl AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Threading::Tasks::ForceAsyncAwaiter {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Threading::Tasks::ForceAsyncAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
impl AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Threading::Tasks::ForceAsyncAwaiter {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(feature = "System+Threading+Tasks+ForceAsyncAwaiter")]
impl AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Threading::Tasks::ForceAsyncAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
