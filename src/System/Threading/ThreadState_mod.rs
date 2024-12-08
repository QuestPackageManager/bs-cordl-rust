#[cfg(feature = "System+Threading+ThreadState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    AbortRequested = 128i32,
    Aborted = 256i32,
    Background = 4i32,
    Running = 0i32,
    StopRequested = 1i32,
    Stopped = 16i32,
    SuspendRequested = 2i32,
    Suspended = 64i32,
    Unstarted = 8i32,
    WaitSleepJoin = 32i32,
}
#[cfg(feature = "System+Threading+ThreadState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadState =>
    "System.Threading"."ThreadState"
);
