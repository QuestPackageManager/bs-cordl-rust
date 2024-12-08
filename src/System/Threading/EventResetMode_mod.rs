#[cfg(feature = "System+Threading+EventResetMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResetMode {
    AutoReset = 0i32,
    ManualReset = 1i32,
}
#[cfg(feature = "System+Threading+EventResetMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::EventResetMode =>
    "System.Threading"."EventResetMode"
);
