#[cfg(feature = "System+Diagnostics+Tracing+EventKeywords")]
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
#[cfg(feature = "System+Diagnostics+Tracing+EventKeywords")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Tracing::EventKeywords =>
    "System.Diagnostics.Tracing"."EventKeywords"
);
