#[cfg(feature = "System+Threading+Tasks+Sources+ValueTaskSourceOnCompletedFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueTaskSourceOnCompletedFlags {
    FlowExecutionContext = 2i32,
    None = 0i32,
    UseSchedulingContext = 1i32,
}
#[cfg(feature = "System+Threading+Tasks+Sources+ValueTaskSourceOnCompletedFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::Sources::ValueTaskSourceOnCompletedFlags =>
    "System.Threading.Tasks.Sources"."ValueTaskSourceOnCompletedFlags"
);
