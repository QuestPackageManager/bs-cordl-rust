#[cfg(feature = "System+Threading+Tasks+Sources+ValueTaskSourceStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueTaskSourceStatus {
    Canceled = 3i32,
    Faulted = 2i32,
    Pending = 0i32,
    Succeeded = 1i32,
}
#[cfg(feature = "System+Threading+Tasks+Sources+ValueTaskSourceStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::Sources::ValueTaskSourceStatus =>
    "System.Threading.Tasks.Sources"."ValueTaskSourceStatus"
);
