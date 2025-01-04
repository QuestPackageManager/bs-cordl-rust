#[cfg(feature = "System+Threading+Tasks+InternalTaskOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InternalTaskOptions {
    #[default]
    ContinuationTask = 512i32,
    DoNotDispose = 16384i32,
    InternalOptionsMask = 65280i32,
    LazyCancellation = 4096i32,
    None = 0i32,
    PromiseTask = 1024i32,
    QueuedByRuntime = 8192i32,
}
#[cfg(feature = "System+Threading+Tasks+InternalTaskOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::InternalTaskOptions =>
    "System.Threading.Tasks"."InternalTaskOptions"
);
