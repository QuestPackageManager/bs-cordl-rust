#[cfg(feature = "System+Diagnostics+Tracing+EventLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventLevel {
    Critical = 1i32,
    Error = 2i32,
    Informational = 4i32,
    LogAlways = 0i32,
    Verbose = 5i32,
    Warning = 3i32,
}
#[cfg(feature = "System+Diagnostics+Tracing+EventLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::Tracing::EventLevel =>
    "System.Diagnostics.Tracing"."EventLevel"
);
