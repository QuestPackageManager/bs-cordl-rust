#[cfg(feature = "System+Diagnostics+TraceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceLevel {
    Error = 1i32,
    Info = 3i32,
    Off = 0i32,
    Verbose = 4i32,
    Warning = 2i32,
}
#[cfg(feature = "System+Diagnostics+TraceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceLevel =>
    "System.Diagnostics"."TraceLevel"
);
