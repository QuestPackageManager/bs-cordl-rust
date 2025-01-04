#[cfg(feature = "System+Diagnostics+TraceEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TraceEventType {
    #[default]
    Critical = 1i32,
    Error = 2i32,
    Information = 8i32,
    Resume = 2048i32,
    Start = 256i32,
    Stop = 512i32,
    Suspend = 1024i32,
    Transfer = 4096i32,
    Verbose = 16i32,
    Warning = 4i32,
}
#[cfg(feature = "System+Diagnostics+TraceEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceEventType =>
    "System.Diagnostics"."TraceEventType"
);
