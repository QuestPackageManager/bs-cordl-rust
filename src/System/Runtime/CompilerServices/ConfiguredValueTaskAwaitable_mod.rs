#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConfiguredValueTaskAwaitable {
    pub _value: crate::System::Threading::Tasks::ValueTask,
}
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable =>
    "System.Runtime.CompilerServices"."ConfiguredValueTaskAwaitable"
);
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable")]
impl crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable {
    #[cfg(
        feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
    )]
    pub type ConfiguredValueTaskAwaiter = crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter;
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAwaiter",
            (),
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
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    pub _value: crate::System::Threading::Tasks::ValueTask,
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter
    => "System.Runtime.CompilerServices"
    ."ConfiguredValueTaskAwaitable/ConfiguredValueTaskAwaiter"
);
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
impl crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
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
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
impl AsRef<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
impl AsMut<crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion>
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
impl AsRef<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredValueTaskAwaitable+ConfiguredValueTaskAwaiter"
)]
impl AsMut<crate::System::Runtime::CompilerServices::INotifyCompletion>
for crate::System::Runtime::CompilerServices::ConfiguredValueTaskAwaitable_ConfiguredValueTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::INotifyCompletion {
        todo!()
    }
}
