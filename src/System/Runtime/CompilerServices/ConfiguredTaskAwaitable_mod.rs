#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConfiguredTaskAwaitable {
    pub m_configuredTaskAwaiter: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ConfiguredTaskAwaitable =>
    "System.Runtime.CompilerServices"."ConfiguredTaskAwaitable"
);
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable")]
impl crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable {
    #[cfg(
        feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
    )]
    pub type ConfiguredTaskAwaiter = crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter;
    pub fn GetAwaiter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAwaiter",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (task, continueOnCapturedContext),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    pub m_task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    pub m_continueOnCapturedContext: bool,
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter =>
    "System.Runtime.CompilerServices"."ConfiguredTaskAwaitable/ConfiguredTaskAwaiter"
);
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
impl crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
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
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
        continueOnCapturedContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (task, continueOnCapturedContext),
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
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    >,
>
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    >,
>
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::ICriticalNotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
>
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        todo!()
    }
}
#[cfg(
    feature = "System+Runtime+CompilerServices+ConfiguredTaskAwaitable+ConfiguredTaskAwaiter"
)]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    >,
>
for crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::CompilerServices::INotifyCompletion,
    > {
        todo!()
    }
}
