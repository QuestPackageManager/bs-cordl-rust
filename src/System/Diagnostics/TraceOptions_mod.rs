#[cfg(feature = "System+Diagnostics+TraceOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TraceOptions {
    #[default]
    Callstack = 32i32,
    DateTime = 2i32,
    LogicalOperationStack = 1i32,
    None = 0i32,
    ProcessId = 8i32,
    ThreadId = 16i32,
    Timestamp = 4i32,
}
#[cfg(feature = "System+Diagnostics+TraceOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::TraceOptions =>
    "System.Diagnostics"."TraceOptions"
);
