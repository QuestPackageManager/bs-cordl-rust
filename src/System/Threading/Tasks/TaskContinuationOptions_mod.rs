#[cfg(feature = "System+Threading+Tasks+TaskContinuationOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskContinuationOptions {
    AttachedToParent = 4i32,
    DenyChildAttach = 8i32,
    ExecuteSynchronously = 524288i32,
    HideScheduler = 16i32,
    LazyCancellation = 32i32,
    LongRunning = 2i32,
    None = 0i32,
    NotOnCanceled = 262144i32,
    NotOnFaulted = 131072i32,
    NotOnRanToCompletion = 65536i32,
    OnlyOnCanceled = 196608i32,
    OnlyOnFaulted = 327680i32,
    OnlyOnRanToCompletion = 393216i32,
    PreferFairness = 1i32,
    RunContinuationsAsynchronously = 64i32,
}
#[cfg(feature = "System+Threading+Tasks+TaskContinuationOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::TaskContinuationOptions => "System.Threading.Tasks"
    ."TaskContinuationOptions"
);
