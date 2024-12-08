#[cfg(feature = "Oculus+Platform+NetSyncConnectionStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetSyncConnectionStatus {
    Connected = 3i32,
    Connecting = 1i32,
    Disconnected = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+NetSyncConnectionStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::NetSyncConnectionStatus =>
    "Oculus.Platform"."NetSyncConnectionStatus"
);
