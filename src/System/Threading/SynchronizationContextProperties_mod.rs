#[cfg(feature = "System+Threading+SynchronizationContextProperties")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SynchronizationContextProperties {
    #[default]
    None = 0i32,
    RequireWaitNotification = 1i32,
}
#[cfg(feature = "System+Threading+SynchronizationContextProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::SynchronizationContextProperties => "System.Threading"
    ."SynchronizationContextProperties"
);
