#[cfg(feature = "System+Threading+ThreadPriority")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    AboveNormal = 3i32,
    BelowNormal = 1i32,
    Highest = 4i32,
    Lowest = 0i32,
    Normal = 2i32,
}
#[cfg(feature = "System+Threading+ThreadPriority")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ThreadPriority =>
    "System.Threading"."ThreadPriority"
);
