#[cfg(feature = "System+Threading+Tasks+TaskCreationOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskCreationOptions {
    #[default]
    AttachedToParent = 4i32,
    DenyChildAttach = 8i32,
    HideScheduler = 16i32,
    LongRunning = 2i32,
    None = 0i32,
    PreferFairness = 1i32,
    RunContinuationsAsynchronously = 64i32,
}
#[cfg(feature = "System+Threading+Tasks+TaskCreationOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskCreationOptions =>
    "System.Threading.Tasks"."TaskCreationOptions"
);
