#[cfg(feature = "System+Threading+Tasks+AsyncCausalityStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncCausalityStatus {
    Canceled = 2i32,
    Completed = 1i32,
    Error = 3i32,
    Started = 0i32,
}
#[cfg(feature = "System+Threading+Tasks+AsyncCausalityStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::AsyncCausalityStatus
    => "System.Threading.Tasks"."AsyncCausalityStatus"
);
