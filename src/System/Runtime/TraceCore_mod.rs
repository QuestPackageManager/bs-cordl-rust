#[cfg(feature = "cordl_class_System+Runtime+TraceCore")]
#[repr(C)]
#[derive(Debug)]
pub struct TraceCore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_System+Runtime+TraceCore")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Runtime::TraceCore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime";
    const CLASS_NAME: &'static str = "TraceCore";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+TraceCore")]
impl std::ops::Deref for crate::System::Runtime::TraceCore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+TraceCore")]
impl std::ops::DerefMut for crate::System::Runtime::TraceCore {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+TraceCore")]
impl crate::System::Runtime::TraceCore {
    pub fn ActionItemCallbackInvokedIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("ActionItemCallbackInvokedIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActionItemCallbackInvokedIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ActionItemScheduledIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("ActionItemScheduledIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ActionItemScheduledIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppDomainUnload(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        appdomainName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        processId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("AppDomainUnload")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppDomainUnload", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (trace, appdomainName, processName, processId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AppDomainUnloadIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("AppDomainUnloadIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppDomainUnloadIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateEventDescriptors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CreateEventDescriptors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateEventDescriptors", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureEventDescriptors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("EnsureEventDescriptors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureEventDescriptors", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledException(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandledException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledException", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionError(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandledExceptionError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionError", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionErrorIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("HandledExceptionErrorIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionErrorIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("HandledExceptionIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionVerbose(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandledExceptionVerbose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionVerbose", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionVerboseIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("HandledExceptionVerboseIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionVerboseIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionWarning(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("HandledExceptionWarning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionWarning", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandledExceptionWarningIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("HandledExceptionWarningIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandledExceptionWarningIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEtwEventEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        eventIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            i32,
                        ),
                        bool,
                        2usize,
                    >("IsEtwEventEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEtwEventEnabled", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, eventIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowingException(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        param1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("ThrowingException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowingException", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, param1, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowingExceptionIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("ThrowingExceptionIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ThrowingExceptionIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogCritical(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        traceRecord: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TraceCodeEventLogCritical")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogCritical", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, traceRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogCriticalIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("TraceCodeEventLogCriticalIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogCriticalIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogError(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        traceRecord: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TraceCodeEventLogError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogError", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, traceRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogErrorIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("TraceCodeEventLogErrorIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogErrorIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogInfo(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        traceRecord: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TraceCodeEventLogInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogInfo", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, traceRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogInfoIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("TraceCodeEventLogInfoIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogInfoIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogVerbose(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        traceRecord: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TraceCodeEventLogVerbose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogVerbose", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, traceRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogVerboseIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("TraceCodeEventLogVerboseIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogVerboseIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogWarning(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        traceRecord: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::TraceRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::TraceRecord,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("TraceCodeEventLogWarning")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogWarning", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, traceRecord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TraceCodeEventLogWarningIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("TraceCodeEventLogWarningIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TraceCodeEventLogWarningIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnhandledException(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        param0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("UnhandledException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnhandledException", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (trace, param0, exception))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnhandledExceptionIsEnabled(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                        >),
                        bool,
                        1usize,
                    >("UnhandledExceptionIsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnhandledExceptionIsEnabled", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (trace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEtwEvent_EtwDiagnosticTrace_i32_EventTraceActivity_Il2CppString_Il2CppString2(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        eventIndex: i32,
        eventParam0: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EventTraceActivity,
        >,
        eventParam1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EventTraceActivity,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        5usize,
                    >("WriteEtwEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEtwEvent", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (trace, eventIndex, eventParam0, eventParam1, eventParam2),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEtwEvent_Il2CppString1(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        eventIndex: i32,
        eventParam0: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EventTraceActivity,
        >,
        eventParam1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EventTraceActivity,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        6usize,
                    >("WriteEtwEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEtwEvent", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        trace,
                        eventIndex,
                        eventParam0,
                        eventParam1,
                        eventParam2,
                        eventParam3,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteEtwEvent_Il2CppString_Il2CppString0(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        eventIndex: i32,
        eventParam0: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EventTraceActivity,
        >,
        eventParam1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eventParam4: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EventTraceActivity,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        bool,
                        7usize,
                    >("WriteEtwEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteEtwEvent", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        trace,
                        eventIndex,
                        eventParam0,
                        eventParam1,
                        eventParam2,
                        eventParam3,
                        eventParam4,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteTraceSource(
        trace: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
        >,
        eventIndex: i32,
        description: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        payload: crate::System::Runtime::TracePayload,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Runtime::Diagnostics::EtwDiagnosticTrace,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Runtime::TracePayload,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteTraceSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteTraceSource", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (trace, eventIndex, description, payload))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Culture() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::CultureInfo,
                        >,
                        0usize,
                    >("get_Culture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Culture", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ResourceManager() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Resources::ResourceManager>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Resources::ResourceManager,
                        >,
                        0usize,
                    >("get_ResourceManager")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ResourceManager", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Resources::ResourceManager,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Runtime+TraceCore")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Runtime::TraceCore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
