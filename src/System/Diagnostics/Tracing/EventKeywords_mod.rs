#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventKeywords {
    #[default]
    All = -1i64,
    AuditFailure = 4503599627370496i64,
    AuditSuccess = 9007199254740992i64,
    EventLogClassic = 36028797018963968i64,
    MicrosoftTelemetry = 562949953421312i64,
    None = 0i64,
    Sqm = 2251799813685248i64,
    WdiDiagnostic = 1125899906842624i64,
}
#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::Tracing::EventKeywords {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Diagnostics.Tracing";
    const CLASS_NAME: &'static str = "EventKeywords";
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
#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Diagnostics::Tracing::EventKeywords {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Diagnostics::Tracing::EventKeywords {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Diagnostics::Tracing::EventKeywords {
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
#[cfg(feature = "cordl_class_System+Diagnostics+Tracing+EventKeywords")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Diagnostics::Tracing::EventKeywords {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
