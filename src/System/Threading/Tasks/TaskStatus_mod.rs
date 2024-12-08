#[cfg(feature = "System+Threading+Tasks+TaskStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Canceled = 6i32,
    Created = 0i32,
    Faulted = 7i32,
    RanToCompletion = 5i32,
    Running = 3i32,
    WaitingForActivation = 1i32,
    WaitingForChildrenToComplete = 4i32,
    WaitingToRun = 2i32,
}
#[cfg(feature = "System+Threading+Tasks+TaskStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::TaskStatus =>
    "System.Threading.Tasks"."TaskStatus"
);
